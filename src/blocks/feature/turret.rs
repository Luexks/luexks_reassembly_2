use crate::blocks::feature::fragment::Fragment;
use crate::blocks::pattern::Pattern;
use crate::utility::color::Color;
use crate::utility::component_formatting::format_component_options;
use crate::utility::display_oriented_math::DisplayOriented2D;
use crate::utility::flags::Flags;
use crate::{blocks::feature::explosive::Explosive, utility::angle::Angle};
use std::fmt::{self, Display};

#[derive(Clone)]
pub struct Turret {
    pub speed: Option<Angle>,
    pub limit: Option<Angle>,
    pub barrel_size: Option<DisplayOriented2D>,
    pub barrel_count: Option<i32>,
    pub barrel_taper: Option<f32>,
}

impl Default for Turret {
    fn default() -> Self {
        Self {
            speed: None,
            limit: None,
            barrel_size: None,
            barrel_count: None,
            barrel_taper: None,
        }
    }
}

impl Display for Turret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            format_component_options!(
                &self.speed => "turretSpeed",
                &self.limit => "turretLimit",
                &self.barrel_size => "barrelSize",
                self.barrel_count => "barrelCount",
                self.barrel_taper => "barrelTaper"
            )
        )
    }
}
