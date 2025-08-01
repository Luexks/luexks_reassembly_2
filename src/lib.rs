pub mod blocks;
pub mod r#mod;
pub mod mod_configs;
pub mod shape_generation;
pub mod shapes;
pub mod utility;

use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use blocks::block_constants::BLOCKS_NAME;
// use mod_configs::mod_generation::*;
// use mod_configs::mod_metadata::*;
use r#mod::Mod;
use shapes::shape_constants::SHAPES_NAME;

// const USER: &str = "";

// fn main() {
//     let mod_path = create_mod_folder_and_get_mod_path(USER);

//     create_blocks_and_shapes(&mod_path);
// }

pub fn create_mod_folder_and_get_mod_path(user: &str, mod_name: &str) -> PathBuf {
    let reassembly_mods_path = format!("C:/users/{}/Saved Games/Reassembly/mods/", user);
    let reassembly_mods_path = Path::new(&reassembly_mods_path);

    let mod_path = reassembly_mods_path.join(mod_name);

    if mod_path.exists() {
        fs::remove_dir_all(&mod_path).expect("Failed to remove existing mod folder");
    }

    fs::create_dir(&mod_path).expect("Failed to create mod folder");

    mod_path
}

// pub fn get_mod_files(mod_path: &PathBuf) -> (Blocks, Shapes) {
//     let mut blocks: Blocks = Blocks::default();

//     let mut shapes = Shapes::default();

//     (blocks, shapes)

// create_mod_specifics(&mut blocks, &mut shapes);

// blocks_file
//     .write_all(blocks.to_string().as_bytes())
//     .expect("Failed to write to blocks.lua");
// shapes_path
//     .write_all(shapes.to_string().as_bytes())
//     .expect("Failed to write to shapes.lua");
// }

// pub fn write_mod_files() {
//     blocks_file
//         .write_all(blocks.to_string().as_bytes())
//         .expect("Failed to write to blocks.lua");
//     shapes_path
//         .write_all(shapes.to_string().as_bytes())
//         .expect("Failed to write to shapes.lua");
// }

pub fn write_mod(mod_path: PathBuf, r#mod: Mod) {
    let blocks_path = mod_path.join(BLOCKS_NAME);
    let shapes_path = mod_path.join(SHAPES_NAME);

    let mut blocks_file = File::create(&blocks_path).expect("Failed to create blocks.lua");
    let mut shapes_file = File::create(&shapes_path).expect("Failed to create shapes.lua");

    blocks_file
        .write_all(r#mod.blocks_option.unwrap().to_string().as_bytes())
        .expect("Failed to write to blocks.lua");
    shapes_file
        .write_all(r#mod.shapes_option.unwrap().to_string().as_bytes())
        .expect("Failed to write to shapes.lua");
}
