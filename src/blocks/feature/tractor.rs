use crate::utility::component_formatting::format_component_options;
use std::fmt::{self, Display};

#[derive(Clone, Debug)]
pub struct TractorFields {
    range: Option<f32>,
}

impl Default for TractorFields {
    fn default() -> Self {
        Self { range: None }
    }
}

impl Display for TractorFields {
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
