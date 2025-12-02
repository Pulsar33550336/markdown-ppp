use crate::ast::*;
use crate::parser::{parse_markdown, MarkdownParserState};

#[test]
fn block_latex() {
    let doc = parse_markdown(MarkdownParserState::default(), "$$\\sum_{i=0}^n i = \\frac{n(n+1)}{2}$$").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::LatexBlock(
                "\\sum_{i=0}^n i = \\frac{n(n+1)}{2}".to_string()
            )],
        }
    );
}

#[test]
fn block_latex_with_text() {
    let doc = parse_markdown(MarkdownParserState::default(), "The formula is:\n\n$$\\int_0^\\infty e^{-x^2} dx = \\frac{\\sqrt{\\pi}}{2}$$\n\nEnd of formula.").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![
                Block::Paragraph(vec![Inline::Text("The formula is:".to_string())]),
                Block::LatexBlock(
                    "\\int_0^\\infty e^{-x^2} dx = \\frac{\\sqrt{\\pi}}{2}".to_string()
                ),
                Block::Paragraph(vec![Inline::Text("End of formula.".to_string())]),
            ],
        }
    );
}
