use crate::blocks::block::Block;
use crate::utility::component_formatting::format_component_options;
use std::fmt::{self, Display};

#[derive(Clone)]
pub struct LauncherFields {
    replicate_block: Option<Block>,
    speed: Option<f32>,
    power: Option<f32>,
    out_speed: Option<f32>,
    ang_vel: Option<f32>,
}

impl Default for LauncherFields {
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

impl Display for LauncherFields {
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
