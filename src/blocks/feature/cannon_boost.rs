use crate::utility::component_formatting::{
    format_bracket_layer, format_component_option, format_component_options,
};
use std::fmt::{self, Display};

#[derive(Clone)]
pub struct CannonBoostFields {
    pub cannon_boost: Option<CannonBoost>,
}

impl Default for CannonBoostFields {
    fn default() -> Self {
        CannonBoostFields { cannon_boost: None }
    }
}

impl Display for CannonBoostFields {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            format_component_option!(&self.cannon_boost => "cannonBoost",
            ),
        )
    }
}

#[derive(Clone)]
pub struct CannonBoost {
    rounds_per_sec: Option<CannonBoostValue>,
    muzzle_vel: Option<CannonBoostValue>,
    power: Option<CannonBoostValue>,
    damage: Option<CannonBoostValue>,
    range: Option<CannonBoostValue>,
    explode_radius: Option<CannonBoostValue>,
    spread: Option<CannonBoostValue>,
}

impl Default for CannonBoost {
    fn default() -> Self {
        CannonBoost {
            rounds_per_sec: None,
            muzzle_vel: None,
            power: None,
            damage: None,
            range: None,
            explode_radius: None,
            spread: None,
        }
    }
}

impl Display for CannonBoost {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            format_bracket_layer(format_component_options!(
                &self.rounds_per_sec => "roundsPerSec",
                &self.muzzle_vel => "muzzleVel",
                &self.power => "power",
                &self.damage => "damage",
                &self.range => "range",
                &self.explode_radius => "explodeRadius",
                &self.spread => "spread"
            )),
        )
    }
}

#[macro_export]
macro_rules! cannon_boost {
    () => {
        CannonBoost::default()
    };
    {$($component_name:ident: $component_value:expr),* $(,)?} => {
        {
            let mut cannon_boost = CannonBoost::default();
            $(
                cannon_boost.$component_name = Some($component_value);
            )*
            cannon_boost
        }
    };
}

#[derive(Clone)]
pub struct CannonBoostValue {
    multiplier: f32,
    flat: f32,
}

impl Display for CannonBoostValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{},{}}}", self.multiplier, self.flat)
    }
}
