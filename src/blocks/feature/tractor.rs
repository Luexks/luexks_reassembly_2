use crate::utility::component_formatting::format_component_options;
use std::fmt::{self, Display};

#[derive(Clone)]
pub struct Tractor {
    range: Option<f32>,
}

impl Default for Tractor {
    fn default() -> Self {
        Self { range: None }
    }
}

impl Display for Tractor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            format_component_options!(
                self.range => "tractorRange",
            )
        )
    }
}
