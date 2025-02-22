use crate::blocks::block::Block;
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
pub struct Launcher {
    replicate_block: Option<Block>,
    speed: Option<f32>,
    power: Option<f32>,
    out_speed: Option<f32>,
    ang_vel: Option<f32>,
}

impl Default for Launcher {
    fn default() -> Self {
        Self {
            replicate_block: None,
            speed: None,
            power: None,
            out_speed: None,
            ang_vel: None,
        }
    }
}

impl Display for Launcher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            format_component_options!(
                &self.replicate_block => "replicateBlock",
                self.speed => "launcherOutSpeed",
                self.power => "launcherPower",
                self.out_speed => "launcherOutSpeed",
                self.ang_vel => "launcherAngVel",
            )
        )
    }
}
