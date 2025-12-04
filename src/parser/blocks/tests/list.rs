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
fn lazy_continuation_lines() {
    let markdown = r#"
1. 将 $A_1, A_2$ 染成红色，将 $A_3$ 染成蓝色（$\color{red}{1}\color{red}{2}\color{blue}{1}$），其得分计算方式如下：

- 对于 $A_1$，由于其左侧没有红色的数，所以 $C_1 = 0$。
- 对于 $A_2$，其左侧与其最靠近的红色数为 $A_1$。由于 $A_1 \neq A_2$，所以 $C_2 = 0$。
- 对于 $A_3$，由于其左侧没有蓝色的数，所以 $C_3 = 0$。
  该方案最终得分为 $C_1 + C_2 + C_3 = 0$。

2. 将 $A_1, A_2, A_3$ 全部染成红色（$\color{red}{121}$），其得分计算方式如下：

- 对于 $A_1$，由于其左侧没有红色的数，所以 $C_1 = 0$。
- 对于 $A_2$，其左侧与其最靠近的红色数为 $A_1$。由于 $A_1 \neq A_2$，所以 $C_2 = 0$。
- 对于 $A_3$，其左侧与其最靠近的红色数为 $A_2$。由于 $A_2 \neq A_3$，所以 $C_3 = 0$。
  该方案最终得分为 $C_1 + C_2 + C_3 = 0$。

3. 将 $A_1, A_3$ 染成红色，将 $A_2$ 染成蓝色（$\color{red}{1}\color{blue}{2}\color{red}{1}$），其得分计算方式如下：

- 对于 $A_1$，由于其左侧没有红色的数，所以 $C_1 = 0$。
- 对于 $A_2$，由于其左侧没有蓝色的数，所以 $C_2 = 0$。
- 对于 $A_3$，其左侧与其最靠近的红色数为 $A_1$。由于 $A_1 = A_3$，所以 $C_3 = A_3 = 1$。
  该方案最终得分为 $C_1 + C_2 + C_3 = 1$。
"#;
    let doc = parse_markdown(MarkdownParserState::default(), markdown.trim()).unwrap();
    assert_eq!(doc.blocks.len(), 1);
    match &doc.blocks[0] {
        Block::List(list) => {
            assert_eq!(list.items.len(), 3);
            assert!(matches!(list.kind, ListKind::Ordered(_)));
        }
        _ => panic!("Expected a single list block"),
    }
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
