use std::fmt::{self, Display};

#[derive(Clone)]
pub enum ExplosiveFields {
    Enabled,
    Final,
    Proximity,
    FragImpact,
    FragFinal,
    FragProximity,
    FragNoFlash,
}

impl Display for ExplosiveFields {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ExplosiveFields::Enabled => "ENABLED",
                ExplosiveFields::Final => "FINAL",
                ExplosiveFields::Proximity => "PROXIMITY",
                ExplosiveFields::FragImpact => "FRAG_IMPACT",
                ExplosiveFields::FragFinal => "FRAG_FINAL",
                ExplosiveFields::FragProximity => "FRAG_PROXIMITY",
                ExplosiveFields::FragNoFlash => "FRAG_NOFLASH",
            }
        )
    }
}
