use crate::ast::{Block, Container, Heading, HeadingKind, Inline};
use crate::parser::{parse_markdown, MarkdownParserState};

#[test]
fn test_container_simple() {
    let a = r#":::a
some content
:::
"#;
    let state = MarkdownParserState::new();
    let doc = parse_markdown(state, a).unwrap();
    assert_eq!(
        doc.blocks,
        vec![Block::Container(Container {
            kind: "a".to_string(),
            params: vec![],
            blocks: vec![Block::Paragraph(vec![Inline::Text(
                "some content".to_string()
            )])]
        })]
    );
}

#[test]
fn test_container_with_params() {
    let a = r#":::a{x=b y=c}
some content
:::
"#;
    let state = MarkdownParserState::new();
    let doc = parse_markdown(state, a).unwrap();
    assert_eq!(
        doc.blocks,
        vec![Block::Container(Container {
            kind: "a".to_string(),
            params: vec![
                ("x".to_string(), "b".to_string()),
                ("y".to_string(), "c".to_string())
            ],
            blocks: vec![Block::Paragraph(vec![Inline::Text(
                "some content".to_string()
            )])]
        })]
    );
}

#[test]
fn test_container_with_quoted_params() {
    let a = r#":::a{x="b c" y=d}
some content
:::
"#;
    let state = MarkdownParserState::new();
    let doc = parse_markdown(state, a).unwrap();
    assert_eq!(
        doc.blocks,
        vec![Block::Container(Container {
            kind: "a".to_string(),
            params: vec![
                ("x".to_string(), "b c".to_string()),
                ("y".to_string(), "d".to_string())
            ],
            blocks: vec![Block::Paragraph(vec![Inline::Text(
                "some content".to_string()
            )])]
        })]
    );
}

#[test]
fn test_container() {
    let a = r#":::a
# H1
some content
:::
"#;
    let state = MarkdownParserState::new();
    let doc = parse_markdown(state, a).unwrap();
    assert_eq!(
        doc.blocks,
        vec![Block::Container(Container {
            kind: "a".to_string(),
            params: vec![],
            blocks: vec![
                Block::Heading(Heading {
                    kind: HeadingKind::Atx(1),
                    content: vec![Inline::Text("H1".to_string())]
                }),
                Block::Paragraph(vec![Inline::Text("some content".to_string())])
            ]
        })]
    );
}

#[test]
fn test_empty_container() {
    let a = r#":::a
:::
"#;
    let state = MarkdownParserState::new();
    let doc = parse_markdown(state, a).unwrap();
    assert_eq!(
        doc.blocks,
        vec![Block::Container(Container {
            kind: "a".to_string(),
            params: vec![],
            blocks: vec![]
        })]
    );
}
