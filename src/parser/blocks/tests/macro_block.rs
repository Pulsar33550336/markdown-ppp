use crate::{
    ast::{Block, Document},
    parser::{parse_markdown, MarkdownParserState},
};

#[test]
fn test_macro_block() {
    let text = "{{ my_block_macro }}\n";
    let state = MarkdownParserState::default();
    let doc = parse_markdown(state, text).unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::MacroBlock("my_block_macro".to_string())]
        }
    );
}

#[test]
fn test_macro_block_with_extra_spaces() {
    let text = "{{  spaced_macro  }}\n";
    let state = MarkdownParserState::default();
    let doc = parse_markdown(state, text).unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::MacroBlock("spaced_macro".to_string())]
        }
    );
}

#[test]
fn test_fail_if_not_on_own_line() {
    let text = "This is not a macro block: {{ not_a_block }}";
    let state = MarkdownParserState::default();
    let doc = parse_markdown(state, text).unwrap();
    assert!(!matches!(doc.blocks[0], Block::MacroBlock(_)));
}
