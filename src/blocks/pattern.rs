use std::fmt::{self, Display};

#[derive(Clone)]
pub enum Pattern {
    Random,
    Constant,
    Spiral,
    Absolute,
    Wave,
}

impl Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Pattern::Random => "RANDOM",
                Pattern::Constant => "CONSTANT",
                Pattern::Spiral => "SPIRAL",
                Pattern::Absolute => "ABSOLUTE",
                Pattern::Wave => "WAVE",
            }
        )
    }
}