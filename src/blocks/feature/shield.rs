use crate::utility::color::Color;
use crate::utility::component_formatting::{
    format_bracket_layer, format_component_option, format_component_options,
};
use std::fmt::{self, Display};

#[derive(Clone)]
pub struct ShieldFields {
    shield: Option<Shield>,
}

impl Default for ShieldFields {
    fn default() -> Self {
        Self { shield: None }
    }
}

impl Display for ShieldFields {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format_component_option!(&self.shield => "shield"))
    }
}

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
        )
    }
}

#[macro_export]
macro_rules! shield {
    () => {
        Shield::default()
    };
    {$($component_name:ident: $component_value:expr),* $(,)?} => {
        {
            let mut shield = Shield::default();
            $(
                shield.$component_name = Some($component_value);
            )*
            shield
        }
    };
}
