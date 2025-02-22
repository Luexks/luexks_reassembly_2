use crate::utility::component_formatting::format_component_option;
use std::fmt::{self, Display};

#[derive(Clone)]
pub struct Torquer {
    torque: Option<f32>,
}

impl Default for Torquer {
    fn default() -> Self {
        Self { torque: None }
    }
}

impl Display for Torquer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            format_component_option!(
                self.torque => "torquerTorque",
            )
        )
    }
}
