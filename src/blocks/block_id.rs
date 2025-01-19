use crate::mod_configs::mod_metadata::*;
use std::fmt::{self, Display};

static mut CURRENT_BLOCK_ID: BlockId = BLOCK_ID_BASE;

#[derive(Clone, Copy)]
pub struct BlockId(pub u32);

impl Default for BlockId {
    fn default() -> Self {
        BlockId(0)
    }
}

impl Display for BlockId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl BlockId {
    pub fn next() -> BlockId {
        unsafe {
            let buffer_block_id = CURRENT_BLOCK_ID;
            CURRENT_BLOCK_ID.0 += 1;
            buffer_block_id
        }
    }
}
