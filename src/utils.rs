use crate::configs::*;
use std::fmt::{self, Display};

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

static mut CURRENT_BLOCK_ID: BlockId = BlockId(BLOCK_ID_BASE);

#[derive(Clone, Copy)]
pub struct BlockId(u32);

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

static mut CURRENT_BLOCK_SORT: BlockSort = BlockSort(BLOCK_SORT_BASE);
#[derive(Clone, Copy)]

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

#[derive(Clone)]
pub struct Flags<T: Display>(pub Vec<T>);

impl<T: Display> Default for Flags<T> {
    fn default() -> Self {
        Flags(Vec::new())
    }
}

impl<T: Display> Display for Flags<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|flag| flag.to_string())
                .collect::<Vec<_>>()
                .join("|")
        )
    }
}

#[derive(Clone)]
pub enum Angle {
    Degree(f32),
    Radian(f32),
}

impl Display for Angle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Angle {
    fn as_radians(&self) -> Angle {
        Angle::Radian(match self {
            Angle::Degree(value) => value * (std::f32::consts::PI / 100.0),
            Angle::Radian(value) => *value,
        })
    }
}

#[derive(Clone)]
pub enum Color {
    RRGGBB { rr: u8, gg: u8, bb: u8 },
    AARRGGBB { aa: u8, rr: u8, gg: u8, bb: u8 },
}

impl Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "0x{}",
            match self {
                Color::RRGGBB { rr, gg, bb } => format!("{:02X}{:2X}{:02X}", rr, gg, bb),
                Color::AARRGGBB { aa, rr, gg, bb } =>
                    format!("{:02X}{:02X}{:2X}{:02X}", aa, rr, gg, bb),
            }
        )
    }
}

impl Color {
    pub fn new_rrggbb(rrggbb: &str) -> Color {
        if rrggbb.len() != 6 {
            panic!("Invalid color length for rrggbb");
        }
        Color::RRGGBB {
            rr: u8::from_str_radix(&rrggbb[0..=1], 16)
                .expect(&format!("Invalid hex for RR of {}", rrggbb)),
            gg: u8::from_str_radix(&rrggbb[2..=3], 16)
                .expect(&format!("Invalid hex for GG of {}", rrggbb)),
            bb: u8::from_str_radix(&rrggbb[4..=5], 16)
                .expect(&format!("Invalid hex for BB of {}", rrggbb)),
        }
    }

    pub fn new_aarrggbb(aarrggbb: &str) -> Color {
        if aarrggbb.len() != 8 {
            panic!("Invalid color length for rrggbb");
        }
        Color::AARRGGBB {
            aa: u8::from_str_radix(&aarrggbb[0..=1], 16)
                .expect(&format!("Invalid hex for AA of {}", aarrggbb)),
            rr: u8::from_str_radix(&aarrggbb[2..=3], 16)
                .expect(&format!("Invalid hex for RR of {}", aarrggbb)),
            gg: u8::from_str_radix(&aarrggbb[4..=5], 16)
                .expect(&format!("Invalid hex for GG of {}", aarrggbb)),
            bb: u8::from_str_radix(&aarrggbb[6..=7], 16)
                .expect(&format!("Invalid hex for BB of {}", aarrggbb)),
        }
    }
}

pub const NO_FEATURE_DATA_NEEDED: &str = "";

macro_rules! funky_string {
    ($value:expr) => {
        FunkyString($value.to_string())
    };
}
pub(crate) use funky_string;

#[derive(Clone)]
pub struct FunkyString(pub String);

impl Display for FunkyString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self.0)
    }
}
