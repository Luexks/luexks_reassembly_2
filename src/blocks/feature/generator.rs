use crate::blocks::feature::explosive::Explosive;
use crate::blocks::feature::fragment::Fragment;
use crate::blocks::pattern::Pattern;
use crate::utility::color::Color;
use crate::utility::component_formatting::format_component_options;
use crate::utility::flags::Flags;
use std::fmt::{self, Display};

#[derive(Clone)]
pub struct Generator {
    pub capacity: Option<f32>,
    pub capacity_per_sec: Option<f32>,
}

impl Default for Generator {
    fn default() -> Self {
        Self {
            capacity: None,
            capacity_per_sec: None,
        }
    }
}

impl Display for Generator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            format_component_options!(
                self.capacity => "powerCapacity",
                self.capacity_per_sec => "generatorCapacityPerSec",
            )
        )
    }
}
