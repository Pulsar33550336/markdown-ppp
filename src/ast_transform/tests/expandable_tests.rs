//! Tests for expandable transformations (1-to-many AST transformations)

use crate::ast::*;
use crate::ast_transform::{ExpandWith, Transformer};

/// Test transformer that splits paragraphs containing "SPLIT" into two paragraphs
struct ParagraphSplitter;

impl Transformer for ParagraphSplitter {
    fn expand_block(&mut self, block: Block) -> Vec<Block> {
        if let Block::Paragraph(ref inlines) = block {
            let mut split_point = None;
            for (i, inline) in inlines.iter().enumerate() {
                if let Inline::Text(text) = inline {
                    if text.contains("SPLIT") {
                        split_point = Some(i);
                        break;
                    }
                }
            }

            if let Some(split_at) = split_point {
                let (first_half, second_half) = inlines.split_at(split_at);

                let second_half = if second_half.len() > 1 {
                    second_half[1..].to_vec()
                } else {
                    vec![]
                };

                let mut result = Vec::new();

                if !first_half.is_empty() {
                    result.push(Block::Paragraph(first_half.to_vec()));
                }

                if !second_half.is_empty() {
                    result.push(Block::Paragraph(second_half));
                }

                return result;
            }
        }
        self.walk_expand_block(block)
    }
}

#[test]
fn test_paragraph_splitter() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![
            Inline::Text("Before ".to_string()),
            Inline::Text("SPLIT".to_string()),
            Inline::Text(" After".to_string()),
        ])],
    };

    let mut transformer = ParagraphSplitter;
    let result = doc.expand_with(&mut transformer);

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].blocks.len(), 2);

    // Check first paragraph
    if let Block::Paragraph(inlines) = &result[0].blocks[0] {
        assert_eq!(inlines.len(), 1);
        assert_eq!(inlines[0], Inline::Text("Before ".to_string()));
    } else {
        panic!("Expected first block to be a paragraph");
    }

    // Check second paragraph
    if let Block::Paragraph(inlines) = &result[0].blocks[1] {
        assert_eq!(inlines.len(), 1);
        assert_eq!(inlines[0], Inline::Text(" After".to_string()));
    } else {
        panic!("Expected second block to be a paragraph");
    }
}

/// Test transformer that expands text containing "EXPAND" into multiple text nodes
struct TextExpander;

impl Transformer for TextExpander {
    fn expand_inline(&mut self, inline: Inline) -> Vec<Inline> {
        if let Inline::Text(text) = &inline {
            if text.contains("EXPAND") {
                let parts: Vec<&str> = text.split("EXPAND").collect();
                let mut result = Vec::new();

                for (i, part) in parts.iter().enumerate() {
                    if !part.is_empty() {
                        result.push(Inline::Text(part.to_string()));
                    }
                    if i < parts.len() - 1 {
                        result.push(Inline::Emphasis(vec![Inline::Text("EXPANDED".to_string())]));
                    }
                }
                return result;
            }
        }
        self.walk_expand_inline(inline)
    }
}

#[test]
fn test_text_expander() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![Inline::Text(
            "Hello EXPAND World EXPAND !".to_string(),
        )])],
    };

    let mut transformer = TextExpander;
    let result = doc.expand_with(&mut transformer);

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].blocks.len(), 1);

    if let Block::Paragraph(inlines) = &result[0].blocks[0] {
        assert_eq!(inlines.len(), 5); // "Hello ", EXPANDED, " World ", EXPANDED, " !"

        assert_eq!(inlines[0], Inline::Text("Hello ".to_string()));
        assert_eq!(
            inlines[1],
            Inline::Emphasis(vec![Inline::Text("EXPANDED".to_string())])
        );
        assert_eq!(inlines[2], Inline::Text(" World ".to_string()));
        assert_eq!(
            inlines[3],
            Inline::Emphasis(vec![Inline::Text("EXPANDED".to_string())])
        );
        assert_eq!(inlines[4], Inline::Text(" !".to_string()));
    } else {
        panic!("Expected paragraph");
    }
}

