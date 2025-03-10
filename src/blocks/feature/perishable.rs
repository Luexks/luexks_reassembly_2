use crate::utility::component_formatting::format_component_options;
use std::fmt::{self, Display};

#[derive(Clone)]
pub struct Perishable {
    lifetime: Option<f32>,
}

impl Default for Perishable {
    fn default() -> Self {
        Self { lifetime: None }
    }
}

impl Display for Perishable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            format_component_options!(
                self.lifetime => "lifetime",
            )
        )
    }
}
