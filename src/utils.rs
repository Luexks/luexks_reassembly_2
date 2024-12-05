use crate::configs::*;

pub const BLOCKS_NAME: &str = "blocks.lua";
pub const SHAPES_NAME: &str = "shapes.lua";

pub struct PortPosition;

impl PortPosition {
    pub const CURRENT_VERT: f32 = 0.0;
    pub const NEXT_VERT: f32 = 1.0;
    pub const CENTER: f32 = 0.5;
}

pub const PORT_COUNT_DECISION_TOLERANCE: f32 = 0.001;

pub const VERTEX_ORIENTATION_MULTIPLIERS: [(f32, f32); 4] =
    [(-1.0, -1.0), (-1.0, 1.0), (1.0, 1.0), (1.0, -1.0)];

pub static mut CURRENT_BLOCK_ID: u32 = BLOCK_ID_BASE;

fn get_next_block_id() -> u32 {
    unsafe {
        let buffer_block_id = CURRENT_BLOCK_ID;
        CURRENT_BLOCK_ID += 1;
        buffer_block_id
    }
}

pub static mut CURRENT_BLOCK_SORT: i32 = BLOCK_SORT_BASE;

fn get_next_block_sort() -> i32 {
    unsafe {
        let buffer_block_sort = CURRENT_BLOCK_SORT;
        CURRENT_BLOCK_SORT += 1;
        buffer_block_sort
    }
}
