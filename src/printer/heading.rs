use crate::ast::*;
use crate::printer::{inline::ToDocInline, ToDoc};
use pretty::{Arena, DocAllocator, DocBuilder};
use std::rc::Rc;

impl<'a> ToDoc<'a> for Heading {
    fn to_doc(
        &self,
        config: Rc<crate::printer::config::Config>,
        arena: &'a Arena<'a>,
    ) -> DocBuilder<'a, Arena<'a>, ()> {
        match self.kind {
            HeadingKind::Atx(level) => {
                let hashes = "#".repeat(level as usize);
                arena
                    .text(hashes)
                    .append(arena.space())
                    .append(self.content.to_doc_inline(false, arena, config.clone()))
            }
            HeadingKind::Setext(SetextHeading::Level1) => self
                .content
                .to_doc_inline(true, arena, config.clone())
                .append(arena.hardline())
                .append(arena.text("==========")),
            HeadingKind::Setext(SetextHeading::Level2) => self
                .content
                .to_doc_inline(true, arena, config.clone())
                .append(arena.hardline())
                .append(arena.text("----------")),
        }
    }
}
