use std::fmt::{self, Display};
use crate::utility::flags::Flags;
use crate::utility::color::Color;
use crate::blocks::explosive::Explosive;
use crate::blocks::pattern::Pattern;
use crate::utility::component_formatting::format_components;

#[derive(Clone)]
pub struct Fragment {
    rounds_per_burst: Option<u8>,
    explosive: Option<Flags<Explosive>>,
    pattern: Option<Flags<Pattern>>,
    muzzle_vel: Option<f32>,
    spread: Option<f32>,
    range_std_dev: Option<f32>,
    damage: Option<f32>,
    range: Option<f32>,
    explode_radius: Option<f32>,
    color: Option<Color>,
    projectile_size: Option<f32>,
    fragment: Option<Box<Fragment>>,
}

impl Display for Fragment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}}}",
            format_components!(
                self.rounds_per_burst => "roundsPerBurst",
                &self.explosive => "explosive",
                &self.pattern => "pattern",
                self.muzzle_vel => "muzzleVel",
                self.spread => "spread",
                self.range_std_dev => "rangeStdDev",
                self.damage => "damage",
                self.range => "range",
                self.explode_radius => "explodeRadius",
                &self.color => "color",
                self.projectile_size => "projectileSize"
            )
        )
    }
}