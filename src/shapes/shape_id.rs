use crate::mod_configs::shape_configs::SHAPE_ID_BASE;
use std::fmt::{self, Display};

static mut CURRENT_SHAPE_ID: ShapeId = ShapeId(SHAPE_ID_BASE);

#[derive(Clone, Copy)]
pub struct ShapeId(u32);

impl Display for ShapeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ShapeId {
    pub fn next() -> ShapeId {
        unsafe {
            let buffer_shape_id = CURRENT_SHAPE_ID;
            CURRENT_SHAPE_ID.0 += 1;
            buffer_shape_id
        }
    }
}
