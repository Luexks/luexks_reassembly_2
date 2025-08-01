use crate::blocks::pattern::Pattern;
use crate::utility::color::Color;
use crate::utility::component_formatting::format_component_options;
use crate::utility::flags::Flags;
use crate::{
    blocks::feature::explosive::ExplosiveFields,
    utility::component_formatting::format_bracket_layer,
};
use std::fmt::{self, Display};

#[derive(Clone)]
pub struct Fragment {
    rounds_per_burst: Option<u8>,
    explosive: Option<Flags<ExplosiveFields>>,
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
            "{}",
            format_bracket_layer(format_component_options!(
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
                self.projectile_size => "projectileSize",
                &self.fragment => "fragment",
            ))
        )
    }
}

#[macro_export]
macro_rules! fragment {
    () => {
        Fragment::default()
    };
    {$($component_name:ident: $component_value:expr),* $(,)?} => {
        {
            let mut fragment = Fragment::default();
            $(
                fragment.$component_name = Some($component_value);
            )*
            fragment
        }
    };
}
