use crate::utility::component_formatting::format_component_options;
use std::fmt::{self, Display};

#[derive(Clone)]
pub struct Seed {
    lifetime: Option<f32>,
}

impl Default for Seed {
    fn default() -> Self {
        Self { lifetime: None }
    }
}

impl Display for Seed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            format_component_options!(
                self.lifetime => "seedLifetime",
            )
        )
    }
}
