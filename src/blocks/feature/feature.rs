use crate::blocks::feature::*;
use std::fmt::{self, Display};

const NO_FEATURE_DATA_NEEDED: &str = "";

#[macro_export]
macro_rules! implicit_features {
    (
        $($feature:ident $( { $($feature_component_name:ident: $feature_component_value:expr),*})? $(,)?),*
    ) => {
        crate::blocks::extend_accounting_feature_list::ExtendAccountingFeatureList {
            features: Flags(vec![
                $(new_feature!($feature $( {$($feature_component_name: $feature_component_value),*})?)),*,
            ]),
            feature_list_same_as_extends: true,
        }
    };
}
pub(crate) use implicit_features;

#[macro_export]
macro_rules! explicit_features {
    (
        $($feature:ident $({ $($feature_component_name:ident: $feature_component_value:expr),*})? $(,)?),*
    ) => {
        crate::blocks::extend_accounting_feature_list::ExtendAccountingFeatureList {
            features: Flags(vec![
                $(new_feature!($feature $({$($feature_component_name: $feature_component_value),*})?)),*,
            ]),
            feature_list_same_as_extends: false,
        }
    };
}
pub(crate) use explicit_features;

#[macro_export]
macro_rules! new_feature {
    ($feature_name:ident) => {
        Feature::$feature_name
    };
    ($feature_name:ident { $($feature_component_name:ident: $feature_component_value:expr),* }) => {
        Feature::$feature_name(
            // Some(
                {
                    let mut feature_data = default_feature_of_variant!($feature_name);
                    $(
                        feature_data.$feature_component_name = Some($feature_component_value);
                    )*
                    feature_data
                }
            // )
        )
    };
}
pub(crate) use new_feature;

#[derive(Clone, Debug)]
pub enum Feature {
    Command,
    Thruster(ThrusterFields),
    Generator(GeneratorFields),
    Perishable,
    Turret(TurretFields),
    // Launch,
    Cannon(CannonFields),
    Laser(LaserFields),
    Autofire,
    Shield(ShieldFields),
    Torquer(TorquerFields),
    Launcher(LauncherFields),
    Explode(ExplodeFields),
    Assembler,
    Regrower,
    CannonBoost(CannonBoostFields),
    Invulnerable,
    NoRegen,
    Persistent,
    Environmental,
    Tractor(TractorFields),
    Root,
    // Grow,
    Photosynth(PhotosynthFields),
    Autolaunch,
    FreeRes,
    Factory,
    Seed(SeedFields),
    Melee(MeleeFields),
    // Ungrow,
    Unique,
    Charging(ChargingFields),
    // Transient,
    SelfFactory,
    NoClip,
    Invisible,
    Bumper,
    Teleporter(TeleporterFields),
    Deactivates,
    Telespawn,
    NoClipAlly,
    IntLines,
    OneUse,
    NoRecolor,
    NoPalette,
    LauncherBarrage,
    AlwaysFire,
    Palette,
    NeverFire,
    NoIcon,
}

#[macro_export]
macro_rules! default_feature_of_variant {
    // (Command
    (Thruster) => {
        ThrusterFields::default()
    };
    (Generator) => {
        GeneratorFields::default()
    };
    // (Perishable) => {
    //     Perishable::default()
    // };
    (Turret) => {
        TurretFields::default()
    };
    (Cannon) => {
        CannonFields::default()
    };
    (Laser) => {
        LaserFields::default()
    };
    (Shield) => {
        ShieldFields::default()
    };
    (Torquer) => {
        TorquerFields::default()
    };
    (Launcher) => {
        LauncherFields::default()
    };
    (Explode) => {
        ExplodeFields::default()
    };
    (CannonBoost) => {
        CannonBoostFields::default()
    };
    (Tractor) => {
        TractorFields::default()
    };
    (Photosynth) => {
        PhotosynthFields::default()
    };
    (Seed) => {
        SeedFields::default()
    };
    (Melee) => {
        MeleeFields::default()
    };
    (Charging) => {
        ChargingFields::default()
    };
    (Teleporter) => {
        TeleporterFields::default()
    }; // (Autofire
       // (Assembler
       // (Regrower
       // (Invulnerable
       // (NoRegen
       // (Persistent
       // (Environmental
       // (Root
       // (Autolaunch
       // (FreeRes
       // (Factory
       // (Unique
       // (SelfFactory
       // (NoClip
       // (Invisible
       // (Bumper
       // (Deactivates
       // (Telespawn
       // (NoClipAlly
       // (IntLines
       // (OneUse
       // (NoRecolor
       // (NoPalette
       // (LauncherBarrage
       // (AlwaysFire
       // (Palette
       // (NeverFire
       // (NoIcon
}
pub(crate) use default_feature_of_variant;

