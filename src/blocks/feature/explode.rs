use crate::utility::component_formatting::format_component_options;
use std::fmt::{self, Display};

#[derive(Clone)]
pub struct Explode {
    explode_damage: Option<f32>,
    explode_radius: Option<f32>,
    explode_std_dev: Option<f32>,
    explode_faction: Option<i32>,
}

impl Default for Explode {
    fn default() -> Self {
        Self {
            explode_damage: None,
            explode_radius: None,
            explode_std_dev: None,
            explode_faction: None,
        }
    }
}

impl Display for Explode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            format_component_options!(
                self.explode_damage => "explodeDamage",
                self.explode_radius => "explodeRadius",
                self.explode_std_dev => "explodeStdDev",
                self.explode_faction => "explodeFaction",
            ),
        )
    }
}
