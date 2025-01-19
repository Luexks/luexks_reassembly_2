use std::fmt::{self, Display};
use crate::utility::color::Color;
use crate::utility::component_formatting::format_components;

#[derive(Clone)]
pub struct Shield {
    strength: Option<f32>,
    armor: Option<f32>,
    regen: Option<f32>,
    delay: Option<f32>,
    radius: Option<f32>,
    color: Option<Color>,
    line_color: Option<Color>,
    damaged_color: Option<Color>,
    power: Option<f32>,
}

impl Display for Shield {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}}}",
            format_components!(
                self.strength => "strength",
                self.armor => "armor",
                self.regen => "regen",
                self.delay => "delay",
                self.radius => "radius",
                &self.color => "color",
                &self.line_color => "lineColor",
                &self.damaged_color => "damagedColor",
                self.power => "power"
            )
        )
    }
}