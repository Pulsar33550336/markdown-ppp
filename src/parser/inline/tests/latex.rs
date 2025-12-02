use crate::ast::*;
use crate::parser::{parse_markdown, MarkdownParserState};

#[test]
fn inline_latex() {
    let doc = parse_markdown(MarkdownParserState::default(), "$a^2 + b^2 = c^2$").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Latex(
                "a^2 + b^2 = c^2".to_string()
            )])],
        }
    );
}

#[test]
fn inline_latex_with_text() {
    let doc = parse_markdown(MarkdownParserState::default(), "The formula is $E=mc^2$.").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![
                Inline::Text("The formula is ".to_string()),
                Inline::Latex("E=mc^2".to_string()),
                Inline::Text(".".to_string()),
            ])],
        }
    );
}
