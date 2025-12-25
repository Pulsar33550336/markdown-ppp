use std::rc::Rc;

use crate::{
    ast::{Block, Document, Inline},
    ast_transform::{macro_expansion::MacroTransformer, ExpandWith},
    parser::{parse_markdown, MarkdownParserState},
};

#[test]
fn test_macro_transformer() {
    let state = MarkdownParserState::default();
    let doc = parse_markdown(
        state,
        "{{ block_macro }}
",
    )
    .unwrap();

    let mut transformer = MacroTransformer {
        block_expander: Rc::new(|content| {
            if content == "block_macro" {
                vec![Block::Paragraph(vec![Inline::Text(
                    "Block macro replaced.".to_string(),
                )])]
            } else {
                vec![Block::Paragraph(vec![Inline::Text(
                    "Unknown block macro.".to_string(),
                )])]
            }
        }),
    };

    let expanded_doc = doc.expand_with(&mut transformer);
    let first_doc = expanded_doc.get(0).unwrap();

    let expected_doc = Document {
        blocks: vec![
            Block::Paragraph(vec![Inline::Text("Block macro replaced.".to_string())]),
        ],
    };

    assert_eq!(first_doc.blocks, expected_doc.blocks);
}
