use crate::mod_configs::mod_metadata::BLOCK_SORT_BASE;
use std::fmt::{self, Display};

static mut CURRENT_BLOCK_SORT: BlockSort = BlockSort(BLOCK_SORT_BASE);

#[derive(Clone, Copy, Debug)]
pub struct BlockSort(i32);

impl Display for BlockSort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl BlockSort {
    pub fn next() -> BlockSort {
        unsafe {
            let buffer_block_sort = CURRENT_BLOCK_SORT;
            CURRENT_BLOCK_SORT.0 += 1;
            buffer_block_sort
        }
    }
}