impl Display for Feature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Feature::Command => "COMMAND",
                Feature::Thruster(_) => "THRUSTER",
                Feature::Generator(_) => "GENERATOR",
                Feature::Perishable => "PERISHABLE",
                Feature::Turret(_) => "TURRET",
                Feature::Cannon(_) => "CANNON",
                Feature::Laser(_) => "LASER",
                Feature::Autofire => "AUTOFIRE",
                Feature::Shield(_) => "SHIELD",
                Feature::Torquer(_) => "TORQUER",
                Feature::Launcher(_) => "LAUNCHER",
                Feature::Explode(_) => "EXPLODE",
                Feature::Assembler => "ASSEMBLER",
                Feature::Regrower => "REGROWER",
                Feature::CannonBoost(_) => "CANNON_BOOST",
                Feature::Invulnerable => "INVULNERABLE",
                Feature::NoRegen => "NOREGEN",
                Feature::Persistent => "PERSISTENT",
                Feature::Environmental => "ENVIRONMENTAL",
                Feature::Tractor(_) => "TRACTOR",
                Feature::Root => "ROOT",
                Feature::Photosynth(_) => "PHOTOSYNTH",
                Feature::Autolaunch => "AUTOLAUNCH",
                Feature::FreeRes => "FREERES",
                Feature::Factory => "FACTORY",
                Feature::Seed(_) => "SEED",
                Feature::Melee(_) => "MELEE",
                Feature::Unique => "UNIQUE",
                Feature::Charging(_) => "CHARGING",
                Feature::SelfFactory => "SELFFACTORY",
                Feature::NoClip => "NOCLIP",
                Feature::Invisible => "INVISIBLE",
                Feature::Bumper => "BUMPER",
                Feature::Teleporter(_) => "TELEPORTER",
                Feature::Deactivates => "DEACTIVATES",
                Feature::Telespawn => "TELESPAWN",
                Feature::NoClipAlly => "NOCLIP_ALLY",
                Feature::IntLines => "INTLINES",
                Feature::OneUse => "ONEUSE",
                Feature::NoRecolor => "NORECOLOR",
                Feature::NoPalette => "NOPALETTE",
                Feature::LauncherBarrage => "LAUNCHER_BARRAGE",
                Feature::AlwaysFire => "ALWAYSFIRE",
                Feature::Palette => "PALETTE",
                Feature::NeverFire => "NEVERFIRE",
                Feature::NoIcon => "NOICON",
            }
        )
    }
}

impl Feature {
    pub fn components_to_string(&self) -> String {
        match self {
            Feature::Command => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Thruster(thruster_fields) => thruster_fields.to_string(),
            Feature::Generator(generator_fields) => generator_fields.to_string(),
            Feature::Perishable => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Turret(turret_fields) => turret_fields.to_string(),
            // Feature::Launch => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Cannon(cannon_fields) => cannon_fields.to_string(),
            Feature::Laser(laser_fields) => laser_fields.to_string(),
            Feature::Autofire => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Shield(shield_fields) => shield_fields.to_string(),
            Feature::Torquer(torquer_fields) => torquer_fields.to_string(),
            Feature::Launcher(launcher_fields) => launcher_fields.to_string(),
            Feature::Explode(explode_fields) => explode_fields.to_string(),
            Feature::Assembler => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Regrower => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::CannonBoost(cannon_boost_fields) => cannon_boost_fields.to_string(),
            Feature::Invulnerable => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::NoRegen => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Persistent => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Environmental => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Tractor(tractor_fields) => tractor_fields.to_string(),
            Feature::Root => NO_FEATURE_DATA_NEEDED.to_string(),
            // Feature::Grow => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Photosynth(photosynth_fields) => photosynth_fields.to_string(),
            Feature::Autolaunch => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::FreeRes => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Factory => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Seed(seed_fields) => seed_fields.to_string(),
            Feature::Melee(melee_fields) => melee_fields.to_string(),
            // Feature::Ungrow => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Unique => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Charging(charging_fields) => charging_fields.to_string(),
            Feature::SelfFactory => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::NoClip => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Invisible => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Bumper => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Teleporter(teleporter_fields) => teleporter_fields.to_string(),
            Feature::Deactivates => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Telespawn => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::NoClipAlly => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::IntLines => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::OneUse => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::NoRecolor => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::NoPalette => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::LauncherBarrage => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::AlwaysFire => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Palette => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::NeverFire => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::NoIcon => NO_FEATURE_DATA_NEEDED.to_string(),
        }
    }
}
