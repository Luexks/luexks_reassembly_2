use crate::mod_configs::shape_configs::shape_configs::SHAPE_ID_BASE;
use std::fmt::{self, Display};

static mut CURRENT_SHAPE_ID: ShapeId = ShapeId::Number(SHAPE_ID_BASE);

#[derive(Clone, Debug)]
pub enum ShapeId {
    Number(u32),
    Vanilla(String),
}

impl Display for ShapeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Number(number) => number.to_string(),
                Self::Vanilla(name) => name.to_string(),
            }
        )
    }
}

impl ShapeId {
    pub fn get_number(&self) -> u32 {
        if let ShapeId::Number(number) = self {
            *number
        } else {
            panic!("ShapeId is not a number.")
        }
    }

    pub fn get_name(&self) -> String {
        if let ShapeId::Vanilla(name) = self {
            name.clone()
        } else {
            panic!("ShapeId is not a number.")
        }
    }
}

impl ShapeId {
    pub fn next() -> ShapeId {
        unsafe {
            let buffer_shape_id = CURRENT_SHAPE_ID.clone();
            CURRENT_SHAPE_ID = ShapeId::Number(CURRENT_SHAPE_ID.get_number() + 1);
            buffer_shape_id
        }
    }
}
