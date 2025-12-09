use crate::ast::*;
use crate::typst_printer::util::{body, escape_typst};
use crate::typst_printer::ToDoc;
use pretty::{Arena, DocAllocator, DocBuilder};

impl<'a> ToDoc<'a> for Vec<Inline> {
    fn to_doc(&self, state: &'a crate::typst_printer::State<'a>) -> DocBuilder<'a, Arena<'a>, ()> {
        state
            .arena
            .concat(self.iter().map(|inline| inline.to_doc(state)))
    }
}

impl<'a> ToDoc<'a> for Inline {
    fn to_doc(&self, state: &'a crate::typst_printer::State<'a>) -> DocBuilder<'a, Arena<'a>, ()> {
        match self {
            Inline::Text(text) => {
                // let text = text.replace('\n', " ");
                // if text.trim().is_empty() {
                //     return state.arena.text(escape_typst(&text));
                // }
                // let words_or_spaces: Vec<_> = split_with_spaces(&text);
                // let words_or_spaces = words_or_spaces.into_iter().map(|v| match v {
                //     Some(word) => state.arena.text(escape_typst(word)),
                //     None => state.arena.softline(),
                // });
                // state.arena.concat(words_or_spaces)
                let escaped = escape_typst(&text);
                let formatted = format!("#\"{}\"", escaped);
                state.arena.text(formatted)
            }

            Inline::LineBreak => state.arena.hardline(),

            Inline::Code(code) => {
                let escaped_code = code.replace('\\', r"\\").replace('"', r#"\""#);
                body(
                    &state.arena,
                    "raw",
                    Some(state.arena.text(format!(r#""{}""#, escaped_code))),
                    vec![],
                )
            }

            Inline::Html(html) => body(
                &state.arena,
                "raw",
                None,
                vec![state.arena.text(escape_typst(html))],
            ),

            Inline::Link(link) => {
                let mut args = vec![state
                    .arena
                    .text(format!(r#""{}""#, escape_typst(&link.destination)))];
                if let Some(title) = &link.title {
                    args.push(
                        state
                            .arena
                            .text(format!(r#", title: "{}""#, escape_typst(title))),
                    );
                }
                body(
                    &state.arena,
                    "link",
                    Some(state.arena.concat(args)),
                    vec![link.children.to_doc(state)],
                )
            }

            Inline::LinkReference(link_ref) => {
                if let Some(definition) = state.get_link_definition(&link_ref.label) {
                    let url = escape_typst(&definition.destination);
                    let text = link_ref.text.to_doc(state);
                    let mut args = vec![state.arena.text(format!(r#""{}""#, url))];
                    if let Some(title) = &definition.title {
                        args.push(
                            state
                                .arena
                                .text(format!(r#", title: "{}""#, escape_typst(title))),
                        );
                    }
                    body(
                        &state.arena,
                        "link",
                        Some(state.arena.concat(args)),
                        vec![text],
                    )
                } else {
                    link_ref.text.to_doc(state)
                }
            }

            Inline::Image(image) => {
                let url = escape_typst(&image.destination);
                let alt = escape_typst(&image.alt);
                state
                    .arena
                    .text(format!("#image(\"{url}\", alt: \"{alt}\")"))
            }

            Inline::Emphasis(content) => state
                .arena
                .text("#emph[")
                .append(content.to_doc(state))
                .append(state.arena.text("]")),

            Inline::Strong(content) => state
                .arena
                .text("#strong[")
                .append(content.to_doc(state))
                .append(state.arena.text("]")),

            Inline::Strikethrough(content) => state
                .arena
                .text("#strike[")
                .append(content.to_doc(state))
                .append(state.arena.text("]")),

            Inline::Autolink(url) => {
                let escaped_url = escape_typst(url);
                body(
                    &state.arena,
                    "link",
                    Some(state.arena.text(format!(r#""{escaped_url}""#))),
                    vec![],
                )
            }

            Inline::FootnoteReference(label) => {
                if let Some(def) = state.get_footnote_definition(label) {
                    let content = def
                        .blocks
                        .iter()
                        .map(|block| block.to_doc(state))
                        .collect::<Vec<_>>();
                    state
                        .arena
                        .text("#footnote[")
                        .append(state.arena.concat(content))
                        .append(state.arena.text("]"))
                } else {
                    state
                        .arena
                        .text("[^")
                        .append(state.arena.text(escape_typst(label)))
                        .append(state.arena.text("]"))
                }
            }

            Inline::Empty => state.arena.nil(),

            Inline::Latex(latex) => state
                .arena
                .text("#mi(block: false, \"")
                .append(state.arena.text(escape_typst(&latex.clone())))
                .append(state.arena.text("\")")),
        }
    }
}
