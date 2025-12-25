use crate::ast::Block;
use crate::ast_transform::Transformer;
use std::rc::Rc;

type BlockExpansionFn = dyn Fn(&str) -> Vec<Block>;

/// A transformer that expands block macros using a user-defined function.
pub struct MacroTransformer {
    /// Function to expand a block-level macro.
    ///
    /// The function takes the macro content (the string inside `{{...}}`) as input
    /// and should return a vector of `Block` nodes to replace the macro.
    pub block_expander: Rc<BlockExpansionFn>,
}

impl Transformer for MacroTransformer {
    fn expand_block(&mut self, block: Block) -> Vec<Block> {
        if let Block::MacroBlock(content) = block {
            (self.block_expander)(&content)
        } else {
            self.walk_expand_block(block)
        }
    }
}
