use std::f32::consts::SQRT_2;

use lazy_static::lazy_static;

pub const MOD_NAME: &str = "Luexks1";

pub const GROUP: i32 = 98;
pub const BLOCK_ID_BASE: u32 = 17000;
pub const BLOCK_SORT_BASE: i32 = 100;
pub const SHAPE_ID_BASE: u32 = 129873000;

pub const FUNKY_PORT_FORMATING: bool = true;

pub const MASTER_SCALE: f32 = 10.0;
pub const PORT_SPACING: f32 = MASTER_SCALE;
pub const PORT_SPACING_FROM_VERT: f32 = MASTER_SCALE / 2.0;

pub const SQUARE_SCALE_COUNT: usize = 4;
pub const RIGHT_TRIANGLE_WIDTH_SCALE_FACTORS: [f32; 8] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
pub const RIGHT_TRIANGLE_HEIGHT_SCALE_FACTORS: [f32; 3] = [0.5, 1.0, 2.0];
lazy_static! {
    pub static ref RECTANGLE_SCALE_FACTORS_AND_NAMES: Vec<(f32, f32, String)> = {
        let mut rectangle_scale_factors: Vec<(f32, f32, String)> = Vec::new();
        add_quarter_scales_to(&mut rectangle_scale_factors);
        add_fifth_rects_to(&mut rectangle_scale_factors);
        add_half_rects_to(&mut rectangle_scale_factors);
        add_octagon_adapting_rects_to(&mut rectangle_scale_factors);
        add_iscoceles_right_triangle_irrational_side_rects_to(&mut rectangle_scale_factors);
        rectangle_scale_factors
    };
}

fn add_quarter_scales_to(rectangle_scale_factors: &mut Vec<(f32, f32, String)>) {
    rectangle_scale_factors.extend((1..=5).map(|scale_index| {
        (
            0.20,
            0.20 * scale_index as f32,
            format!("FifthRectS{}", scale_index),
        )
    }));
}
fn add_fifth_rects_to(rectangle_scale_factors: &mut Vec<(f32, f32, String)>) {
    rectangle_scale_factors.extend((1..=4).map(|scale_index| {
        (
            0.25,
            0.25 * scale_index as f32,
            format!("QuarterRectS{}", scale_index),
        )
    }));
}
fn add_half_rects_to(rectangle_scale_factors: &mut Vec<(f32, f32, String)>) {
    rectangle_scale_factors.extend((1..=2).map(|scale_index| {
        (
            0.5,
            0.5 * scale_index as f32,
            format!("HalfRectS{}", scale_index),
        )
    }));
}
fn add_octagon_adapting_rects_to(rectangle_scale_factors: &mut Vec<(f32, f32, String)>) {
    rectangle_scale_factors.extend((1..=4).map(|scale_index| {
        (
            scale_index as f32 * (1.0 - 1.0 / SQRT_2),
            1.0 * scale_index as f32,
            format!("OctAdaptingRectS{}", scale_index),
        )
    }));
}
fn add_iscoceles_right_triangle_irrational_side_rects_to(
    rectangle_scale_factors: &mut Vec<(f32, f32, String)>,
) {
    rectangle_scale_factors.extend((1..=4).map(|scale_index| {
        (
            1.0,
            scale_index as f32 * (1.0 / SQRT_2),
            format!("IRTISrectS{}", scale_index as f32 * 0.5),
        )
    }));
}
