use core::fmt;
use std::fmt::Display;
use std::fs::{self, File};
use std::io::{Write, Read};
use std::path::PathBuf;

const MOD_NAME: &str = "Luexks1";
const BLOCKS_NAME: &str = "blocks.lua";
const SHAPES_NAME: &str = "shapes.lua";

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

struct Scale {
    verts: Vec<Vert>,
    ports: Vec<Port>,
}

impl Display for Scale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{}{}}}", display_verts(&self.verts), display_ports(&self.ports))
    }
}

struct Vert {
    x: DisplayOrientedNumber,
    y: DisplayOrientedNumber,
}

impl Display for Vert {
    fn fmt(&self, f: &mut std::fmt::Formatter::<'_>) -> fmt::Result {
        write!(f, "{{{},{}}}",
            self.x,
            self.y,
        )
    }
}

fn display_verts(verts: &Vec<Vert>) -> String {
    format!("verts={{{}}}", verts.iter()
            .map(|vert| vert.to_string())
            .collect::<Vec<_>>()
            .join("")
    )
}

struct Port {
    side_id: i32,
    position: DisplayOrientedNumber,
    flags: Flags<PortFlag>,
}

impl Display for Port {
    fn fmt(&self, f: &mut std::fmt::Formatter::<'_>) -> fmt::Result {
        write!(f, "ports={{{}}}", format!("{{{},{}{}}}",
            self.side_id,
            self.position,
            self.flags
        ))
    }
}

fn display_ports(ports: &Vec<Port>) -> String {
    format!("{}", ports.iter()
            .map(|port| port.to_string())
            .collect::<Vec<_>>()
            .join("")
    )
}

struct Flags<T: Display>(Vec<T>);

impl<T: Display> Display for Flags<T> {
    // fn display(&self) -> String {
        // let mut output = String::new();
        // for (flag_index, flag) in self.0.iter().enumerate() {
            // output.push_str(&flag.to_string());
            // if flag_index != self.0.len() - 1 {
                // output.push('|');
            // }
        // }
        // output
    // }
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
    
    create_blocks(&mod_path);
    create_shapes(&mod_path);
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

fn create_blocks(mod_path: &PathBuf) {
    let blocks_path = mod_path.join(BLOCKS_NAME);
    let mut blocks_file = File::create(&blocks_path)
        .expect("Failed to create blocks.lua");

    let mut blocks: String = String::new();

    blocks_file.write_all(blocks.as_bytes())
        .expect("Failed to write to blocks.lua");
}

fn create_shapes(mod_path: &PathBuf) {
    let shapes_path = mod_path.join(BLOCKS_NAME);
    let mut shapes_path = File::create(&shapes_path)
        .expect("Failed to create shapes.lua");

    let mut shapes: String = String::new();

    shapes.push_str("{");

    shapes.push_str("}");

    shapes_path.write_all(shapes.as_bytes())
        .expect("Failed to write to shapes.lua");

}

fn create_shape_square(shapes: &mut String) {

}
fn get_shape_scale(verts: Vec<Vert>, ports: Vec<Port>) -> String {
    let mut shape_scale = String::new();
    shape_scale.push_str("\n\t\t\t{verts={");
    for vert in verts.iter() {
        shape_scale.push_str(&format!("{}", vert));
    }
    shape_scale.push_str("}ports={");
    for port in ports.iter() {
        shape_scale.push_str(&format!("{}", port));
    }
    shape_scale.push_str("}}");
    shape_scale
}
