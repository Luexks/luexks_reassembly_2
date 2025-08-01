use crate::utility::component_formatting::format_component_option;
use std::fmt::{self, Display};

#[derive(Clone)]
pub struct TorquerFields {
    torque: Option<f32>,
}

impl Default for TorquerFields {
    fn default() -> Self {
        Self { torque: None }
    }
}

impl Display for TorquerFields {
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
