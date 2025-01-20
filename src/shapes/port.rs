use crate::shapes::port_flags::*;
use crate::shapes::shape_constants::PortPosition;
use crate::utility::display_oriented_math::DisplayOrientedNumber;
use crate::utility::flags::*;
use std::fmt::{self, Display};

#[derive(Clone, Debug)]
pub struct Port {
    pub side_index: usize,
    pub position: DisplayOrientedNumber,
    pub flags: Flags<PortFlag>,
}

impl Display for Port {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{},{}{}}}", self.side_index, self.position, self.flags)
    }
}

impl Port {
    pub fn has_valid_position(&self) -> bool {
        is_port_position_valid(&self.position)
    }
}

pub fn is_port_position_valid(position: &DisplayOrientedNumber) -> bool {
    0.0 <= position.to_f32() && position.to_f32() <= 1.0
}

pub fn halfway_port(side_index: usize) -> Port {
    Port {
        side_index: side_index,
        position: DisplayOrientedNumber::Float(PortPosition::CENTER),
        flags: Flags::<PortFlag>::default(),
    }
}
