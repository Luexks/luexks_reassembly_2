use crate::utility::component_formatting::format_component_options;
use std::fmt::{self, Display};

#[derive(Clone)]
pub struct MeleeFields {
    damage: Option<f32>,
}

impl Default for MeleeFields {
    fn default() -> Self {
        Self { damage: None }
    }
}

impl Display for MeleeFields {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            format_component_options!(
                self.damage => "meleeDamage",
            )
        )
    }
}
