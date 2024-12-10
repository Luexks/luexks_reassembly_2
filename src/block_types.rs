use std::fmt::{self, Display};

use crate::{display_oriented_number::DisplayOriented2D, utils::*, Shapes};

macro_rules! format_components {
    ($($component:expr => $component_name:expr),*) => {
        format!(
            "{}",
            vec![$(format_component!($component => $component_name)),*].join("")
        )
    };
}

macro_rules! format_component {
    ($component:expr => $component_name:expr) => {
        match $component {
            Some(value) => format!(",{}={}", $component_name, value),
            None => "".to_string(),
        }
    };
}

macro_rules! block {
    ($($component_name:ident: $component_value:expr),*) => {
        Block {
            id: Some(BlockId::next()),
            $($component_name: Some($component_value),)*
            ..Block::default()
        }
    };
}
pub(crate) use block;

macro_rules! block_without_id {
    ($($component_name:ident: $component_value:expr),*) => {
        Block {
            id: None,
            $($component_name: Some($component_value),)*
            ..Block::default()
        }
    };
}
pub(crate) use block_without_id;

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

#[derive(Clone)]
pub struct Block {
    pub id: Option<BlockId>,
    pub extends: Option<BlockId>,
    pub group: Option<i32>,
    pub sort: Option<BlockSort>,
    pub features: Option<ExtendAccountingFeatureList>,
    pub capacity: Option<f32>,
    pub elasticity: Option<f32>,
    pub binding_id: Option<u8>,
    pub color_1: Option<Color>,
    pub color_2: Option<Color>,
    pub line_color: Option<Color>,
    pub shape: Option<ShapeId>,
    pub scale: Option<u8>,
    pub name: Option<FunkyString>,
    pub points: Option<i32>,
    pub durability: Option<f32>,
    pub armor: Option<f32>,
    pub density: Option<f32>,
    pub blurb: Option<FunkyString>,
}

impl Block {
    pub fn get_next_scale(&self) -> Block {
        block!(
            extends: self.id.unwrap(),
            scale: match self.scale {
                Some(value) => value + 1,
                None => 2,
            }
        )
    }

    pub fn get_scales(&self, scale_count: usize) -> Vec<Block> {
        (1..scale_count).fold(vec![self.clone()], |mut blocks, _| {
            blocks.push(blocks.last().unwrap().get_next_scale());
            blocks
        })
    }

    pub fn get_hull_blocks_from_shapes(&self, shapes: &Shapes) -> Vec<Block> {
        static mut BASE_BLOCK_ID: Option<BlockId> = None;
        static mut LAST_SHAPE_BLOCK_ID: Option<BlockId> = None;
        shapes
            .0
            .iter()
            .enumerate()
            .map(|(shape_index, shape)| {
                (0..shape.get_scale_count()).map(move |scale_index| match scale_index {
                    0 => match shape_index {
                        0 => {
                            unsafe {
                                BASE_BLOCK_ID = Some(self.id.unwrap());
                                LAST_SHAPE_BLOCK_ID = BASE_BLOCK_ID;
                            }
                            let mut new_block = self.clone();
                            new_block.shape = shape.get_id();
                            new_block.scale = Some(scale_index as u8 + 1);
                            new_block
                        }
                        _ => {
                            let new_block = block!(
                                extends: unsafe { BASE_BLOCK_ID.unwrap() },
                                shape: shape.get_id().unwrap()
                            );
                            unsafe {
                                LAST_SHAPE_BLOCK_ID = Some(new_block.id.unwrap());
                            }
                            new_block
                        }
                    },
                    _ => {
                        block!(
                            extends: unsafe { LAST_SHAPE_BLOCK_ID.unwrap() },
                            scale: scale_index as u8 + 1
                        )
                    }
                })
            })
            .flatten()
            .collect::<Vec<_>>()
    }
}

//     pub fn get_hull_blocks_from_shapes(&self, shapes: &Shapes) -> Vec<Block> {
//         shapes
//             .0
//             .iter()
//             .map(|shape| {
//                 (0..shape.get_scale_count()).map(|scale_index| {
//                     let mut new_block = self.clone();
//                     new_block.id = Some(BlockId::next());
//                     new_block.shape = shape.get_id();
//                     new_block.scale = Some(scale_index as u8 + 1);
//                     new_block
//                     // block!(
//                     //     shape: shape.get_id(),
//                     //     scale: scale_index as u8 + 1
//                     // )
//                 })
//             })
//             .flatten()
//             .collect::<Vec<_>>()
//     }
// }

