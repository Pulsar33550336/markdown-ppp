use crate::ast::{Block, Inline};
use crate::ast_transform::Transformer;
use std::rc::Rc;

type BlockExpansionFn = dyn Fn(&str) -> Vec<Block>;
type InlineExpansionFn = dyn Fn(&str) -> Vec<Inline>;

/// A transformer that expands block and inline macros using user-defined functions.
pub struct MacroTransformer {
    /// Function to expand a block-level macro.
    ///
    /// The function takes the macro content (the string inside `{{...}}`) as input
    /// and should return a vector of `Block` nodes to replace the macro.
    pub block_expander: Rc<BlockExpansionFn>,

    /// Function to expand an inline-level macro.
    ///
    /// The function takes the macro content (the string inside `{{...}}`) as input
    /// and should return a vector of `Inline` nodes to replace the macro.
    pub inline_expander: Rc<InlineExpansionFn>,
}

impl Transformer for MacroTransformer {
    fn expand_block(&mut self, block: Block) -> Vec<Block> {
        if let Block::MacroBlock(content) = block {
            (self.block_expander)(&content)
        } else {
            self.walk_expand_block(block)
        }
    }

    fn expand_inline(&mut self, inline: Inline) -> Vec<Inline> {
        if let Inline::Macro(content) = inline {
            (self.inline_expander)(&content)
        } else {
            self.walk_expand_inline(inline)
        }
    }
}
