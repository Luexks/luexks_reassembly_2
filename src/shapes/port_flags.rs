use std::fmt::{self, Display};

#[derive(Clone, Debug)]
pub enum PortFlag {
    ThrusterOut,
    ThrusterIn,
    WeaponOut,
    WeaponIn,
    Launcher,
    Missile,
    Root,
    None,
    Normal,
}

impl Display for PortFlag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PortFlag::ThrusterOut => "THRUSTER_OUT",
                PortFlag::ThrusterIn => "THRUSTER_IN",
                PortFlag::WeaponOut => "WEAPON_OUT",
                PortFlag::WeaponIn => "WEAPON_IN",
                PortFlag::Launcher => "LAUNCHER",
                PortFlag::Missile => "MISSILE",
                PortFlag::Root => "ROOT",
                PortFlag::None => "NONE",
                PortFlag::Normal => "NORMAL",
            }
        )
    }
}
