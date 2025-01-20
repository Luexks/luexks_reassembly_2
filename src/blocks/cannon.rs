use crate::blocks::explosive::Explosive;
use crate::blocks::fragment::Fragment;
use crate::blocks::pattern::Pattern;
use crate::utility::color::Color;
use crate::utility::component_formatting::format_components;
use crate::utility::flags::Flags;
use std::fmt::{self, Display};

#[derive(Clone)]
pub struct Cannon {
    rounds_per_sec: Option<f32>,
    rounds_per_burst: Option<u8>,
    explosive: Option<Flags<Explosive>>,
    pattern: Option<Flags<Pattern>>,
    burstyness: Option<f32>,
    muzzle_vel: Option<f32>,
    spread: Option<f32>,
    range_std_dev: Option<f32>,
    power: Option<f32>,
    damage: Option<f32>,
    range: Option<f32>,
    explode_radius: Option<f32>,
    color: Option<Color>,
    projectile_size: Option<f32>,
    fragment: Option<Fragment>,
}

impl Display for Cannon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}}}",
            format_components!(
                self.rounds_per_sec => "roundsPerSec",
                self.rounds_per_burst => "roundsPerBurst",
                &self.explosive => "explosive",
                &self.pattern => "pattern",
                self.burstyness => "burstyness",
                self.muzzle_vel => "muzzleVel",
                self.spread => "spread",
                self.range_std_dev => "rangeStdDev",
                self.power => "power",
                self.damage => "damage",
                self.range => "range",
                self.explode_radius => "explodeRadius",
                &self.color => "color",
                self.projectile_size => "projectileSize",
                &self.fragment => "fragment"
            )
        )
    }
}
