use std::fmt::{self, Display};
use crate::blocks::block::Block;

pub struct Blocks(pub Vec<Block>);

impl Default for Blocks {
    fn default() -> Self {
        Blocks(Vec::new())
    }
}

impl Display for Blocks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{\n{}\n}}",
            self.0
                .iter()
                .map(|block| block.to_string())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

impl Blocks {
    pub fn add_blocks(&mut self, blocks: Vec<Block>) {
        self.0.extend(blocks);
    }

    pub fn extend_first_block(&self, block: Block) -> Block {
        self.0.first().unwrap().extend(block)
    }
}