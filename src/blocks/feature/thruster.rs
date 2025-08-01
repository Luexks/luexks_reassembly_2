use crate::utility::color::Color;
use crate::utility::component_formatting::format_component_options;
use std::fmt::{self, Display};

#[derive(Clone)]
pub struct ThrusterFields {
    force: Option<f32>,
    boost: Option<f32>,
    boost_time: Option<f32>,
    color_1: Option<Color>,
    color_2: Option<Color>,
}

impl Default for ThrusterFields {
    fn default() -> Self {
        Self {
            force: None,
            boost: None,
            boost_time: None,
            color_1: None,
            color_2: None,
        }
    }
}

impl Display for ThrusterFields {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            format_component_options!(
                self.force => "thrusterForce",
                self.boost => "thrusterBoost",
                self.boost_time => "thrusterBoostTime",
                &self.color_1 => "thrusterColor",
                &self.color_2 => "thrusterColor1"
            )
        )
    }
}
