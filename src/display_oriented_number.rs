use crate::utils::*;
use std::fmt::{self, Display};

#[derive(Clone)]
pub enum DisplayOrientedNumber {
    Float(f32),
    Fraction {
        numerator: Box<DisplayOrientedNumber>,
        denominator: Box<DisplayOrientedNumber>,
    },
}

impl DisplayOrientedNumber {
    pub fn to_f32(&self) -> f32 {
        match self {
            DisplayOrientedNumber::Float(value) => *value,
            DisplayOrientedNumber::Fraction {
                numerator,
                denominator,
            } => numerator.to_f32() / denominator.to_f32(),
        }
    }
}

impl Display for DisplayOrientedNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DisplayOrientedNumber::Float(value) => format!("{}", value),
                DisplayOrientedNumber::Fraction {
                    numerator,
                    denominator,
                } => format!("{}/{}", numerator.to_f32(), denominator.to_f32()),
            }
        )
    }
}

#[derive(Clone)]
pub struct DisplayOriented2D {
    pub x: DisplayOrientedNumber,
    pub y: DisplayOrientedNumber,
}

impl DisplayOriented2D {
    pub fn orient_by_index(&mut self, index: usize) -> Self {
        DisplayOriented2D {
            x: DisplayOrientedNumber::Float(
                self.x.to_f32() * VERTEX_ORIENTATION_MULTIPLIERS[index].0,
            ),
            y: DisplayOrientedNumber::Float(
                self.y.to_f32() * VERTEX_ORIENTATION_MULTIPLIERS[index].1,
            ),
        }
    }
}

impl Display for DisplayOriented2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{},{}}}", self.x, self.y)
    }
}

#[derive(Clone)]
pub struct DisplayOriented3D {
    pub x: DisplayOrientedNumber,
    pub y: DisplayOrientedNumber,
    pub z: DisplayOrientedNumber,
}

impl DisplayOriented3D {
    pub fn orient_by_index(&mut self, index: usize) -> Self {
        DisplayOriented3D {
            x: DisplayOrientedNumber::Float(
                self.x.to_f32() * VERTEX_ORIENTATION_MULTIPLIERS[index].0,
            ),
            y: DisplayOrientedNumber::Float(
                self.y.to_f32() * VERTEX_ORIENTATION_MULTIPLIERS[index].1,
            ),
            z: self.z.clone(),
        }
    }
}

impl Display for DisplayOriented3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{},{},{}}}", self.x, self.y, self.z)
    }
}
