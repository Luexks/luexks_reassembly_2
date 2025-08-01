use crate::utility::component_formatting::format_component_options;
use std::fmt::{self, Display};

#[derive(Clone)]
pub struct ChargingFields {
    max_seconds: Option<f32>,
    min_fraction: Option<f32>,
}

impl Default for ChargingFields {
    fn default() -> Self {
        Self {
            max_seconds: None,
            min_fraction: None,
        }
    }
}

impl Display for ChargingFields {
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
