use crate::utility::component_formatting::format_component_options;
use std::fmt::{self, Display};

#[derive(Clone)]
pub struct SeedFields {
    seed_lifetime: Option<f32>,
    launch_lifetime: Option<f32>,
}

impl Default for SeedFields {
    fn default() -> Self {
        Self {
            seed_lifetime: None,
            launch_lifetime: None,
        }
    }
}

impl Display for SeedFields {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            format_component_options!(
                self.seed_lifetime => "seedLifetime",
                self.launch_lifetime => "launchLifetime",
            )
        )
    }
}
