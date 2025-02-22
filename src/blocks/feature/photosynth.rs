use crate::utility::component_formatting::format_component_options;
use std::fmt::{self, Display};

#[derive(Clone)]
pub struct Photosynth {
    per_sec: Option<f32>,
}

impl Default for Photosynth {
    fn default() -> Self {
        Self { per_sec: None }
    }
}

impl Display for Photosynth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            format_component_options!(
                self.per_sec => "photosynthPerSec",
            )
        )
    }
}
