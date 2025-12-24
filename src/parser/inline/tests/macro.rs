use crate::{
    ast::{Block, Document, Inline},
    parser::{parse_markdown, MarkdownParserState},
};

#[test]
fn test_inline_macro() {
    let text = "Hello, {{ my_inline_macro }}.";
    let state = MarkdownParserState::default();
    let doc = parse_markdown(state, text).unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![
                Inline::Text("Hello, ".to_string()),
                Inline::Macro("my_inline_macro".to_string()),
                Inline::Text(".".to_string()),
            ])]
        }
    );
}

#[test]
fn test_inline_macro_with_spaces() {
    let text = "Spaces: {{  spaced_macro  }}.";
    let state = MarkdownParserState::default();
    let doc = parse_markdown(state, text).unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![
                Inline::Text("Spaces: ".to_string()),
                Inline::Macro("spaced_macro".to_string()),
                Inline::Text(".".to_string()),
            ])]
        }
    );
}
