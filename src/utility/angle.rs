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

    pub fn get_value_mut(&mut self) -> &mut f32 {
        match self {
            Angle::Degree(value) => value,
            Angle::Radian(value) => value,
        }
    }

    pub fn as_radians_mut(&mut self) -> &mut Angle {
        *self = self.as_radians();
        self
    }

    pub fn as_degrees_mut(&mut self) -> &mut Angle {
        *self = self.as_degrees();
        self
    }
}
