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
            Block::Paragraph(inlines) => inlines.to_doc(state),

            Block::Heading(heading) => {
                let level = match heading.kind {
                    HeadingKind::Atx(level) => level,
                    HeadingKind::Setext(SetextHeading::Level1) => 1,
                    HeadingKind::Setext(SetextHeading::Level2) => 2,
                };
                state
                    .arena
                    .text("=".repeat(level as usize))
                    .append(state.arena.space())
                    .append(heading.content.to_doc(state))
            }

            Block::ThematicBreak => state.arena.text("---"),

            Block::BlockQuote(blocks) => state
                .arena
                .text("> ")
                .append(blocks.to_doc(state).nest(2)),

            Block::List(list) => list.to_doc(state),

            Block::CodeBlock(code_block) => {
                let lang = match &code_block.kind {
                    CodeBlockKind::Fenced { info: Some(lang) } => lang.clone(),
                    _ => String::new(),
                };
                state
                    .arena
                    .text("```")
                    .append(state.arena.text(lang))
                    .append(state.arena.hardline())
                    .append(state.arena.text(code_block.literal.clone()))
                    .append(state.arena.hardline())
                    .append(state.arena.text("```"))
            }

            Block::HtmlBlock(html) => body(
                &state.arena,
                "raw",
                None,
                vec![state.arena.text(escape_typst(html))],
            ),

            Block::Definition(_) => state.arena.nil(),

            Block::Table(table) => table.to_doc(state),

            Block::FootnoteDefinition(_) => {
                state.arena.nil()
            }

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
                    .append(state.arena.text("*], ["))
                    .append(alert.blocks.to_doc(state))
                    .append(state.arena.text("]))"))
            }

            Block::Empty => state.arena.nil(),
            Block::LatexBlock(latex) => state
                .arena
                .text("$")
                .append(state.arena.text(latex.clone()))
                .append(state.arena.text("$")),
        }
    }
}

impl<'a> ToDoc<'a> for List {
    fn to_doc(&self, state: &'a crate::typst_printer::State<'a>) -> DocBuilder<'a, Arena<'a>, ()> {
        state
            .arena
            .intersperse(
                self.items.iter().map(|item| item.to_doc(self, state)),
                state.arena.hardline(),
            )
            .group()
    }
}

impl ListItem {
    fn to_doc<'a>(
        &self,
        list: &List,
        state: &'a crate::typst_printer::State<'a>,
    ) -> DocBuilder<'a, Arena<'a>, ()> {
        let marker = match list.kind {
            ListKind::Ordered(_) => "+",
            ListKind::Bullet(_) => "-",
        };

        let mut item_content = state.arena.text(marker).append(state.arena.space());

        if let Some(task_state) = self.task {
            let checkbox = match task_state {
                TaskState::Complete => "[#sym.checked]",
                TaskState::Incomplete => "[#sym.checkbox]",
            };
            item_content = item_content
                .append(state.arena.text(checkbox))
                .append(state.arena.space());
        }

        item_content.append(self.blocks.to_doc(state)).nest(2)
    }
}