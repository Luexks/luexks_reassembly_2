use crate::blocks::feature::explosive::ExplosiveFields;
use crate::blocks::feature::fragment::Fragment;
use crate::blocks::pattern::Pattern;
use crate::utility::color::Color;
use crate::utility::component_formatting::{
    format_bracket_layer, format_component_option, format_component_options,
};
use crate::utility::flags::Flags;
use std::fmt::{self, Display};

#[derive(Clone, Debug)]
pub struct CannonFields {
    pub cannon: Option<Cannon>,
}

impl Default for CannonFields {
    fn default() -> Self {
        Self { cannon: None }
    }
}

impl Display for CannonFields {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            format_component_option!(
                &self.cannon => "cannon",
            ),
        )
    }
}

#[derive(Clone, Debug)]
pub struct Cannon {
    pub rounds_per_sec: Option<f32>,
    pub rounds_per_burst: Option<u8>,
    pub explosive: Option<Flags<ExplosiveFields>>,
    pub pattern: Option<Flags<Pattern>>,
    pub burstyness: Option<f32>,
    pub muzzle_vel: Option<f32>,
    pub spread: Option<f32>,
    pub range_std_dev: Option<f32>,
    pub power: Option<f32>,
    pub damage: Option<f32>,
    pub range: Option<f32>,
    pub explode_radius: Option<f32>,
    pub color: Option<Color>,
    pub projectile_size: Option<f32>,
    pub fragment: Option<Fragment>,
}

impl Default for Cannon {
    fn default() -> Self {
        Self {
            rounds_per_sec: None,
            rounds_per_burst: None,
            explosive: None,
            pattern: None,
            burstyness: None,
            muzzle_vel: None,
            spread: None,
            range_std_dev: None,
            power: None,
            damage: None,
            range: None,
            explode_radius: None,
            color: None,
            projectile_size: None,
            fragment: None,
        }
    }
}

impl Display for Cannon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            format_bracket_layer(format_component_options!(
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
            )),
        )
    }
}

#[macro_export]
macro_rules! cannon {
    () => {
        Cannon::default()
    };
    {$($component_name:ident: $component_value:expr),* $(,)?} => {
        {
            let mut cannon = Cannon::default();
            $(
                cannon.$component_name = Some($component_value);
            )*
            cannon
        }
    };
}
