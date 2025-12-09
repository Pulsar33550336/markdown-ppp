use crate::ast::*;
use crate::typst_printer::ToDoc;
use pretty::{Arena, DocAllocator, DocBuilder};

impl<'a> ToDoc<'a> for Table {
    fn to_doc(&self, state: &'a crate::typst_printer::State<'a>) -> DocBuilder<'a, Arena<'a>, ()> {
        if self.rows.is_empty() {
            return state.arena.nil();
        }

        let mut content = state.arena.nil();

        // Add table columns specification
        let column_spec = self
            .alignments
            .iter()
            // .map(|align| match align {
            //     Alignment::Left | Alignment::None => "auto",
            //     Alignment::Center => "1fr",
            //     Alignment::Right => "auto",
            // })
            .map(|align| match align {
                Alignment::Left => "left + horizon",
                Alignment::Center | Alignment::None => "center + horizon",
                Alignment::Right => "right + horizon",
            })
            .collect::<Vec<_>>()
            .join(", ");

        let columns = Some(self.alignments.len())
            .filter(|&len| len > 0)
            .unwrap_or_else(|| self.rows.first().map_or(0, |row| row.len()));

        content = content
            .append(
                state
                    .arena
                    .text(format!("#figure(table(\n  columns: ({}),", columns)),
            )
            .append(state.arena.text(format!("\n  align: ({}),", column_spec)));

        // Add all rows
        for row in &self.rows {
            content = content.append(state.arena.hardline());
            for cell in row {
                if cell.removed_by_extended_table {
                    continue;
                }

                let mut cell_parts = Vec::new();
                if let Some(colspan) = cell.colspan {
                    if colspan > 1 {
                        cell_parts.push(format!("colspan: {}", colspan));
                    }
                }
                if let Some(rowspan) = cell.rowspan {
                    if rowspan > 1 {
                        cell_parts.push(format!("rowspan: {}", rowspan));
                    }
                }

                let cell_doc = if cell_parts.is_empty() {
                    state
                        .arena
                        .text("  [")
                        .append(cell.content.to_doc(state).nest(2))
                        .append(state.arena.text("],"))
                } else {
                    state
                        .arena
                        .text(format!("  table.cell({})[", cell_parts.join(", ")))
                        .append(cell.content.to_doc(state).nest(2))
                        .append(state.arena.text("],"))
                };
                content = content.append(cell_doc);
            }
        }
        content = content.append(state.arena.hardline());
        content.append(state.arena.text("))"))
    }
}
