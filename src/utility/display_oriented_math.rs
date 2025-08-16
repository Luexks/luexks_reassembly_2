use crate::shapes::shape_constants::*;
use crate::utility::angle::Angle;
use std::fmt::{self, Display};

#[derive(Clone, Debug)]
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

    pub fn get_numerator(&self) -> f32 {
        if let DisplayOrientedNumber::Fraction { numerator, .. } = self {
            numerator.to_f32()
        } else {
            panic!();
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

pub fn don_float_from(x: f32) -> DisplayOrientedNumber {
    DisplayOrientedNumber::Float(x)
}

pub fn don_fraction_from(numerator: f32, denominator: f32) -> DisplayOrientedNumber {
    DisplayOrientedNumber::Fraction {
        numerator: Box::new(don_float_from(numerator)),
        denominator: Box::new(don_float_from(denominator)),
    }
}

#[derive(Clone, Debug)]
pub struct DisplayOriented2D {
    pub x: DisplayOrientedNumber,
    pub y: DisplayOrientedNumber,
}

impl DisplayOriented2D {
    pub fn orient_by_vert_index(&self, vert_index: usize) -> Self {
        DisplayOriented2D {
            x: DisplayOrientedNumber::Float(
                self.x.to_f32().abs() * VERTEX_ORIENTATION_MULTIPLIERS[vert_index].0,
            ),
            y: DisplayOrientedNumber::Float(
                self.y.to_f32().abs() * VERTEX_ORIENTATION_MULTIPLIERS[vert_index].1,
            ),
        }
    }

    pub fn rotate_by_vert_index(&self, vert_index: usize) -> Self {
        if vert_index % 2 == 0 {
            DisplayOriented2D {
                x: DisplayOrientedNumber::Float(
                    self.y.to_f32() * VERTEX_ORIENTATION_MULTIPLIERS[vert_index].1,
                ),
                y: DisplayOrientedNumber::Float(
                    self.x.to_f32() * VERTEX_ORIENTATION_MULTIPLIERS[vert_index].0,
                ),
            }
        } else {
            DisplayOriented2D {
                x: DisplayOrientedNumber::Float(
                    self.x.to_f32() * VERTEX_ORIENTATION_MULTIPLIERS[vert_index].0,
                ),
                y: DisplayOrientedNumber::Float(
                    self.y.to_f32() * VERTEX_ORIENTATION_MULTIPLIERS[vert_index].1,
                ),
            }
        }
    }
    pub fn simple(x: f32, y: f32) -> DisplayOriented2D {
        DisplayOriented2D {
            x: DisplayOrientedNumber::Float(x),
            y: DisplayOrientedNumber::Float(y),
        }
    }

    pub fn translation_by_angle_and_distance(
        self,
        angle: Angle,
        distance: f32,
    ) -> DisplayOriented2D {
        DisplayOriented2D {
            x: don_float_from(self.x.to_f32() + distance * angle.as_radians().get_value().cos()),
            y: don_float_from(self.y.to_f32() + distance * angle.as_radians().get_value().sin()),
        }
    }

    pub fn diagonal_orient_by_vert_index(&self, vert_index: usize) -> Self {
        DisplayOriented2D {
            x: DisplayOrientedNumber::Float(
                self.x.to_f32().abs() * VERTEX_DIAGONAL_ORIENTATION_MULTIPLIERS[vert_index].0,
            ),
            y: DisplayOrientedNumber::Float(
                self.y.to_f32().abs() * VERTEX_DIAGONAL_ORIENTATION_MULTIPLIERS[vert_index].1,
            ),
        }
    }
}

impl Display for DisplayOriented2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{},{}}}", self.x, self.y)
    }
}

pub fn do2d_float_from(x: f32, y: f32) -> DisplayOriented2D {
    DisplayOriented2D {
        x: DisplayOrientedNumber::Float(x),
        y: DisplayOrientedNumber::Float(y),
    }
}

#[derive(Clone, Debug)]
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

pub fn distance_and_angle_to_do2d(distance: f32, angle: Angle) -> DisplayOriented2D {
    let angle = angle.as_radians().get_value();
    do2d_float_from(distance * angle.cos(), distance * angle.sin())
}

pub fn do3d_float_from(x: f32, y: f32, z: f32) -> DisplayOriented3D {
    DisplayOriented3D {
        x: DisplayOrientedNumber::Float(x),
        y: DisplayOrientedNumber::Float(y),
        z: DisplayOrientedNumber::Float(z),
    }
}