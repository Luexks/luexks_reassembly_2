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
