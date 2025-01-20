use crate::blocks::explosive::Explosive;
use crate::utility::color::Color;
use crate::utility::component_formatting::format_components;
use crate::utility::flags::Flags;
use std::fmt::{self, Display};

#[derive(Clone)]
pub struct Laser {
    pulses_per_sec: Option<f32>,
    pulses_per_burst: Option<u8>,
    explosive: Option<Flags<Explosive>>,
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

impl Display for Laser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}}}",
            format_components!(
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
                self.linear_force => "linearForce"
            )
        )
    }
}
