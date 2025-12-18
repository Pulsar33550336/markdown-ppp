use crate::ast::*;
use crate::typst_printer::util::{body, escape_typst};
use crate::typst_printer::ToDoc;
use pretty::{Arena, DocAllocator, DocBuilder};

impl<'a> ToDoc<'a> for Vec<Block> {
    fn to_doc(&self, state: &'a crate::typst_printer::State<'a>) -> DocBuilder<'a, Arena<'a>, ()> {
        let refs: Vec<_> = self.iter().collect();
        refs.to_doc(state)
    }
}

impl<'a> ToDoc<'a> for Vec<&Block> {
    fn to_doc(&self, state: &'a crate::typst_printer::State<'a>) -> DocBuilder<'a, Arena<'a>, ()> {
        state
            .arena
            .intersperse(
                self.iter().map(|block| block.to_doc(state)),
                state.arena.hardline().append(state.arena.hardline()),
            )
            .group()
    }
}

impl<'a> ToDoc<'a> for Block {
    fn to_doc(&self, state: &'a crate::typst_printer::State<'a>) -> DocBuilder<'a, Arena<'a>, ()> {
        match self {
            Block::Paragraph(inlines) => state
                .arena
                .text("#par[")
                .append(inlines.to_doc(state))
                .append("]"), //TODO: #par[]
            Block::Heading(heading) => {
                let level = match heading.kind {
                    HeadingKind::Atx(level) => level,
                    HeadingKind::Setext(SetextHeading::Level1) => 1,
                    HeadingKind::Setext(SetextHeading::Level2) => 2,
                };
                state
                    .arena
                    .text("#heading(level: ")
                    .append(level.to_string())
                    .append(", [")
                    // .append(state.arena.space())
                    .append(heading.content.to_doc(state))
                    .append("])")
            }

            Block::ThematicBreak => state.arena.text("#thematic-break"),

            Block::BlockQuote(blocks) => {
                if blocks.is_empty() {
                    state.arena.text("#quote(block: true)[]")
                } else {
                    state
                        .arena
                        .text("#quote(block: true)[")
                        .append(blocks.to_doc(state))
                        .append("]")
                }
            }

            Block::List(list) => list.to_doc(state),

            Block::CodeBlock(code_block) => {
                let lang = match &code_block.kind {
                    CodeBlockKind::Fenced { info: Some(lang) } => lang.as_str(),
                    _ => "",
                };

                let mut args = vec![state.arena.text("block: true")];
                if !lang.is_empty() {
                    args.push(state.arena.text(format!(r#", lang: "{}""#, lang)));
                }
                let escaped_code = code_block
                    .literal
                    .replace('\\', r"\\")
                    .replace('"', r#"\""#);
                args.push(state.arena.text(format!(r#", "{}""#, escaped_code)));

                body(&state.arena, "raw", Some(state.arena.concat(args)), vec![])
            }

            Block::HtmlBlock(html) => body(
                &state.arena,
                "raw",
                None,
                vec![state.arena.text(escape_typst(html))],
            ),

            Block::Definition(_) => state.arena.nil(),

            Block::Table(table) => table.to_doc(state),

            Block::FootnoteDefinition(_) => state.arena.nil(),

            Block::GitHubAlert(alert) => {
                let title = match &alert.alert_type {
                    GitHubAlertType::Note => "Note",
                    GitHubAlertType::Tip => "Tip",
                    GitHubAlertType::Important => "Important",
                    GitHubAlertType::Warning => "Warning",
                    GitHubAlertType::Caution => "Caution",
                    GitHubAlertType::Custom(s) => s,
                };

                state
                    .arena
                    .text("#rect(width: 100%, inset: 8pt, radius: 4pt, fill: luma(240), stroke: none, grid(columns: (auto, 1fr), column-gutter: 8pt, [*")
                    .append(state.arena.text(title.to_string()))
                    .append(state.arena.text("*], \n["))
                    .append(alert.blocks.to_doc(state))
                    .append(state.arena.text("]))"))
            }

            Block::Empty => state.arena.nil(),
            Block::LatexBlock(latex) => state
                .arena
                .text("#mi(block: true, \"")
                .append(state.arena.text(escape_typst(&latex.clone())))
                .append(state.arena.text("\")")),
            Block::Container(container) => {
                if container.kind == "figure" {
                    let mut args = Vec::new();
                    if let Some((_, caption)) = container.params.iter().find(|(k, _)| k == "caption") {
                        args.push(state.arena.text(format!("caption: [\"{}\"]", escape_typst(caption))));
                    }
                    let body = container.blocks.to_doc(state);
                    let mut doc = state.arena.text("#figure(").append(body);
                    if !args.is_empty() {
                        doc = doc.append(state.arena.text(", ")).append(state.arena.concat(args));
                    }
                    doc.append(state.arena.text(")"))
                } else {
                    let mut doc = state.arena.text(format!("#block(breakable: true, inset: (y: 0.5em), stroke: luma(190) + 1pt, width: 100%)[*{}*", container.kind));
                    if !container.blocks.is_empty() {
                        doc = doc.append(state.arena.hardline());
                        doc = doc.append(container.blocks.to_doc(state));
                    }
                    doc.append(state.arena.text("]"))
                }
            }
        }
    }
}

impl<'a> ToDoc<'a> for List {
    fn to_doc(&self, state: &'a crate::typst_printer::State<'a>) -> DocBuilder<'a, Arena<'a>, ()> {
        if self.items.is_empty() {
            return state.arena.nil();
        }
        let mut doc = state.arena.nil();
        if state.render_with_hash {
            doc = doc.append(state.arena.text("#"));
        }
        let prefix = match self.kind {
            ListKind::Ordered(_) => "enum(\n  [",
            ListKind::Bullet(_) => "list(\n  [",
        };
        doc = doc.append(prefix);

        let list_content = state.arena.intersperse(
            self.items.iter().map(|item| item.to_doc(self, state)),
            state.arena.text("],\n  ["),
        );

        doc.append(list_content).append(state.arena.text("],\n)"))
    }
}

impl ListItem {
    fn to_doc<'a>(
        &self,
        _list: &List,
        state: &'a crate::typst_printer::State<'a>,
    ) -> DocBuilder<'a, Arena<'a>, ()> {
        // 处理 blocks，如果是段落则只渲染子节点
        let item_content = state.arena.intersperse(
            self.blocks.iter().map(|block| {
                // 如果是段落，只渲染段落的内联子节点
                if let Block::Paragraph(inlines) = block {
                    state.arena.intersperse(
                        inlines.iter().map(|inline| inline.to_doc(state)),
                        state.arena.nil(),
                    )
                } else {
                    // 非段落直接渲染
                    block.to_doc(state)
                }
            }),
            state.arena.line(), // 块级元素之间的分隔符
        );

        // 处理任务列表
        if let Some(task_state) = self.task {
            let checkbox = match task_state {
                TaskState::Complete => "[#sym.checked] ",
                TaskState::Incomplete => "[#sym.checkbox] ",
            };
            state.arena.text(checkbox).append(item_content)
        } else {
            item_content
        }
    }
}
