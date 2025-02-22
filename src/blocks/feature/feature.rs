use crate::blocks::block::Block;
use crate::blocks::feature::*;
use crate::utility::angle::Angle;
use crate::utility::color::Color;
use crate::utility::component_formatting::{format_component_option, format_component_options};
use crate::utility::display_oriented_math::DisplayOriented2D;
use std::fmt::{self, Display};

const NO_FEATURE_DATA_NEEDED: &str = "";

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
            // $(
            //     $feature_component_name: Some($feature_component_value),
            // )*
            // ..Default::default()
            // ..Feature::$feature_name.default_for_variant()
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
    // Launch,\
    Cannon(Option<Cannon>),
    Laser(Option<Laser>),
    Autofire,
    Shield(Option<Shield>),
    Torquer(Option<Torquer>),
    Launcher(Option<Launcher>),
    Explode {
        explode_damage: Option<f32>,
        explode_radius: Option<f32>,
        explode_std_dev: Option<f32>,
        explode_faction: Option<i32>,
    },
    Assembler,
    Regrower,
    CannonBoost(Option<CannonBoost>),
    Invulnerable,
    NoRegen,
    Persistent,
    Environmental,
    Tractor {
        range: Option<f32>,
    },
    Root,
    // Grow,
    Photosynth {
        per_sec: Option<f32>,
    },
    Autolaunch,
    FreeRes,
    Factory,
    Seed {
        lifetime: Option<f32>,
    },
    Melee {
        damage: Option<f32>,
    },
    // Ungrow,
    Unique,
    Charging {
        max_time: Option<f32>,
        min: Option<f32>,
    },
    // Transient,
    SelfFactory,
    NoClip,
    Invisible,
    Bumper,
    Teleporter {
        power: Option<f32>,
        radius: Option<f32>,
    },
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

macro_rules! default_feature_of_variant {
    (Thruster) => {
        Thruster::default()
    };
    (Generator) => {
        Generator::default()
    };
    (Turret) => {
        Turret::default()
    };
       // ($feature_variant:ident) => {
       //     // {
       //     //     if let Feature::Thruster(_) = Feature::$feature_variant(None) {
       //     //         Thruster::default()
       //     //     } else if let Feature::Generator(_) = Feature::$feature_variant(None) {
       //     //         Generator::default()
       //     //     } else {
       //     //         panic!()
       //     //     }
       //     // }
       //     match Feature::$feature_variant(None) {
       //         // Feature::Thruster(_) => Thruster::default(),
       //         // Feature::Generator(_) => Generator::default(),
       //         // Feature::Perishable(_) => Perishable::default(),
       //         // Feature::Turret(_) => Turret::default(),
       //         // Feature::Cannon(_) => Cannon::default(),
       //         // Feature::Laser(_) => Laser::default(),
       //         // Feature::Shield(_) => Shield::default(),
       //         // Feature::Torquer(_) => Torquer::default(),
       //         // Feature::Launcher(_) => Launcher::default(),
       //         Feature::Thruster(_) => Feature::Thruster(Some(Thruster::default())),
       //         Feature::Generator(_) => Feature::Generator(Some(Generator::default())),
       //         Feature::Perishable(_) => Feature::Perishable(Some(Perishable::default())),
       //         Feature::Turret(_) => Feature::Turret(Some(Turret::default())),
       //         Feature::Cannon(_) => Feature::Cannon(Some(Cannon::default())),
       //         Feature::Laser(_) => Feature::Laser(Some(Laser::default())),
       //         Feature::Shield(_) => Feature::Shield(Some(Shield::default())),
       //         Feature::Torquer(_) => Feature::Torquer(Some(Torquer::default())),
       //         Feature::Launcher(_) => Feature::Launcher(Some(Launcher::default())),
       //         _ => todo!(),
       //     }
       // };
}
pub(crate) use default_feature_of_variant;

// macro_rules! impl_default_for_feature {
//     ($($variant:ident { $($field:ident),* }),* $(,)?) => {
//         impl Default for Feature {
//             fn default() -> Self {
//                 Feature::Command
//             }
//         }

//         impl Feature {
//             pub fn default_for_variant(variant: &Feature) -> Feature {
//                 match variant {
//                     $(Feature::$variant { .. } => Feature::$variant {
//                         $($field: None),*
//                     },)*
//                     _ => variant.clone(),
//                 }
//             }
//         }
//     };
// }

