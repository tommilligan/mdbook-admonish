use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use mdbook::preprocess::{CmdPreprocessor, Preprocessor};
use mdbook_admonish::Admonish;
use serde::Deserialize;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process;

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
        ///
        /// If not set, defaults to the current directory.
        dir: Option<PathBuf>,

        /// Relative directory for the css assets, from the book directory root
        ///
        /// If not set, defaults to the current directory.
        #[arg(long)]
        css_dir: Option<PathBuf>,
    },

    /// Generate CSS file for custom directives.
    GenerateCustom {
        /// Root directory for the book, should contain the configuration file (`book.toml`)
        ///
        /// If not set, defaults to the current directory.
        #[arg(long)]
        dir: Option<PathBuf>,

        /// File to write generated css to.
        output: PathBuf,
    },
}

fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let cli = Cli::parse();
    if let Err(error) = run(cli) {
        log::error!("Fatal error: {}", error);
        for error in error.chain() {
            log::error!("  - {}", error);
        }
        process::exit(1);
    }
}

fn run(cli: Cli) -> Result<()> {
    match cli.command {
        None => handle_preprocessing(),
        Some(Commands::Supports { renderer }) => {
            handle_supports(renderer);
        }
        #[cfg(feature = "cli-install")]
        Some(Commands::Install { dir, css_dir }) => install::handle_install(
            dir.unwrap_or_else(|| PathBuf::from(".")),
            css_dir.unwrap_or_else(|| PathBuf::from(".")),
        ),
        Some(Commands::GenerateCustom { dir, output }) => {
            handle_generate_custom(dir.unwrap_or_else(|| PathBuf::from(".")), output)
        }
    }
}

fn handle_preprocessing() -> std::result::Result<(), mdbook::errors::Error> {
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

#[derive(Deserialize)]
struct Config {
    #[serde(default)]
    preprocessor: Preprocessors,
}

#[derive(Default, Deserialize)]
struct Preprocessors {
    #[serde(default)]
    admonish: Option<toml::Table>,

    #[serde(flatten)]
    _other: toml::Table,
}

/// Load the plugin specific config as a toml string, for private deserialization.
fn admonish_config_string(config: &Config) -> Result<String> {
    Ok(toml_mdbook::to_string(
        &config
            .preprocessor
            .admonish
            .as_ref()
            .context("No configuration for mdbook-admonish in book.toml")?,
    )?)
}

fn handle_generate_custom(proj_dir: PathBuf, output: PathBuf) -> Result<()> {
    let config = proj_dir.join("book.toml");
    log::info!("Reading configuration file '{}'", config.display());
    let data = fs::read_to_string(&config)
        .with_context(|| format!("can't read configuration file '{}'", config.display()))?;
    let config: Config = toml::from_str(&data).context("Invalid configuration file")?;

    let css =
        mdbook_admonish::custom::css_from_config(&proj_dir, &admonish_config_string(&config)?)?;

    log::info!("Writing custom CSS file '{}'", output.display());
    fs::write(output, css)?;
    Ok(())
}

#[cfg(feature = "cli-install")]
mod install {
    use anyhow::{Context, Result};
    use path_slash::PathExt;
    use std::borrow::Cow;
    use std::path::Path;
    use std::{
        fs::{self, File},
        io::Write,
        path::PathBuf,
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

    // Normalize path to UNIX style.
    // This avoids conflicts/rewriting files when projects are used under different
    // operating systems (e.g. on Windows, after being used on Linux)
    //
    // https://github.com/tommilligan/mdbook-admonish/issues/161
    fn normalize_config_file_path(path: &Path) -> Result<Cow<'_, str>> {
        path.to_slash()
            .context("UNIX style path normalization error")
    }

    pub fn handle_install(proj_dir: PathBuf, css_dir: PathBuf) -> Result<()> {
        let config = proj_dir.join("book.toml");
        log::info!("Reading configuration file '{}'", config.display());
        let toml = fs::read_to_string(&config)
            .with_context(|| format!("can't read configuration file '{}'", config.display()))?;
        let mut doc = toml
            .parse::<Document>()
            .context("configuration is not valid TOML")?;

        if let Ok(preprocessor) = preprocessor(&mut doc) {
            const ASSETS_VERSION: &str = std::include_str!("./assets/VERSION");
            let value = toml_edit::value(
                toml_edit::Value::from(ASSETS_VERSION.trim())
                    .decorated(" ", " # do not edit: managed by `mdbook-admonish install`"),
            );
            preprocessor["assets_version"] = value;
        } else {
            log::info!("Unexpected configuration, not updating preprocessor configuration");
        };

        let mut additional_css = additional_css(&mut doc);
        for (name, content) in ADMONISH_CSS_FILES {
            let filepath = proj_dir.join(css_dir.clone()).join(name);
            // Normalize path to remove no-op components
            // https://github.com/tommilligan/mdbook-admonish/issues/47
            let filepath: PathBuf = filepath.components().collect();

            if let Ok(ref mut additional_css) = additional_css {
                let filepath_str = normalize_config_file_path(&filepath)?;

                if !additional_css.contains_str(&filepath_str) {
                    log::info!("Adding '{filepath_str}' to 'additional-css'");
                    additional_css.push(filepath_str.as_ref());
                }
            } else {
                log::warn!("Unexpected configuration, not updating 'additional-css'");
            }

            log::info!(
                "Copying '{name}' to '{filepath}'",
                filepath = filepath.display()
            );
            let mut file = File::create(&filepath).context("can't open file for writing")?;
            file.write_all(content)
                .context("can't write content to file")?;
        }

        let new_toml = doc.to_string();
        if new_toml != toml {
            log::info!("Saving changed configuration to '{}'", config.display());
            let mut file =
                File::create(config).context("can't open configuration file for writing.")?;
            file.write_all(new_toml.as_bytes())
                .context("can't write configuration")?;
        } else {
            log::info!("Configuration '{}' already up to date", config.display());
        }

        log::info!("mdbook-admonish is now installed. You can start using it in your book.");
        let codeblock = r#"```admonish warning
A beautifully styled message.
```"#;
        log::info!("Add a code block like:\n{}", codeblock);
        Ok(())
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

    #[cfg(test)]
    mod test {
        use super::*;

        /// This test seems redundant, but would fail on Windows.
        ///
        /// We want to always convert to a fixed output string style, independant
        /// of runtime platform, and forward slashes in relative paths are fine on
        /// Windows.
        #[test]
        fn test_normalize_config_file_path() {
            let input = PathBuf::from(".")
                .join("css-dir")
                .join("mdbook-admonish.css");
            let expected = "./css-dir/mdbook-admonish.css";
            let actual = normalize_config_file_path(&input).unwrap();
            assert_eq!(actual.as_ref(), expected);
        }
    }
}
