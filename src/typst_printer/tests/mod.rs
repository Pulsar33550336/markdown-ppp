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
    assert_eq!(result.trim(), r#"#par[#"Hello, world!"]"#);
}

#[test]
fn test_typst_escaping() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![Inline::Text(
            "Special chars: \\ \"".to_string(),
        )])],
    };

    let result = render_typst(&doc, Config::default());
    assert_eq!(result.trim(), r#"#par[#"Special chars: \\ \""]"#);
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
    let expected = [
        r#"#heading(level: 1, [#"Level 1"])"#,
        r#"#heading(level: 2, [#"Level 2"])"#,
        r#"#heading(level: 1, [#"Setext 1"])"#,
    ]
    .join("\n\n");
    assert_eq!(result.trim(), expected);
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
    let expected = r##"#par[#"Normal "#emph[#"italic"]#" and "#strong[#"bold"]#" text."]"##;
    assert_eq!(result.trim(), expected);
}

#[test]
fn test_code_block() {
    let literal = "fn main() {\n    println!(\"Hello!\");\n}";
    let doc = Document {
        blocks: vec![Block::CodeBlock(CodeBlock {
            kind: CodeBlockKind::Fenced {
                info: Some("rust".to_string()),
            },
            literal: literal.to_string(),
        })],
    };

    let result = render_typst(&doc, Config::default());
    let expected = format!("#raw(block: true, lang: \"rust\", \"{}\")", literal);
    assert_eq!(result.trim(), expected);
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
    let expected = [
        "#list(\n  [#\"Item 1\"],\n  [[#sym.checked] #\"Done item\"],\n)",
        "#enum(\n  [#\"Numbered\"],\n)",
    ]
    .join("\n\n");
    assert_eq!(result.trim(), expected);
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
    let expected = [
        "#figure(table(",
        "  columns: (2),",
        "  align: (left + horizon, right + horizon),",
        r##"  [#"Header 1"],  [#"Header 2"],"##,
        r##"  [#"Cell 1"],  [#"Cell 2"],"##,
        "))",
    ]
    .join("\n");
    assert_eq!(result.trim(), expected);
}

#[test]
fn test_blockquote() {
    let doc = Document {
        blocks: vec![Block::BlockQuote(vec![Block::Paragraph(vec![
            Inline::Text("This is a quote.".to_string()),
        ])])],
    };

    let result = render_typst(&doc, Config::default());
    assert_eq!(
        result.trim(),
        r#"#quote(block: true)[#par[#"This is a quote."]]"#
    );
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
    let expected =
        r##"#par[#"Visit "#link("https://example.com", title: "Example Site")[#"this link"]#"."]"##;
    assert_eq!(result.trim(), expected);
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
    let expected = r##"#par[#"Visit "#link("https://example.com")#"."]"##;
    assert_eq!(result.trim(), expected);
}
