use core::fmt;
use std::fmt::Display;
use std::fs::{self, File};
use std::io::{Write, Read};
use std::path::PathBuf;

mod configs;
use configs::*;
mod consts;
use consts::*;


const MOD_NAME: &str = "Luexks1";
const BLOCKS_NAME: &str = "blocks.lua";
const SHAPES_NAME: &str = "shapes.lua";

#[derive(Clone)]
enum DisplayOrientedNumber {
    Float(f32),
    Fraction{
        numerator: Box<DisplayOrientedNumber>,
        denominator: Box<DisplayOrientedNumber>,
    },
}

impl DisplayOrientedNumber {
    fn to_f32(&self) -> f32 {
        match self {
            DisplayOrientedNumber::Float(value) => *value,
            DisplayOrientedNumber::Fraction {numerator, denominator} => numerator.to_f32() / denominator.to_f32(),
        }
    }
}
    
impl Display for DisplayOrientedNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            DisplayOrientedNumber::Float(value) => format!("{}", value),
            DisplayOrientedNumber::Fraction { numerator, denominator } => format!("{}/{}", numerator.to_f32(), denominator.to_f32()),
        })
    }
}

enum PortDistribution {
    Center,
    TowardsFromCurrentVert,
    BackwardsFromNextVert,
}

struct Shapes(Vec<Shape>);

impl Default for Shapes {
    fn default() -> Self {
        Shapes(Vec::new())
    }
}

impl Display for Shapes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\n{}\n}}", self.0.iter()
        .map(|shape| shape.to_string())
        .collect::<Vec<_>>()
        .join("\n")
    )}
}

struct Shape {
    id: i32,
    scales: Vec<Scale>,
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{{{}{{\n{}\n}}}}", self.id, self.scales.iter()
        .map(|scale| scale.to_string())
        .collect::<Vec<_>>()
        .join("\n")
    ))}
}

struct Scale {
    verts: Verts,
    ports: Ports,
}

impl Display for Scale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{}{}}}", self.verts, self.ports)
    }
}

#[derive(Clone)]
struct Verts(Vec<Vert>);

impl Verts {
    fn to_hull_scale(self) -> Scale {
        Scale {
            verts: self.clone(),
            ports: Ports(self.0.iter()
                .enumerate()
                .zip(self.0.iter().cycle().skip(1))
                .flat_map(|((side_index, vert_1), vert_2)| {
                    let side_length = ((vert_1.x.to_f32() - vert_2.x.to_f32()).powi(2) + (vert_1.y.to_f32() - vert_2.y.to_f32()).powi(2)).sqrt();
                    let port_count = (side_length / TOTAL_SCALE).floor();
                    if port_count <= 1.0 {
                        return vec![Port {
                            side_index: side_index,
                            position: DisplayOrientedNumber::Float(PORT_POSITION_CENTER),
                            flags: Flags::<PortFlag>::default(),
                        }]
                    } else {
                        return (0..port_count as usize)
                            .map(|port_index| Port {
                                side_index: side_index,
                                // position: match PortDistribution::Center {
                                //     PortDistribution::Center => (),
                                //     _ => (),
                                // },
                                position: DisplayOrientedNumber::Fraction {
                                    // numerator: Box::new(DisplayOrientedNumber::Float(
                                    //     PORT_POSITION_CENTER
                                    //     - ((PORT_SPACING / side_length) * (port_count / 2.0 - 0.5))
                                    //     + ((PORT_SPACING / side_length) * port_index as f32))
                                    // ),
                                    numerator: Box::new(DisplayOrientedNumber::Float(
                                        PORT_POSITION_CENTER * side_length
                                        - (PORT_SPACING * (port_count / 2.0 - 0.5))
                                        + (PORT_SPACING * port_index as f32))
                                    ),
                                    denominator: Box::new(DisplayOrientedNumber::Float(side_length)),
                                },
                                flags: Flags::<PortFlag>::default(),
                            })
                            .collect::<Vec<_>>();
                    }
                }
                )
                .collect()
            )
        }
    }
}

impl Display for Verts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("verts={{{}}}", self.0.iter()
            .map(|vert| vert.to_string())
            .collect::<Vec<_>>()
            .join("")
    ))
    }
}

#[derive(Clone)]
struct Vert {
    x: DisplayOrientedNumber,
    y: DisplayOrientedNumber,
}

