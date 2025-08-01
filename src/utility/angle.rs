use std::fmt::{self, Display};

#[derive(Clone, Debug)]
pub enum Angle {
    Degree(f32),
    Radian(f32),
}

impl Display for Angle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_radians().get_value())
    }
}

impl Angle {
    pub fn as_radians(&self) -> Angle {
        Angle::Radian(match self {
            // Angle::Degree(value) => value * (std::f32::consts::PI / 90.0),
            Angle::Degree(value) => value * (std::f32::consts::PI / 180.0),
            Angle::Radian(value) => *value,
        })
    }

    pub fn as_degrees(&self) -> Angle {
        Angle::Degree(match self {
            Angle::Degree(value) => *value,
            Angle::Radian(value) => value * (180.0 / std::f32::consts::PI),
        })
    }

    pub fn get_value(&self) -> f32 {
        match self {
            Angle::Degree(value) => *value,
            Angle::Radian(value) => *value,
        }
    }
}
