use crate::blocks::feature::fragment::Fragment;
use crate::blocks::pattern::Pattern;
use crate::utility::color::Color;
use crate::utility::component_formatting::format_component_options;
use crate::utility::flags::Flags;
use crate::{
    blocks::feature::explosive::Explosive, utility::component_formatting::format_component_option,
};
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