// impl_default_for_feature!(
//     Thruster { force, boost, boost_time, color_1, color_2 },
//     Generator { capacity, capacity_per_sec },
//     Perishable { lifetime },
//     Turret { speed, limit, barrel_size, barrel_count, barrel_taper },
//     Cannon { cannon },
//     Laser { laser },
//     Shield { shield },
//     Torquer { torque },
//     Launcher { replicate_block, speed, power, out_speed, ang_vel },
//     Explode { explode_damage, explode_radius, explode_std_dev, explode_faction },
//     Tractor { range },
//     Photosynth { per_sec },
//     Seed { lifetime },
//     Melee { damage },
//     Charging { max_time, min },
//     Teleporter { power, radius },
// );

impl Display for Feature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Feature::Command => "COMMAND",
                Feature::Thruster { .. } => "THRUSTER",
                Feature::Generator { .. } => "GENERATOR",
                Feature::Perishable { .. } => "PERISHABLE",
                Feature::Turret { .. } => "TURRET",
                Feature::Cannon { .. } => "CANNON",
                Feature::Laser { .. } => "LASER",
                Feature::Autofire => "AUTOFIRE",
                Feature::Shield { .. } => "SHIELD",
                Feature::Torquer { .. } => "TORQUER",
                Feature::Launcher { .. } => "LAUNCHER",
                Feature::Explode { .. } => "EXPLODE",
                Feature::Assembler => "ASSEMBLER",
                Feature::Regrower => "REGROWER",
                Feature::CannonBoost(_) => "CANNON_BOOST",
                Feature::Invulnerable => "INVULNERABLE",
                Feature::NoRegen => "NOREGEN",
                Feature::Persistent => "PERSISTENT",
                Feature::Environmental => "ENVIRONMENTAL",
                Feature::Tractor { .. } => "TRACTOR",
                Feature::Root => "ROOT",
                Feature::Photosynth { .. } => "PHOTOSYNTH",
                Feature::Autolaunch => "AUTOLAUNCH",
                Feature::FreeRes => "FREERES",
                Feature::Factory => "FACTORY",
                Feature::Seed { .. } => "SEED",
                Feature::Melee { .. } => "MELEE",
                Feature::Unique => "UNIQUE",
                Feature::Charging { .. } => "CHARGING",
                Feature::SelfFactory => "SELFFACTORY",
                Feature::NoClip => "NOCLIP",
                Feature::Invisible => "INVISIBLE",
                Feature::Bumper => "BUMPER",
                Feature::Teleporter { .. } => "TELEPORTER",
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
            Feature::Explode {
                explode_damage,
                explode_radius,
                explode_std_dev,
                explode_faction,
            } => format_component_options!(explode_damage => "explodeDamage",
                explode_radius => "explodeRadius",
                explode_std_dev => "explodeStdDev",
                explode_faction => "explodeFaction"
            ),
            Feature::Assembler => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Regrower => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::CannonBoost(cannon_boost) => {
                format_component_option!(cannon_boost => "cannonBoost")
            }
            Feature::Invulnerable => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::NoRegen => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Persistent => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Environmental => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Tractor { range } => format_component_option!(range => "tractorRange"),
            Feature::Root => NO_FEATURE_DATA_NEEDED.to_string(),
            // Feature::Grow => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Photosynth { per_sec } => {
                format_component_option!(per_sec => "photosynthPerSec")
            }
            Feature::Autolaunch => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::FreeRes => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Factory => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Seed { lifetime } => format_component_option!(lifetime => "seedLifetime"),
            Feature::Melee { damage } => format_component_option!(damage => "meleeDamage"),
            // Feature::Ungrow => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Unique => NO_FEATURE_DATA_NEEDED.to_string(),
            Self::Charging { max_time, min } => {
                format_component_options!(max_time => "chargingMaxTime", min => "chargingMin")
            }
            Feature::SelfFactory => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::NoClip => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Invisible => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Bumper => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Teleporter { power, radius } => {
                format_component_options!(power => "teleporterPower", radius => "teleporterRadius")
            }
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
