use std::fmt::{self, Display};

use crate::{display_oriented_number::DisplayOriented2D, utils::*};

pub struct Blocks(pub Vec<Block>);

impl Default for Blocks {
    fn default() -> Self {
        Blocks(Vec::new())
    }
}

impl Display for Blocks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{\n{}\n}}",
            self.0
                .iter()
                .map(|block| block.to_string())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

pub struct Block {
    pub id: BlockId,
    pub extends: Option<BlockId>,
    pub group: Option<i32>,
    pub sort: Option<BlockSort>,
    pub feautures: Option<Flags<Feature>>,
    pub capacity: Option<f32>,
    pub elasticity: Option<f32>,
    pub binding_id: Option<u8>,
    pub color_1: Option<Color>,
    pub color_2: Option<Color>,
    pub line_color: Option<Color>,
    pub shape: Option<i32>,
    pub scale: Option<u8>,
    pub name: Option<String>,
    pub points: Option<i32>,
    pub durability: Option<f32>,
    pub armor: Option<f32>,
    pub density: Option<f32>,
    pub blurb: Option<String>,
}

impl Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}{}{}{}{}{}{}{}}}",
            self.id,
            match self.extends {
                Some(extends) => format!(",{}", extends.to_string()),
                None => "".to_string(),
            },
            match self.group {
                Some(group) => format!(",{}", group.to_string()),
                None => "".to_string(),
            },
            match self.sort {
                Some(sort) => format!(",{}", sort.to_string()),
                None => "".to_string(),
            },
            match &self.feautures {
                Some(features) => format!(",{}", features.to_string()),
                None => "".to_string(),
            },
            match self.capacity {
                Some(capacity) => format!(",{}", capacity.to_string()),
                None => "".to_string(),
            },
            match self.elasticity {
                Some(elasticity) => format!(",{}", elasticity.to_string()),
                None => "".to_string(),
            },
            match &self.feautures {
                Some(features) => features
                    .0
                    .iter()
                    .map(|feature| feature.components_to_string())
                    .collect::<Vec<_>>()
                    .join(""),
                None => "".to_string(),
            },
        )
    }
}

pub enum Feature {
    Command,
    Thruster {
        force: Option<f32>,
        boost: Option<f32>,
        boost_time: Option<f32>,
        color_1: Option<Color>,
        color_2: Option<Color>,
    },
    Generator {
        capacity: Option<f32>,
        capacity_per_sec: Option<f32>,
    },
    Perishable {
        lifetime: Option<f32>,
    },
    Turret {
        speed: Option<Angle>,
        limit: Option<Angle>,
        barrel_size: Option<DisplayOriented2D>,
        barrel_count: Option<i32>,
        barrel_taper: Option<f32>,
    },
    // Launch,
    Cannon(Option<Cannon>),
    Laser(Option<Laser>),
    Autofire,
    Shield(Option<Shield>),
    Torquer {
        torque: Option<f32>,
    },
    Launcher {
        replicate_block: Option<Block>,
        speed: Option<f32>,
        power: Option<f32>,
        out_speed: Option<f32>,
        ang_vel: Option<f32>,
    },
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
                Feature::Cannon(_) => "CANNON",
                Feature::Laser(_) => "LASER",
                Feature::Autofire => "AUTOFIRE",
                Feature::Shield(_) => "SHIELD",
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
                Feature::NoClipAlly => "NOCLIPALY",
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

macro_rules! format_features {
    ($($feature:expr => $feature_name:expr),*) => {
        format!(
            "{}",
            vec![$(format_feature!($feature => $feature_name)),*].join("")
        )
    };
}

macro_rules! format_feature {
    ($feature:expr => $feature_name:expr) => {
        match $feature {
            Some(value) => format!(",{}={}", $feature_name, value),
            None => "".to_string(),
        }
    };
}

impl Feature {
    pub fn components_to_string(&self) -> String {
        match self {
            Feature::Command => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Thruster {
                force,
                boost,
                boost_time,
                color_1,
                color_2,
            } => format_features!(
                force => "thrusterForce",
                boost => "thrusterBoost",
                boost_time => "thrusterBoostTime",
                color_1 => "thrusterColor",
                color_2 => "thrusterColor1"
            ),
            Feature::Generator {
                capacity,
                capacity_per_sec,
            } => format_features!(
                capacity => "powerCapacity",
                capacity_per_sec => "generatorCapacityPerSec"
            ),
            Feature::Perishable { lifetime } => format_feature!(lifetime => "lifetime"),
            Feature::Turret {
                speed,
                limit,
                barrel_size,
                barrel_count,
                barrel_taper,
            } => format_features!(
                speed => "turretSpeed",
                limit => "turretLimit",
                barrel_size => "barrelSize",
                barrel_count => "barrelCount",
                barrel_taper => "barrelTaper"
            ),
            // Feature::Launch => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Cannon(cannon) => format_feature!(cannon => "cannon"),
            Feature::Laser(laser) => format_feature!(laser => "laser"),
            Feature::Autofire => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Shield(shield) => format_feature!(shield => "shield"),
            Feature::Torquer { torque } => format_feature!(torque => "torquerTorque"),
            Feature::Launcher {
                replicate_block,
                speed,
                power,
                out_speed,
                ang_vel,
            } => {
                format_features!(replicate_block => "replicateBlock", speed => "launcherOutSpeed", power => "launcherPower", out_speed => "launcherOutSpeed", ang_vel => "launcherAngVel")
            }
            Feature::Explode {
                explode_damage,
                explode_radius,
                explode_std_dev,
                explode_faction,
            } => format_features!(explode_damage => "explodeDamage",
                explode_radius => "explodeRadius",
                explode_std_dev => "explodeStdDev",
                explode_faction => "explodeFactioN"
            ),
            Feature::Assembler => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Regrower => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::CannonBoost(cannon_boost) => format_feature!(cannon_boost => "cannonBoost"),
            Feature::Invulnerable => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::NoRegen => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Persistent => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Environmental => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Tractor { range } => format_feature!(range => "tractorRange"),
            Feature::Root => NO_FEATURE_DATA_NEEDED.to_string(),
            // Feature::Grow => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Photosynth { per_sec } => format_feature!(per_sec => "photosynthPerSec"),
            Feature::Autolaunch => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::FreeRes => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Factory => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Seed { lifetime } => format_feature!(lifetime => "seedLifetime"),
            Feature::Melee { damage } => format_feature!(damage => "meleeDamage"),
            // Feature::Ungrow => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Unique => NO_FEATURE_DATA_NEEDED.to_string(),
            Self::Charging { max_time, min } => {
                format_features!(max_time => "chargingMaxTime", min => "chargingMin")
            }
            Feature::SelfFactory => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::NoClip => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Invisible => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Bumper => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Teleporter { power, radius } => {
                format_features!(power => "teleporterPower", radius => "teleporterRadius")
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

pub struct Cannon {
    rounds_per_sec: Option<f32>,
    rounds_per_burst: Option<u8>,
    explosive: Option<Flags<Explosive>>,
    pattern: Option<Flags<Pattern>>,
    burstyness: Option<f32>,
    muzzle_vel: Option<f32>,
    spread: Option<f32>,
    range_std_dev: Option<f32>,
    power: Option<f32>,
    damage: Option<f32>,
    range: Option<f32>,
    explode_radius: Option<f32>,
    color: Option<Color>,
    projectile_size: Option<f32>,
    fragment: Option<Fragment>,
}

impl Display for Cannon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}}}",
            format_features!(
                self.rounds_per_sec => "roundsPerSec",
                self.rounds_per_burst => "roundsPerBurst",
                &self.explosive => "explosive",
                &self.pattern => "pattern",
                self.burstyness => "burstyness",
                self.muzzle_vel => "muzzleVel",
                self.spread => "spread",
                self.range_std_dev => "rangeStdDev",
                self.power => "power",
                self.damage => "damage",
                self.range => "range",
                self.explode_radius => "explodeRadius",
                &self.color => "color",
                self.projectile_size => "projectileSize",
                &self.fragment => "fragment"
            )
        )
    }
}

