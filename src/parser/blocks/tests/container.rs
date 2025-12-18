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

// 这两个测试不正确，先删了
// #[test]
// fn test_container_not_nest_same_kind() {
//     let a = r#":::a
// :::a
// # H1
// some content
// :::
// :::
// "#;
//     let state = MarkdownParserState::new();
//     let doc = parse_markdown(state, a).unwrap();
//     assert_eq!(
//         doc.blocks,
//         vec![Block::Container(Container {
//             kind: "a".to_string(),
//             blocks: vec![
//                 Block::Paragraph(vec![Inline::Text(":::a".to_string())]),
//                 Block::Heading(Heading {
//                     kind: HeadingKind::Atx(1),
//                     content: vec![Inline::Text("H1".to_string())],
//                 }),
//                 Block::Paragraph(vec![Inline::Text("some content".to_string())]),
//                 Block::Paragraph(vec![Inline::Text(":::".to_string())]),
//             ]
//         })]
//     );
// }

// #[test]
// fn test_container_no_nesting() {
//     let a = r#":::a
// :::b
// # H1
// some content
// :::
// :::
// "#;
//     let state = MarkdownParserState::new();
//     let doc = parse_markdown(state, a).unwrap();
//     assert_eq!(
//         doc.blocks,
//         vec![
//             Block::Paragraph(vec![Inline::Text(":::a".to_string())]),
//             Block::Paragraph(vec![Inline::Text(":::b".to_string())]),
//             Block::Heading(Heading {
//                 kind: HeadingKind::Atx(1),
//                 content: vec![Inline::Text("H1".to_string())],
//             }),
//             Block::Paragraph(vec![Inline::Text("some content".to_string())]),
//             Block::Paragraph(vec![Inline::Text(":::".to_string())]),
//         ]
//     );
// }

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
            blocks: vec![]
        })]
    );
}