/// Test transformer that converts headings into heading + paragraph pairs
struct HeadingExpander;

impl Transformer for HeadingExpander {
    fn expand_block(&mut self, block: Block) -> Vec<Block> {
        if let Block::Heading(heading) = block {
            let mut transformed_heading = heading.clone();
            transformed_heading.content = transformed_heading
                .content
                .into_iter()
                .flat_map(|inline| self.expand_inline(inline))
                .collect();

            // Create an additional paragraph with metadata
            let meta_paragraph =
                Block::Paragraph(vec![Inline::Emphasis(vec![Inline::Text(format!(
                    "This is a {} heading",
                    match &heading.kind {
                        HeadingKind::Atx(level) => format!("level {level}"),
                        HeadingKind::Setext(setext) => match setext {
                            SetextHeading::Level1 => "level 1".to_string(),
                            SetextHeading::Level2 => "level 2".to_string(),
                        },
                    }
                ))])]);

            vec![Block::Heading(transformed_heading), meta_paragraph]
        } else {
            self.walk_expand_block(block)
        }
    }
}

#[test]
fn test_heading_expander() {
    let doc = Document {
        blocks: vec![Block::Heading(Heading {
            kind: HeadingKind::Atx(2),
            content: vec![Inline::Text("Test Heading".to_string())],
        })],
    };

    let mut transformer = HeadingExpander;
    let result = doc.expand_with(&mut transformer);

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].blocks.len(), 2);

    // Check heading is preserved
    if let Block::Heading(heading) = &result[0].blocks[0] {
        assert_eq!(heading.kind, HeadingKind::Atx(2));
        assert_eq!(heading.content[0], Inline::Text("Test Heading".to_string()));
    } else {
        panic!("Expected first block to be a heading");
    }

    // Check metadata paragraph is added
    if let Block::Paragraph(inlines) = &result[0].blocks[1] {
        assert_eq!(inlines.len(), 1);
        if let Inline::Emphasis(content) = &inlines[0] {
            assert_eq!(
                content[0],
                Inline::Text("This is a level 2 heading".to_string())
            );
        } else {
            panic!("Expected emphasis in metadata paragraph");
        }
    } else {
        panic!("Expected second block to be a paragraph");
    }
}

/// Test using the ExpandWith trait for convenient API
#[test]
fn test_expand_with_trait() {
    let block = Block::Paragraph(vec![
        Inline::Text("Before ".to_string()),
        Inline::Text("SPLIT".to_string()),
        Inline::Text(" After".to_string()),
    ]);

    let mut transformer = ParagraphSplitter;
    let result = block.expand_with(&mut transformer);

    assert_eq!(result.len(), 2);

    if let Block::Paragraph(inlines) = &result[0] {
        assert_eq!(inlines[0], Inline::Text("Before ".to_string()));
    } else {
        panic!("Expected first result to be a paragraph");
    }

    if let Block::Paragraph(inlines) = &result[1] {
        assert_eq!(inlines[0], Inline::Text(" After".to_string()));
    } else {
        panic!("Expected second result to be a paragraph");
    }
}

/// Test transformer that doesn't expand (returns single element)
struct NoOpExpander;

impl Transformer for NoOpExpander {}

#[test]
fn test_no_expansion() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![Inline::Text(
            "Regular paragraph".to_string(),
        )])],
    };

    let mut transformer = NoOpExpander;
    let result = doc.expand_with(&mut transformer);

    // Should return exactly one document with one block
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].blocks.len(), 1);

    if let Block::Paragraph(inlines) = &result[0].blocks[0] {
        assert_eq!(inlines[0], Inline::Text("Regular paragraph".to_string()));
    } else {
        panic!("Expected paragraph");
    }
}

/// Test complex transformation that combines multiple expansion strategies
struct ComplexExpander;