impl Vert {
    fn orient_by_index(&mut self, index: usize) -> Self {
        Vert {
            x: DisplayOrientedNumber::Float(self.x.to_f32() * VERTEX_ORIENTATION_MULTIPLIERS[index].0),
            y: DisplayOrientedNumber::Float(self.y.to_f32() * VERTEX_ORIENTATION_MULTIPLIERS[index].1),
        }
    }
}

impl Display for Vert {
    fn fmt(&self, f: &mut std::fmt::Formatter::<'_>) -> fmt::Result {
        write!(f, "{{{},{}}}",
            self.x,
            self.y,
        )
    }
}

struct Ports(Vec<Port>);

impl Display for Ports {
    fn fmt(&self, f: &mut std::fmt::Formatter::<'_>) -> fmt::Result {
        write!(f, "{}", format!("ports={{{}}}", self.0.iter()
            .map(|port| format!("{}", port))
            .collect::<Vec<_>>()
            .join("")
    ))}
}

struct Port {
    side_index: usize,
    position: DisplayOrientedNumber,
    flags: Flags<PortFlag>,
}

impl Display for Port {
    fn fmt(&self, f: &mut std::fmt::Formatter::<'_>) -> fmt::Result {
        write!(f, "{}", format!("{{{},{}{}}}",
            self.side_index,
            self.position,
            self.flags
        ))
    }
}

struct Flags<T: Display>(Vec<T>);

impl<T: Display> Default for Flags<T> {
    fn default() -> Self {
        Flags(Vec::new())
    }
}

impl<T: Display> Display for Flags<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0
            .iter()
            .map(|flag| flag.to_string())
            .collect::<Vec<_>>()
            .join("|")
        )
    }
}

enum PortFlag {
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
        write!(f, "{}", match self {
            PortFlag::ThrusterOut => "THRUSTER_OUT",
            PortFlag::ThrusterIn => "THRUSTER_IN",
            PortFlag::WeaponOut => "WEAPON_OUT",
            PortFlag::WeaponIn => "WEAPON_IN",
            PortFlag::Launcher => "LAUNCHER",
            PortFlag::Missile => "MISSILE",
            PortFlag::Root => "ROOT",
            PortFlag::None => "NONE",
            PortFlag::Normal => "NORMAL",
        })
    }
}

fn main() {
    let mod_path = create_mod_folder();
    
    create_blocks_and_shapes(&mod_path);
}

fn create_mod_folder() -> PathBuf {
    let generator_path = std::env::current_dir()
        .expect("Failed to get generator folder");
    let reassembly_mods_path = generator_path
        .parent()
        .expect("Failed to get reassembly mods folder");

    let mod_path = reassembly_mods_path.join(MOD_NAME);

    if mod_path.exists() {
        fs::remove_dir_all(&mod_path)
            .expect("Failed to remove existing mod folder");
    }

    fs::create_dir(&mod_path).expect("Failed to create mod folder");

    mod_path
}

fn create_blocks_and_shapes(mod_path: &PathBuf) {
    let blocks_path = mod_path.join(BLOCKS_NAME);
    let mut blocks_file = File::create(&blocks_path)
        .expect("Failed to create blocks.lua");

    let mut blocks: String = String::new();


    let shapes_path = mod_path.join(SHAPES_NAME);
    let mut shapes_path = File::create(&shapes_path)
        .expect("Failed to create shapes.lua");

    let mut shapes = Shapes::default();



    blocks_file.write_all(blocks.as_bytes())
        .expect("Failed to write to blocks.lua");
    shapes_path.write_all(shapes.to_string().as_bytes())
        .expect("Failed to write to shapes.lua");
}

fn create_squares(blocks: &mut String, shapes: &mut Shapes) {
    // shapes.0.push(Shape {
    //     id: 0,
    //     scales: (0..SQUARE_SCALE_COUNT)
    //         .map(|square_id| Scale {
    //             verts: Verts((0..4)
    //                 .map(|vert_index| Vert {
    //                     x: DisplayOrientedNumber::Float(TOTAL_SCALE / 2.0),
    //                     y: DisplayOrientedNumber::Float(TOTAL_SCALE / 2.0),
    //                 }.orient_by_index(vert_index))
    //                 .collect()
    //             ),
    //             ports: (),
    //         })
    //         .collect()
    // });
}