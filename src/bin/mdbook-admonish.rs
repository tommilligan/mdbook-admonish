use mdbook::{
    errors::Error,
    preprocess::{CmdPreprocessor, Preprocessor},
};
use mdbook_admonish::Admonish;
#[cfg(feature = "cli-install")]
use std::path::PathBuf;
use std::{io, process};

use clap::{Parser, Subcommand};

/// mdbook preprocessor to add support for admonitions
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Check whether a renderer is supported by this preprocessor
    Supports { renderer: String },

    #[cfg(feature = "cli-install")]
    /// Install the required assset files and include it in the config
    Install {
        /// Root directory for the book, should contain the configuration file (`book.toml`)
        dir: Option<PathBuf>,

        /// Relative directory for the css assets, from the book directory root
        #[arg(long)]
        css_dir: Option<PathBuf>,
    },
}

fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let cli = Cli::parse();

    match cli.command {
        None => {
            if let Err(e) = handle_preprocessing() {
                eprintln!("{}", e);
                process::exit(1);
            }
        }
        Some(Commands::Supports { renderer }) => {
            handle_supports(renderer);
        }
        #[cfg(feature = "cli-install")]
        Some(Commands::Install { dir, css_dir }) => {
            install::handle_install(
                dir.unwrap_or_else(|| PathBuf::from(".")),
                css_dir.unwrap_or_else(|| PathBuf::from(".")),
            );
        }
    }
}

fn handle_preprocessing() -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    if ctx.mdbook_version != mdbook::MDBOOK_VERSION {
        eprintln!(
            "Warning: The mdbook-admonish preprocessor was built against version \
             {} of mdbook, but we're being called from version {}",
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = Admonish.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

fn handle_supports(renderer: String) -> ! {
    let supported = Admonish.supports_renderer(&renderer);

    // Signal whether the renderer is supported by exiting with 1 or 0.
    if supported {
        process::exit(0);
    } else {
        process::exit(1);
    }
}

#[cfg(feature = "cli-install")]
mod install {
    use std::{
        fs::{self, File},
        io::Write,
        path::PathBuf,
        process,
    };
    use toml_edit::{self, Array, Document, Item, Table, Value};

    const ADMONISH_CSS_FILES: &[(&str, &[u8])] = &[(
        "mdbook-admonish.css",
        include_bytes!("assets/mdbook-admonish.css"),
    )];

    trait ArrayExt {
        fn contains_str(&self, value: &str) -> bool;
    }

    impl ArrayExt for Array {
        fn contains_str(&self, value: &str) -> bool {
            self.iter().any(|element| match element.as_str() {
                None => false,
                Some(element_str) => element_str == value,
            })
        }
    }

    pub fn handle_install(proj_dir: PathBuf, css_dir: PathBuf) {
        let config = proj_dir.join("book.toml");

        if !config.exists() {
            log::error!("Configuration file '{}' missing", config.display());
            process::exit(1);
        }

        log::info!("Reading configuration file '{}'", config.display());
        let toml = fs::read_to_string(&config).expect("can't read configuration file");
        let mut doc = toml
            .parse::<Document>()
            .expect("configuration is not valid TOML");

        if let Ok(preprocessor) = preprocessor(&mut doc) {
            const ASSETS_VERSION: &str = std::include_str!("./assets/VERSION");
            let value = toml_edit::value(
                toml_edit::Value::from(ASSETS_VERSION.trim())
                    .decorated(" ", " # do not edit: managed by `mdbook-admonish install`"),
            );
            preprocessor["assets_version"] = value;
        } else {
            log::info!("Unexpected configuration, not updating prereprocessor configuration");
        };

        let mut additional_css = additional_css(&mut doc);
        for (name, content) in ADMONISH_CSS_FILES {
            let filepath = proj_dir.join(&css_dir).join(name);
            let filepath_str = filepath.to_str().expect("non-utf8 filepath");

            if let Ok(ref mut additional_css) = additional_css {
                if !additional_css.contains_str(filepath_str) {
                    log::info!("Adding '{filepath_str}' to 'additional-css'");
                    additional_css.push(filepath_str);
                }
            } else {
                log::warn!("Unexpected configuration, not updating 'additional-css'");
            }

            log::info!(
                "Copying '{name}' to '{filepath}'",
                filepath = filepath.display()
            );
            let mut file = File::create(filepath).expect("can't open file for writing");
            file.write_all(content)
                .expect("can't write content to file");
        }

        let new_toml = doc.to_string();
        if new_toml != toml {
            log::info!("Saving changed configuration to '{}'", config.display());
            let mut file =
                File::create(config).expect("can't open configuration file for writing.");
            file.write_all(new_toml.as_bytes())
                .expect("can't write configuration");
        } else {
            log::info!("Configuration '{}' already up to date", config.display());
        }

        log::info!("mdbook-admonish is now installed. You can start using it in your book.");
        let codeblock = r#"```admonish warning
A beautifully styled message.
```"#;
        log::info!("Add a code block like:\n{}", codeblock);

        process::exit(0);
    }

    /// Return the `additional-css` field, initializing if required.
    ///
    /// Return `Err` if the existing configuration is unknown.
    fn additional_css(doc: &mut Document) -> Result<&mut Array, ()> {
        let doc = doc.as_table_mut();

        let empty_table = Item::Table(Table::default());
        let empty_array = Item::Value(Value::Array(Array::default()));

        doc.entry("output")
            .or_insert(empty_table.clone())
            .as_table_mut()
            .and_then(|item| {
                item.entry("html")
                    .or_insert(empty_table)
                    .as_table_mut()?
                    .entry("additional-css")
                    .or_insert(empty_array)
                    .as_value_mut()?
                    .as_array_mut()
            })
            .ok_or(())
    }

    /// Return the preprocessor table for admonish, initializing if required.
    ///
    /// Return `Err` if the existing configuration is unknown.
    fn preprocessor(doc: &mut Document) -> Result<&mut Item, ()> {
        let doc = doc.as_table_mut();

        let empty_table = Item::Table(Table::default());
        let item = doc.entry("preprocessor").or_insert(empty_table.clone());
        let item = item
            .as_table_mut()
            .ok_or(())?
            .entry("admonish")
            .or_insert(empty_table);
        item["command"] = toml_edit::value("mdbook-admonish");
        Ok(item)
    }
}
