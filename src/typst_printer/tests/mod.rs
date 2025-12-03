mod comprehensive;
mod config_combinations;
mod edge_cases;

use crate::ast::*;
use crate::typst_printer::{config::*, render_typst};

#[test]
fn test_simple_paragraph() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![Inline::Text(
            "Hello, world!".to_string(),
        )])],
    };

    let result = render_typst(&doc, Config::default());
    assert_eq!(result.trim(), "Hello, world!");
}

#[test]
fn test_typst_escaping() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![Inline::Text(
            "Special chars: * _ \\ \"".to_string(),
        )])],
    };

    let result = render_typst(&doc, Config::default());
    // assert!(result.contains(r"\*"));
    // assert!(result.contains(r"\_"));
    assert!(result.contains(r"\\"));
    assert!(result.contains(r#"\""#));
}

#[test]
fn test_headings() {
    let doc = Document {
        blocks: vec![
            Block::Heading(Heading {
                kind: HeadingKind::Atx(1),
                content: vec![Inline::Text("Level 1".to_string())],
            }),
            Block::Heading(Heading {
                kind: HeadingKind::Atx(2),
                content: vec![Inline::Text("Level 2".to_string())],
            }),
            Block::Heading(Heading {
                kind: HeadingKind::Setext(SetextHeading::Level1),
                content: vec![Inline::Text("Setext 1".to_string())],
            }),
        ],
    };

    let result = render_typst(&doc, Config::default());
    assert!(result.contains("= Level 1"));
    assert!(result.contains("== Level 2"));
    assert!(result.contains("= Setext 1"));
}

#[test]
fn test_emphasis() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![
            Inline::Text("Normal ".to_string()),
            Inline::Emphasis(vec![Inline::Text("italic".to_string())]),
            Inline::Text(" and ".to_string()),
            Inline::Strong(vec![Inline::Text("bold".to_string())]),
            Inline::Text(" text.".to_string()),
        ])],
    };

    let result = render_typst(&doc, Config::default());
    assert!(result.contains("_italic_"));
    assert!(result.contains("*bold*"));
}

#[test]
fn test_code_block() {
    let doc = Document {
        blocks: vec![Block::CodeBlock(CodeBlock {
            kind: CodeBlockKind::Fenced {
                info: Some("rust".to_string()),
            },
            literal: "fn main() {\n    println!(\"Hello!\");\n}".to_string(),
        })],
    };

    let result = render_typst(&doc, Config::default());
    assert!(result.contains("```rust"));
    assert!(result.contains("fn main()"));
}

#[test]
fn test_lists() {
    let doc = Document {
        blocks: vec![
            Block::List(List {
                kind: ListKind::Bullet(ListBulletKind::Star),
                items: vec![
                    ListItem {
                        task: None,
                        blocks: vec![Block::Paragraph(vec![Inline::Text("Item 1".to_string())])],
                    },
                    ListItem {
                        task: Some(TaskState::Complete),
                        blocks: vec![Block::Paragraph(vec![Inline::Text(
                            "Done item".to_string(),
                        )])],
                    },
                ],
            }),
            Block::List(List {
                kind: ListKind::Ordered(ListOrderedKindOptions { start: 1 }),
                items: vec![ListItem {
                    task: None,
                    blocks: vec![Block::Paragraph(vec![Inline::Text("Numbered".to_string())])],
                }],
            }),
        ],
    };

    let result = render_typst(&doc, Config::default());
    assert!(result.contains("- Item 1"));
    assert!(result.contains("[#sym.checked]"));
    assert!(result.contains("+ Numbered"));
}

#[test]
fn test_table() {
    let doc = Document {
        blocks: vec![Block::Table(Table {
            rows: vec![
                vec![
                    TableCell {
                        content: vec![Inline::Text("Header 1".to_string())],
                        colspan: None,
                        rowspan: None,
                        removed_by_extended_table: false,
                    },
                    TableCell {
                        content: vec![Inline::Text("Header 2".to_string())],
                        colspan: None,
                        rowspan: None,
                        removed_by_extended_table: false,
                    },
                ],
                vec![
                    TableCell {
                        content: vec![Inline::Text("Cell 1".to_string())],
                        colspan: None,
                        rowspan: None,
                        removed_by_extended_table: false,
                    },
                    TableCell {
                        content: vec![Inline::Text("Cell 2".to_string())],
                        colspan: None,
                        rowspan: None,
                        removed_by_extended_table: false,
                    },
                ],
            ],
            alignments: vec![Alignment::Left, Alignment::Right],
        })],
    };

    let result = render_typst(&doc, Config::default());
    assert!(result.contains("#figure(table"));
    assert!(result.contains("columns: (2)"));
    assert!(result.contains("[Header 1]"));
    assert!(result.contains("[Cell 1]"));
    assert!(result.contains("[Cell 2]"));
}

#[test]
fn test_blockquote() {
    let doc = Document {
        blocks: vec![Block::BlockQuote(vec![Block::Paragraph(vec![
            Inline::Text("This is a quote.".to_string()),
        ])])],
    };

    let result = render_typst(&doc, Config::default());
    assert!(result.contains("> This is a quote."));
}

#[test]
fn test_links() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![
            Inline::Text("Visit ".to_string()),
            Inline::Link(Link {
                destination: "https://example.com".to_string(),
                title: Some("Example Site".to_string()),
                children: vec![Inline::Text("this link".to_string())],
            }),
            Inline::Text(".".to_string()),
        ])],
    };

    let result = render_typst(&doc, Config::default());
    assert!(result.contains("#link(\"https://example.com\", title: \"Example Site\")[this link]"));
}

#[test]
fn test_autolink() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![
            Inline::Text("Visit ".to_string()),
            Inline::Autolink("https://example.com".to_string()),
            Inline::Text(".".to_string()),
        ])],
    };

    let result = render_typst(&doc, Config::default());
    assert!(result.contains("#link(\"https://example.com\")"));
}