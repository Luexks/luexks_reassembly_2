use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

mod configs;
use configs::*;
mod utils;
use utils::*;
pub mod display_oriented_number;

pub mod shape_types;
use shape_types::*;
pub mod block_types;
use block_types::*;
pub mod mod_generation;
use mod_generation::*;
pub mod shape_configs;

fn main() {
    let mod_path = create_mod_folder();

    create_blocks_and_shapes(&mod_path);
}

fn create_mod_folder() -> PathBuf {
    let generator_path = std::env::current_dir().expect("Failed to get generator folder");
    let reassembly_mods_path = generator_path
        .parent()
        .expect("Failed to get reassembly mods folder");

    let mod_path = reassembly_mods_path.join(MOD_NAME);

    if mod_path.exists() {
        fs::remove_dir_all(&mod_path).expect("Failed to remove existing mod folder");
    }

    fs::create_dir(&mod_path).expect("Failed to create mod folder");

    mod_path
}

fn create_blocks_and_shapes(mod_path: &PathBuf) {
    let blocks_path = mod_path.join(BLOCKS_NAME);
    let mut blocks_file = File::create(&blocks_path).expect("Failed to create blocks.lua");

    let mut blocks: Blocks = Blocks::default();

    let shapes_path = mod_path.join(SHAPES_NAME);
    let mut shapes_path = File::create(&shapes_path).expect("Failed to create shapes.lua");

    let mut shapes = Shapes::default();

    create_mod_specifics(&mut blocks, &mut shapes);

    blocks_file
        .write_all(blocks.to_string().as_bytes())
        .expect("Failed to write to blocks.lua");
    shapes_path
        .write_all(shapes.to_string().as_bytes())
        .expect("Failed to write to shapes.lua");
}
