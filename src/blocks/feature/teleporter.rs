use crate::utility::component_formatting::format_component_options;
use std::fmt::{self, Display};

#[derive(Clone)]
pub struct Teleporter {
    power: Option<f32>,
    radius: Option<f32>,
}

impl Default for Teleporter {
    fn default() -> Self {
        Self {
            power: None,
            radius: None,
        }
    }
}

impl Display for Teleporter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            format_component_options!(
                self.power => "teleporterPower",
                self.radius => "teleporterRadius",
            )
        )
    }
}