impl Transformer for ComplexExpander {
    fn expand_block(&mut self, block: Block) -> Vec<Block> {
        if let Block::Paragraph(inlines) = &block {
            if inlines
                .iter()
                .any(|inline| matches!(inline, Inline::Text(t) if t.contains("SPLIT")))
            {
                // This is a simplified logic for splitting, a real implementation
                // would be more robust.
                let mut result = Vec::new();
                let mut current_paragraph = Vec::new();

                for inline in inlines {
                    if let Inline::Text(text) = inline {
                        if text.contains("SPLIT") {
                            if !current_paragraph.is_empty() {
                                result.push(Block::Paragraph(current_paragraph));
                            }
                            current_paragraph = Vec::new();
                            continue;
                        }
                    }
                    current_paragraph.push(inline.clone());
                }

                if !current_paragraph.is_empty() {
                    result.push(Block::Paragraph(current_paragraph));
                }
                return result
                    .into_iter()
                    .flat_map(|b| self.expand_block(b))
                    .collect();
            }
        }

        if let Block::Heading(heading) = block {
            let mut transformed_heading = heading;
            transformed_heading.content = transformed_heading
                .content
                .into_iter()
                .flat_map(|inline| self.expand_inline(inline))
                .collect();

            let meta_paragraph =
                Block::Paragraph(vec![Inline::Text("(Generated metadata)".to_string())]);

            return vec![Block::Heading(transformed_heading), meta_paragraph];
        }

        self.walk_expand_block(block)
    }

    fn expand_inline(&mut self, inline: Inline) -> Vec<Inline> {
        if let Inline::Text(text) = &inline {
            if text.contains("EXPAND") {
                return vec![
                    Inline::Text(text.replace("EXPAND", "")),
                    Inline::Strong(vec![Inline::Text("EXPANDED".to_string())]),
                ];
            }
        }
        self.walk_expand_inline(inline)
    }
}

#[test]
fn test_complex_expansion() {
    let doc = Document {
        blocks: vec![
            Block::Heading(Heading {
                kind: HeadingKind::Atx(1),
                content: vec![Inline::Text("Main EXPAND Title".to_string())],
            }),
            Block::Paragraph(vec![
                Inline::Text("First EXPAND part".to_string()),
                Inline::Text("SPLIT".to_string()),
                Inline::Text("Second EXPAND part".to_string()),
            ]),
        ],
    };

    let mut transformer = ComplexExpander;
    let result = doc.expand_with(&mut transformer);

    assert_eq!(result.len(), 1);
    // Should have: heading + meta paragraph + first paragraph + second paragraph = 4 blocks
    assert_eq!(result[0].blocks.len(), 4);

    // Check heading expansion
    if let Block::Heading(heading) = &result[0].blocks[0] {
        assert_eq!(heading.content.len(), 2); // Text + Strong
        if let Inline::Text(text) = &heading.content[0] {
            assert_eq!(text, "Main  Title"); // "EXPAND" removed
        }
        if let Inline::Strong(content) = &heading.content[1] {
            if let Inline::Text(text) = &content[0] {
                assert_eq!(text, "EXPANDED");
            }
        }
    } else {
        panic!("Expected first block to be heading");
    }

    // Check metadata paragraph
    if let Block::Paragraph(inlines) = &result[0].blocks[1] {
        assert_eq!(inlines[0], Inline::Text("(Generated metadata)".to_string()));
    } else {
        panic!("Expected second block to be metadata paragraph");
    }

    // Check split paragraphs with inline expansion
    if let Block::Paragraph(inlines) = &result[0].blocks[2] {
        assert_eq!(inlines.len(), 2); // Text + Strong from "EXPAND"
    } else {
        panic!("Expected third block to be paragraph");
    }

    if let Block::Paragraph(inlines) = &result[0].blocks[3] {
        assert_eq!(inlines.len(), 2); // Text + Strong from "EXPAND"
    } else {
        panic!("Expected fourth block to be paragraph");
    }
}