impl Default for Block {
    fn default() -> Self {
        Block {
            id: Some(BlockId::default()),
            extends: None,
            group: None,
            sort: None,
            features: None,
            capacity: None,
            elasticity: None,
            binding_id: None,
            color_1: None,
            color_2: None,
            line_color: None,
            shape: None,
            scale: None,
            name: None,
            points: None,
            durability: None,
            armor: None,
            density: None,
            blurb: None,
        }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}{}{}{}}}",
            match self.id {
                Some(value) => value.to_string(),
                None => String::new(),
            },
            format_components!(
                self.extends => "extends",
                self.group => "group",
                self.sort => "sort",
                self.capacity => "capacity",
                self.elasticity => "elasicity",
                self.binding_id => "bindingId",
                &self.color_1 => "fillColor",
                &self.color_2 => "fillColor1",
                &self.line_color => "lineColor",
                self.shape => "shape",
                self.scale => "scale",
                &self.name => "name",
                self.points => "points",
                self.durability => "durability",
                self.armor => "armor",
                self.density => "density",
                &self.blurb => "blurb"
            ),
            match &self.features {
                Some(extend_accounting_feature_list) => {
                    match extend_accounting_feature_list.feature_list_same_as_extends {
                        true => "".to_string(),
                        false => {
                            format_component!(Some(&extend_accounting_feature_list.features) => "features")
                        }
                    }
                }

                None => "".to_string(),
            },
            match &self.features {
                Some(extend_accounting_feature_list) => extend_accounting_feature_list
                    .features
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

macro_rules! features {
    (
        $($feature:ident $(: { $($feature_component_name:ident: $feature_component_value:expr),*})?),*
    ) => {
        Flags(vec![
            $(new_feature!($feature $(: {$($feature_component_name: $feature_component_value),*})?)),*,
        ])

    };
}
pub(crate) use features;

macro_rules! new_feature {
    ($feature_name:ident) => {
        Feature::$feature_name
    };
    ($feature_name:ident: { $($feature_component_name:ident: $feature_component_value:expr),* }) => {
        Feature::$feature_name {
            $(
                $feature_component_name: Some($feature_component_value),
            )*
        }
    };
}
pub(crate) use new_feature;

#[derive(Clone)]
pub struct ExtendAccountingFeatureList {
    features: Flags<Feature>,
    feature_list_same_as_extends: bool,
}

#[derive(Clone)]
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
    Cannon {
        cannon: Option<Cannon>,
    },
    Laser {
        laser: Option<Laser>,
    },
    Autofire,
    Shield {
        shield: Option<Shield>,
    },
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
            } => format_components!(
                force => "thrusterForce",
                boost => "thrusterBoost",
                boost_time => "thrusterBoostTime",
                color_1 => "thrusterColor",
                color_2 => "thrusterColor1"
            ),
            Feature::Generator {
                capacity,
                capacity_per_sec,
            } => format_components!(
                capacity => "powerCapacity",
                capacity_per_sec => "generatorCapacityPerSec"
            ),
            Feature::Perishable { lifetime } => format_component!(lifetime => "lifetime"),
            Feature::Turret {
                speed,
                limit,
                barrel_size,
                barrel_count,
                barrel_taper,
            } => format_components!(
                speed => "turretSpeed",
                limit => "turretLimit",
                barrel_size => "barrelSize",
                barrel_count => "barrelCount",
                barrel_taper => "barrelTaper"
            ),
            // Feature::Launch => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Cannon { cannon } => format_component!(cannon => "cannon"),
            Feature::Laser { laser } => format_component!(laser => "laser"),
            Feature::Autofire => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Shield { shield } => format_component!(shield => "shield"),
            Feature::Torquer { torque } => format_component!(torque => "torquerTorque"),
            Feature::Launcher {
                replicate_block,
                speed,
                power,
                out_speed,
                ang_vel,
            } => {
                format_components!(replicate_block => "replicateBlock", speed => "launcherOutSpeed", power => "launcherPower", out_speed => "launcherOutSpeed", ang_vel => "launcherAngVel")
            }
            Feature::Explode {
                explode_damage,
                explode_radius,
                explode_std_dev,
                explode_faction,
            } => format_components!(explode_damage => "explodeDamage",
                explode_radius => "explodeRadius",
                explode_std_dev => "explodeStdDev",
                explode_faction => "explodeFactioN"
            ),
            Feature::Assembler => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Regrower => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::CannonBoost(cannon_boost) => format_component!(cannon_boost => "cannonBoost"),
            Feature::Invulnerable => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::NoRegen => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Persistent => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Environmental => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Tractor { range } => format_component!(range => "tractorRange"),
            Feature::Root => NO_FEATURE_DATA_NEEDED.to_string(),
            // Feature::Grow => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Photosynth { per_sec } => format_component!(per_sec => "photosynthPerSec"),
            Feature::Autolaunch => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::FreeRes => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Factory => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Seed { lifetime } => format_component!(lifetime => "seedLifetime"),
            Feature::Melee { damage } => format_component!(damage => "meleeDamage"),
            // Feature::Ungrow => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Unique => NO_FEATURE_DATA_NEEDED.to_string(),
            Self::Charging { max_time, min } => {
                format_components!(max_time => "chargingMaxTime", min => "chargingMin")
            }
            Feature::SelfFactory => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::NoClip => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Invisible => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Bumper => NO_FEATURE_DATA_NEEDED.to_string(),
            Feature::Teleporter { power, radius } => {
                format_components!(power => "teleporterPower", radius => "teleporterRadius")
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

#[derive(Clone)]
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
            format_components!(
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

#[derive(Clone)]
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
            format_components!(
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

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
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
            format_components!(
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

#[derive(Clone)]
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
            format_components!(&self.rounds_per_sec => "roundsPerSec",
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

#[derive(Clone)]
pub struct CannonBoostValue {
    multiplier: f32,
    flat: f32,
}

impl Display for CannonBoostValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{},{}}}", self.multiplier, self.flat)
    }
}
