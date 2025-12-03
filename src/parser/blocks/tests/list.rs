use crate::ast::*;
use crate::parser::{parse_markdown, MarkdownParserState};

#[test]
fn list1() {
    let doc = parse_markdown(MarkdownParserState::default(), "1. a").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::List(List {
                kind: ListKind::Ordered(ListOrderedKindOptions { start: 1 }),
                items: vec![ListItem {
                    task: None,
                    blocks: vec![Block::Paragraph(vec![Inline::Text("a".to_owned())])]
                }]
            })]
        }
    );
}

#[test]
fn list2() {
    let doc = parse_markdown(MarkdownParserState::default(), "0100. a").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::List(List {
                kind: ListKind::Ordered(ListOrderedKindOptions { start: 100 }),
                items: vec![ListItem {
                    task: None,
                    blocks: vec![Block::Paragraph(vec![Inline::Text("a".to_owned())])]
                }]
            })]
        }
    );
}

#[test]
fn list3() {
    let doc = parse_markdown(MarkdownParserState::default(), "1) a").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::List(List {
                kind: ListKind::Ordered(ListOrderedKindOptions { start: 1 }),
                items: vec![ListItem {
                    task: None,
                    blocks: vec![Block::Paragraph(vec![Inline::Text("a".to_owned())])]
                }]
            })]
        }
    );
}

#[test]
fn list4() {
    let doc = parse_markdown(MarkdownParserState::default(), " -   a").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::List(List {
                kind: ListKind::Bullet(ListBulletKind::Dash),
                items: vec![ListItem {
                    task: None,
                    blocks: vec![Block::Paragraph(vec![Inline::Text("a".to_owned())])]
                }]
            })]
        }
    );
}

#[test]
fn list5() {
    let doc = parse_markdown(MarkdownParserState::default(), "1. a\n2. b").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::List(List {
                kind: ListKind::Ordered(ListOrderedKindOptions { start: 1 }),
                items: vec![
                    ListItem {
                        task: None,
                        blocks: vec![Block::Paragraph(vec![Inline::Text("a".to_owned())])]
                    },
                    ListItem {
                        task: None,
                        blocks: vec![Block::Paragraph(vec![Inline::Text("b".to_owned())])]
                    }
                ]
            })]
        }
    );
}

#[test]
fn list6() {
    let doc = parse_markdown(MarkdownParserState::default(), " - a\nb").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::List(List {
                kind: ListKind::Bullet(ListBulletKind::Dash),
                items: vec![ListItem {
                    task: None,
                    blocks: vec![Block::Paragraph(vec![Inline::Text("a\nb".to_owned())])]
                }]
            })]
        }
    );
}

#[test]
fn list7() {
    let doc = parse_markdown(MarkdownParserState::default(), " - a\nb\n\nc").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![
                Block::List(List {
                    kind: ListKind::Bullet(ListBulletKind::Dash),
                    items: vec![ListItem {
                        task: None,
                        blocks: vec![Block::Paragraph(vec![Inline::Text("a\nb".to_owned())])]
                    }]
                }),
                Block::Paragraph(vec![Inline::Text("c".to_owned())])
            ]
        },
    );
}

#[test]
fn list8() {
    let doc = parse_markdown(MarkdownParserState::default(), " - a\nb\n\n   c").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::List(List {
                kind: ListKind::Bullet(ListBulletKind::Dash),
                items: vec![ListItem {
                    task: None,
                    blocks: vec![
                        Block::Paragraph(vec![Inline::Text("a\nb".to_owned())]),
                        Block::Paragraph(vec![Inline::Text("c".to_owned())]),
                    ]
                }]
            })]
        },
    );
}

#[test]
fn list9() {
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "1. list1\n   * list2\n   * list2\n2. list1",
    )
    .unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::List(List {
                kind: ListKind::Ordered(ListOrderedKindOptions { start: 1 }),
                items: vec![
                    ListItem {
                        task: None,
                        blocks: vec![
                            Block::Paragraph(vec![Inline::Text("list1".to_owned())]),
                            Block::List(List {
                                kind: ListKind::Bullet(ListBulletKind::Star),
                                items: vec![
                                    ListItem {
                                        task: None,
                                        blocks: vec![Block::Paragraph(vec![Inline::Text(
                                            "list2".to_owned()
                                        )]),]
                                    },
                                    ListItem {
                                        task: None,
                                        blocks: vec![Block::Paragraph(vec![Inline::Text(
                                            "list2".to_owned()
                                        )]),]
                                    }
                                ]
                            })
                        ]
                    },
                    ListItem {
                        task: None,
                        blocks: vec![Block::Paragraph(vec![Inline::Text("list1".to_owned())])]
                    }
                ]
            })]
        },
    );
}

#[test]
fn list10() {
    let doc = parse_markdown(MarkdownParserState::default(), " * list1\n * list1").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::List(List {
                kind: ListKind::Bullet(ListBulletKind::Star),
                items: vec![
                    ListItem {
                        task: None,
                        blocks: vec![Block::Paragraph(vec![Inline::Text("list1".to_owned())])]
                    },
                    ListItem {
                        task: None,
                        blocks: vec![Block::Paragraph(vec![Inline::Text("list1".to_owned())])]
                    }
                ]
            })]
        },
    );
}

#[test]
fn task_list1() {
    let doc = parse_markdown(MarkdownParserState::default(), " - [ ] a").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::List(List {
                kind: ListKind::Bullet(ListBulletKind::Dash),
                items: vec![ListItem {
                    task: Some(TaskState::Incomplete),
                    blocks: vec![Block::Paragraph(vec![Inline::Text("a".to_owned())])]
                }]
            })]
        },
    );
}

#[test]
fn task_list2() {
    let doc = parse_markdown(MarkdownParserState::default(), " - [x] a").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::List(List {
                kind: ListKind::Bullet(ListBulletKind::Dash),
                items: vec![ListItem {
                    task: Some(TaskState::Complete),
                    blocks: vec![Block::Paragraph(vec![Inline::Text("a".to_owned())])]
                }]
            })]
        },
    );
}

#[test]
fn task_list3() {
    let doc = parse_markdown(MarkdownParserState::default(), " -   [x]   a").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::List(List {
                kind: ListKind::Bullet(ListBulletKind::Dash),
                items: vec![ListItem {
                    task: Some(TaskState::Complete),
                    blocks: vec![Block::Paragraph(vec![Inline::Text("a".to_owned())])]
                }]
            })]
        },
    );
}

#[test]
fn task_list4() {
    let doc = parse_markdown(MarkdownParserState::default(), " - [ ] ").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::List(List {
                kind: ListKind::Bullet(ListBulletKind::Dash),
                items: vec![ListItem {
                    task: Some(TaskState::Incomplete),
                    blocks: vec![]
                }]
            })]
        },
    );
}

#[test]
fn task_list5() {
    let doc = parse_markdown(MarkdownParserState::default(), "  -  [ ] \n\n     a").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::List(List {
                kind: ListKind::Bullet(ListBulletKind::Dash),
                items: vec![ListItem {
                    task: Some(TaskState::Incomplete),
                    blocks: vec![Block::Paragraph(vec![Inline::Text("a".to_owned())])]
                }]
            })]
        },
    );
}
