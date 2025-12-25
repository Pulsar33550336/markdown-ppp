use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{Block, Document, Inline},
    parser::{
        config::{InlineMacroReplacerFn, MarkdownParserConfig},
        parse_markdown, MarkdownParserState,
    },
};

#[test]
fn test_inline_macro_replacer() {
    let replacer: InlineMacroReplacerFn = Rc::new(RefCell::new(Box::new(|macro_content| {
        match macro_content {
            "my_inline_macro" => "replacement".to_string(),
            "another_macro" => "another replacement".to_string(),
            "outer {{ inner }}" => "OUTER".to_string(),
            _ => format!("unhandled: '{}'", macro_content),
        }
    })));

    let config = MarkdownParserConfig::default().with_inline_macro_replacer(replacer);
    let state = MarkdownParserState::with_config(config);
    let doc = parse_markdown(
        state,
        "Hello, {{ my_inline_macro }}. Nested: {{ outer {{ inner }} }}. and {{another_macro}}",
    )
    .unwrap();

    let expected_doc = Document {
        blocks: vec![Block::Paragraph(vec![Inline::Text(
            "Hello, replacement. Nested: OUTER. and another replacement".to_string(),
        )])],
    };

    assert_eq!(doc.blocks, expected_doc.blocks);
}
