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

#[macro_export]
macro_rules! no_port_flags {
    () => {
        Flags::default()
    };
}
pub(crate) use no_port_flags;

impl PortFlag {
    pub fn from_str(s: &str) -> PortFlag {
        match s {
            "THRUSTER_OUT" => PortFlag::ThrusterOut,
            "THRUSTER_IN" => PortFlag::ThrusterIn,
            "WEAPON_OUT" => PortFlag::WeaponOut,
            "WEAPON_IN" => PortFlag::WeaponIn,
            "LAUNCHER" => PortFlag::Launcher,
            "MISSILE" => PortFlag::Missile,
            "ROOT" => PortFlag::Root,
            "NONE" => PortFlag::None,
            "NORMAL" => PortFlag::Normal,
            _ => panic!(),
        }
    }
}