struct Fragment {
    rounds_per_burst: Option<u8>,
    explosive: Option<Flags<Explosive>>,
    pattern: Option<Flags<Pattern>>,
    muzzle_vel: Option<f32>,
    spread: Option<f32>,
    range_std_dev: Option<f32>,
    damage: Option<f32>,
    range: Option<f32>,
    explode_radius: Option<f32>,
    color: Option<Color>,
    projectile_size: Option<f32>,
    fragment: Option<Box<Fragment>>,
}

impl Display for Fragment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}}}",
            format_features!(
                self.rounds_per_burst => "roundsPerBurst",
                &self.explosive => "explosive",
                &self.pattern => "pattern",
                self.muzzle_vel => "muzzleVel",
                self.spread => "spread",
                self.range_std_dev => "rangeStdDev",
                self.damage => "damage",
                self.range => "range",
                self.explode_radius => "explodeRadius",
                &self.color => "color",
                self.projectile_size => "projectileSize"
            )
        )
    }
}

enum Pattern {
    Random,
    Constant,
    Spiral,
    Absolute,
    Wave,
}

impl Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Pattern::Random => "RANDOM",
                Pattern::Constant => "CONSTANT",
                Pattern::Spiral => "SPIRAL",
                Pattern::Absolute => "ABSOLUTE",
                Pattern::Wave => "WAVE",
            }
        )
    }
}

enum Explosive {
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
            format_features!(
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

pub struct Shield {
    strength: Option<f32>,
    armor: Option<f32>,
    regen: Option<f32>,
    delay: Option<f32>,
    radius: Option<f32>,
    color: Option<Color>,
    line_color: Option<Color>,
    damaged_color: Option<Color>,
    power: Option<f32>,
}

impl Display for Shield {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}}}",
            format_features!(
                self.strength => "strength",
                self.armor => "armor",
                self.regen => "regen",
                self.delay => "delay",
                self.radius => "radius",
                &self.color => "color",
                &self.line_color => "lineColor",
                &self.damaged_color => "damagedColor",
                self.power => "power"
            )
        )
    }
}

pub struct CannonBoost {
    rounds_per_sec: Option<CannonBoostValue>,
    muzzle_vel: Option<CannonBoostValue>,
    power: Option<CannonBoostValue>,
    damage: Option<CannonBoostValue>,
    range: Option<CannonBoostValue>,
    explode_radius: Option<CannonBoostValue>,
    spread: Option<CannonBoostValue>,
}

impl Display for CannonBoost {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}}}",
            format_features!(&self.rounds_per_sec => "roundsPerSec",
                &self.muzzle_vel => "muzzleVel",
                &self.power => "power",
                &self.damage => "damage",
                &self.range => "range",
                &self.explode_radius => "explodeRadius",
                &self.spread => "spread"
            )
        )
    }
}

pub struct CannonBoostValue {
    multiplier: f32,
    flat: f32,
}

impl Display for CannonBoostValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{},{}}}", self.multiplier, self.flat)
    }
}
