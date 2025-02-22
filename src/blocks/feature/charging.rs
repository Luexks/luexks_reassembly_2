use crate::utility::component_formatting::format_component_options;
use std::fmt::{self, Display};

#[derive(Clone)]
pub struct Charging {
    max_seconds: Option<f32>,
    min_fraction: Option<f32>,
}

impl Default for Charging {
    fn default() -> Self {
        Self {
            max_seconds: None,
            min_fraction: None,
        }
    }
}

impl Display for Charging {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            format_component_options!(
                self.max_seconds => "chargingMaxTime",
                self.min_fraction => "chargingMin",
            )
        )
    }
}
