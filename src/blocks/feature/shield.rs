use crate::utility::color::Color;
use crate::utility::component_formatting::{
    format_bracket_layer, format_component, format_component_options,
};
use std::fmt::{self, Display};

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

impl Default for Shield {
    fn default() -> Self {
        Shield {
            strength: None,
            armor: None,
            regen: None,
            delay: None,
            radius: None,
            color: None,
            line_color: None,
            damaged_color: None,
            power: None,
        }
    }
}

impl Display for Shield {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            format_component(
                format_bracket_layer(format_component_options!(
                    self.strength => "strength",
                    self.armor => "armor",
                    self.regen => "regen",
                    self.delay => "delay",
                    self.radius => "radius",
                    &self.color => "color",
                    &self.line_color => "lineColor",
                    &self.damaged_color => "damagedColor",
                    self.power => "power",
                )),
                "shield"
            )
        )
    }
}

// impl Display for Shield {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "{}",
//             format_component(
//                     format!("{{{}}}",
//                         format_component_options(
//                             &[
//                             (self.strength, "strength"),
//                             (self.armor, "armor"),
//                             (self.regen, "regen"),
//                             (self.delay, "delay"),
//                             (self.radius, "radius"),
//                             (self.color, "color"),
//                             (self.line_color, "lineColor"),
//                             (self.damaged_color, "damagedColor"),
//                             (self.power, "power"),
//                             ]
//                         )
//                 ), "shield"
//             )
//         )
//     }
// }
