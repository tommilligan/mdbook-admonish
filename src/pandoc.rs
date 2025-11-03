use mdbook::errors::Result as MdbookResult;
use mdbook::utils::unique_id_from_content;

use crate::{
    book_config::OnFailure,
    render::ANCHOR_ID_DEFAULT,
    types::{Overrides, RenderTextMode},
};

pub(crate) fn preprocess(
    content: &str,
    on_failure: OnFailure,
    overrides: &Overrides,
    render_text_mode: RenderTextMode,
) -> MdbookResult<String> {
    if render_text_mode == RenderTextMode::Strip {
        // We don't support stripping in pandoc - they just show up as divs.
        return Ok(content.to_owned());
    }

    let mut pandoc = pandoc::new();
    pandoc.set_input(pandoc::InputKind::Pipe(content.into()));
    pandoc.set_input_format(pandoc::InputFormat::CommonmarkX, Vec::new());
    pandoc.set_output_format(pandoc::OutputFormat::CommonmarkX, Vec::new());
    pandoc.set_output(pandoc::OutputKind::Pipe);

    pandoc.add_filter(|json| pandoc_ast::filter(json, process_blocks));

    let result = pandoc.execute().unwrap();
    use pandoc::PandocOutput::*;
    let output = match result {
        ToBuffer(output) => output,
        ToFile(_) | ToBufferRaw(_) => panic!("expected buffer output"),
    };
    Ok(output)
}

/// Process all a `Div` blocks like
///
/// :::admonition {title="this" kind="note" other="stuff"}
///  ....
/// :::
fn process_blocks(mut pandoc: pandoc_ast::Pandoc) -> pandoc_ast::Pandoc {
    const ADMONISH_ID_PREFIX: &str = "admonition-";
    const ADMONISH_DIV_NAME: &str = "admonition";

    let mut id_counter = &mut Default::default();

    for block in &mut pandoc.blocks {
        use pandoc_ast::Block;
        use pandoc_ast::Inline;

        match block {
            Block::Div((id, classes, kv), content) => {
                if !classes.contains(&ADMONISH_DIV_NAME.to_owned()) {
                    continue;
                }

                let mut kv_rest = vec![];
                let mut kind = "note";
                let mut title = "";

                kv.iter().for_each(|(k, v)| match k.as_str() {
                    "title" => title = v,
                    "kind" => kind = v,
                    _ => kv_rest.push((k.to_owned(), v.to_owned())),
                });

                if id.is_empty() {
                    *id = format!(
                        "{ADMONISH_ID_PREFIX}{}",
                        unique_id_from_content(
                            if !title.is_empty() {
                                &title
                            } else {
                                ANCHOR_ID_DEFAULT
                            },
                            &mut id_counter,
                        )
                    );
                }

                kv_rest.append(&mut vec![
                    ("role".to_owned(), "note".to_owned()),
                    ("aria-labelledby".to_owned(), format!("{id}-title")),
                ]);

                *block = Block::Div(
                    (
                        id.to_owned(),
                        vec!["admonition".to_owned(), format!("admonish-{kind}")],
                        kv_rest,
                    ),
                    vec![
                        Block::Div(
                            (format!("{id}-title").to_owned(), vec![], vec![]),
                            vec![Block::Plain(vec![Inline::Str(title.to_owned())])],
                        ),
                        Block::Div(
                            (format!("{id}-content").to_owned(), vec![], vec![]),
                            content.clone(),
                        ),
                    ],
                )
            }
            _ => {}
        }
    }

    pandoc
}
