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
            Some(
                {
                    let mut feature_data = default_feature_of_variant!($feature_name);
                    $(
                        feature_data.$feature_component_name = Some($feature_component_value);
                    )*
                    feature_data
                }
            )
        )
    };
}
pub(crate) use new_feature;

#[derive(Clone)]
pub enum Feature {
    Command,
    Thruster(Option<Thruster>),
    Generator(Option<Generator>),
    Perishable(Option<Perishable>),
    Turret(Option<Turret>),
    // Launch,
    Cannon(Option<Cannon>),
    Laser(Option<Laser>),
    Autofire,
    Shield(Option<Shield>),
    Torquer(Option<Torquer>),
    Launcher(Option<Launcher>),
    Explode(Option<Explode>),
    Assembler,
    Regrower,
    CannonBoost(Option<CannonBoost>),
    Invulnerable,
    NoRegen,
    Persistent,
    Environmental,
    Tractor(Option<Tractor>),
    Root,
    // Grow,
    Photosynth(Option<Photosynth>),
    Autolaunch,
    FreeRes,
    Factory,
    Seed(Option<Seed>),
    Melee(Option<Melee>),
    // Ungrow,
    Unique,
    Charging(Option<Charging>),
    // Transient,
    SelfFactory,
    NoClip,
    Invisible,
    Bumper,
    Teleporter(Option<Teleporter>),
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
        Thruster::default()
    };
    (Generator) => {
        Generator::default()
    };
    (Perishable) => {
        Perishable::default()
    };
    (Turret) => {
        Turret::default()
    };
    (Cannon) => {
        Cannon::default()
    };
    (Laser) => {
        Laser::default()
    };
    (Shield) => {
        Shield::default()
    };
    (Torquer) => {
        Torquer::default()
    };
    (Launcher) => {
        Launcher::default()
    };
    (Explode) => {
        Explode::default()
    };
    (CannonBoost) => {
        CannonBoos::default()
    };
    (Tractor) => {
        Tractor::default()
    };
    (Photosynth) => {
        Photosynth::default()
    };
    (Seed) => {
        Seed::default()
    };
    (Melee) => {
        Melee::default()
    };
    (Charging) => {
        Charging::default()
    };
    (Teleporter) => {
        Teleporter::default()
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
                Feature::Perishable(_) => "PERISHABLE",
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
            Feature::Thruster(thruster_option) => thruster_option
                .clone()
                .map_or(String::new(), |thruster| thruster.to_string()),
            Feature::Generator(generator_option) => generator_option
                .clone()
                .map_or(String::new(), |generator| generator.to_string()),
            Feature::Perishable(perishable_option) => perishable_option
                .clone()
                .map_or(String::new(), |perishable| perishable.to_string()),
            Feature::Turret(turret_option) => turret_option
                .clone()
                .map_or(String::new(), |turret| turret.to_string()),
            // Feature::Launch => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Cannon(cannon_option) => cannon_option
                .clone()
                .map_or(String::new(), |cannon| cannon.to_string()),
            Feature::Laser(laser_option) => laser_option
                .clone()
                .map_or(String::new(), |laser| laser.to_string()),
            Feature::Autofire => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Shield(shield_option) => shield_option
                .clone()
                .map_or(String::new(), |shield| shield.to_string()),
            Feature::Torquer(torquer_option) => torquer_option
                .clone()
                .map_or(String::new(), |torquer| torquer.to_string()),
            Feature::Launcher(launcher_option) => launcher_option
                .clone()
                .map_or(String::new(), |launcher| launcher.to_string()),
            Feature::Explode(explode_option) => explode_option
                .clone()
                .map_or(String::new(), |explode| explode.to_string()),
            Feature::Assembler => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Regrower => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::CannonBoost(cannon_boost_option) => cannon_boost_option
                .clone()
                .map_or(String::new(), |cannon_boost| cannon_boost.to_string()),
            Feature::Invulnerable => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::NoRegen => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Persistent => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Environmental => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Tractor(tractor_option) => tractor_option
                .clone()
                .map_or(String::new(), |tractor| tractor.to_string()),
            Feature::Root => NO_FEATURE_DATA_NEEDED.to_string(),
            // Feature::Grow => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Photosynth(photosynth_option) => photosynth_option
                .clone()
                .map_or(String::new(), |photosynth| photosynth.to_string()),
            Feature::Autolaunch => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::FreeRes => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Factory => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Seed(seed_option) => seed_option
                .clone()
                .map_or(String::new(), |seed| seed.to_string()),
            Feature::Melee(melee_option) => melee_option
                .clone()
                .map_or(String::new(), |melee| melee.to_string()),
            // Feature::Ungrow => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Unique => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Charging(charging_option) => charging_option
                .clone()
                .map_or(String::new(), |charging| charging.to_string()),
            Feature::SelfFactory => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::NoClip => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Invisible => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Bumper => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Teleporter(teleporter_option) => teleporter_option
                .clone()
                .map_or(String::new(), |teleporter| teleporter.to_string()),
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
