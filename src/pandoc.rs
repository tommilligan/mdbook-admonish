use mdbook::errors::Result as MdbookResult;

use crate::{
    book_config::OnFailure,
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

    let ADMONISH_KIND = "note".to_owned();

    let mut pandoc = pandoc::new();
    pandoc.set_input(pandoc::InputKind::Pipe(content.into()));
    pandoc.set_input_format(pandoc::InputFormat::CommonmarkX, Vec::new());
    pandoc.set_output_format(pandoc::OutputFormat::CommonmarkX, Vec::new());
    pandoc.set_output(pandoc::OutputKind::Pipe);
    pandoc.add_filter(|json| {
        pandoc_ast::filter(json, |mut pandoc| {
            for block in &mut pandoc.blocks {
                println!("{:?}", &block);
                use pandoc_ast::Block;
                use pandoc_ast::Inline;
                match block {
                    Block::Div((identifier, classes, kv), content) => {
                        if let Some(ADMONISH_KIND) = classes.first() {
                            // TODO: generate values for below by reading the pandoc input
                            // decide on a syntax for kv and classes, admonition type, and plug
                            // that in to our existing structs which deal with links and overrides
                            // etc.

                            *block = Block::Div(
                                (
                                    // TODO: get from kv
                                    "admonition-title".to_owned(),
                                    // TODO: compute from input above
                                    vec!["admonition".to_owned(), "admonish-note".to_owned()],
                                    vec![
                                        ("role".to_owned(), "note".to_owned()),
                                        (
                                            "aria-labelledby".to_owned(),
                                            "admonition-title-title".to_owned(),
                                        ),
                                    ],
                                ),
                                vec![
                                    Block::Div(
                                        // TODO: add anchor link elements (a, href, etc)
                                        ("admonition-title-title".to_owned(), vec![], vec![]),
                                        vec![Block::Plain(vec![Inline::Str("Title".to_owned())])],
                                    ),
                                    Block::Div(("".to_owned(), vec![], vec![]), content.clone()),
                                ],
                            )
                        }
                    }
                    _ => {}
                }
            }
            pandoc
        })
    });
    let result = pandoc.execute().unwrap();
    use pandoc::PandocOutput::*;
    let output = match result {
        ToBuffer(output) => output,
        ToFile(_) | ToBufferRaw(_) => panic!("expected buffer output"),
    };
    Ok(output)
}
