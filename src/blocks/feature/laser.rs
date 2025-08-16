use crate::blocks::feature::explosive::ExplosiveFields;
use crate::utility::color::Color;
use crate::utility::component_formatting::{
    format_bracket_layer, format_component_option, format_component_options,
};
use crate::utility::flags::Flags;
use std::fmt::{self, Display};

#[derive(Clone, Debug)]
pub struct LaserFields {
    laser: Option<Laser>,
}

impl Default for LaserFields {
    fn default() -> Self {
        LaserFields { laser: None }
    }
}

impl Display for LaserFields {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            format_component_option!(
                &self.laser => "laser"
            )
        )
    }
}

#[derive(Clone, Debug)]
pub struct Laser {
    pulses_per_sec: Option<f32>,
    pulses_per_burst: Option<u8>,
    explosive: Option<Flags<ExplosiveFields>>,
    burstyness: Option<f32>,
    pulse_availability: Option<f32>,
    decay: Option<f32>,
    power: Option<f32>,
    width: Option<f32>,
    damage: Option<f32>,
    color: Option<Color>,
    range: Option<f32>,
    explode_radius: Option<f32>,
    immobilize_force: Option<f32>,
    linear_force: Option<f32>,
}

impl Default for Laser {
    fn default() -> Self {
        Self {
            pulses_per_sec: None,
            pulses_per_burst: None,
            explosive: None,
            burstyness: None,
            pulse_availability: None,
            decay: None,
            power: None,
            width: None,
            damage: None,
            color: None,
            range: None,
            explode_radius: None,
            immobilize_force: None,
            linear_force: None,
        }
    }
}

impl Display for Laser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            format_bracket_layer(format_component_options!(
                self.pulses_per_sec => "pulsesPerSec",
                self.pulses_per_burst => "pulsesPerBurst",
                &self.explosive => "explosive",
                self.burstyness => "burstyness",
                self.pulse_availability => "pulseAvailability",
                self.decay => "decay",
                self.power => "power",
                self.width => "width",
                self.damage => "damage",
                &self.color => "color",
                self.range => "range",
                self.explode_radius => "explodeRadius",
                self.immobilize_force => "immobilizeForce",
                self.linear_force => "linearForce",
            )),
        )
    }
}

#[macro_export]
macro_rules! laser {
    () => {
        Laser::default()
    };
    {$($component_name:ident: $component_value:expr),* $(,)?} => {
        {
            let mut laser = Laser::default();
            $(
                laser.$component_name = Some($component_value);
            )*
            laser
        }
    };
}
