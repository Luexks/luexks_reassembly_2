use std::fmt::{self, Display};

#[derive(Clone)]
pub enum Explosive {
    Enabled,
    Final,
    Proximity,
    FragImpact,
    FragFinal,
    FragProximity,
    FragNoFlash,
}

impl Display for Explosive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Explosive::Enabled => "ENABLED",
                Explosive::Final => "FINAL",
                Explosive::Proximity => "PROXIMITY",
                Explosive::FragImpact => "FRAG_IMPACT",
                Explosive::FragFinal => "FRAG_FINAL",
                Explosive::FragProximity => "FRAG_PROXIMITY",
                Explosive::FragNoFlash => "FRAG_NOFLASH",
            }
        )
    }
}
