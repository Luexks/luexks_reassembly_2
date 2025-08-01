use crate::utility::component_formatting::format_component_options;
use std::fmt::{self, Display};

#[derive(Clone)]
pub struct GeneratorFields {
    pub capacity: Option<f32>,
    pub capacity_per_sec: Option<f32>,
}

impl Default for GeneratorFields {
    fn default() -> Self {
        Self {
            capacity: None,
            capacity_per_sec: None,
        }
    }
}

impl Display for GeneratorFields {
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
