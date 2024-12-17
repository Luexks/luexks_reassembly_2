use std::f32::consts::SQRT_2;

use lazy_static::lazy_static;

use crate::display_oriented_number::*;
use crate::shape_types::*;

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
        // add_fifth_rects_to(&mut rectangle_scale_factors);
        add_quarter_scales_to(&mut rectangle_scale_factors);
        add_half_rects_to(&mut rectangle_scale_factors);
        add_complementary_reciprocal_root_2_rects_to(&mut rectangle_scale_factors);
        add_reciprocal_root_2_rects_to(&mut rectangle_scale_factors);
        rectangle_scale_factors
    };
}

fn add_fifth_rects_to(rectangle_scale_factors: &mut Vec<(f32, f32, String)>) {
    rectangle_scale_factors.extend((1..=5).map(|scale_index| {
        (
            0.20,
            0.20 * scale_index as f32,
            format!("FifthRectS{}", scale_index),
        )
    }));
}
fn add_quarter_scales_to(rectangle_scale_factors: &mut Vec<(f32, f32, String)>) {
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
fn add_complementary_reciprocal_root_2_rects_to(
    rectangle_scale_factors: &mut Vec<(f32, f32, String)>,
) {
    rectangle_scale_factors.extend((1..=4).map(|scale_index| {
        (
            1.0,
            scale_index as f32 * (1.0 - 1.0 / SQRT_2),
            format!("ComplementaryReciprocalRoot2rectS{}", scale_index),
        )
    }));
}
fn add_reciprocal_root_2_rects_to(rectangle_scale_factors: &mut Vec<(f32, f32, String)>) {
    rectangle_scale_factors.extend((1..=4).map(|scale_index| {
        (
            1.0,
            scale_index as f32 * (1.0 / SQRT_2),
            format!("ReciprocalRoot2rectS{}", scale_index),
        )
    }));
}

pub const OCTAGON_SCALE_COUNT: usize = 4;

pub fn add_squares_to_the(shapes: &mut Shapes) {
    let scale_from = |scale_index: usize| {
        let half_square_length = 0.5 * MASTER_SCALE * (scale_index as f32);
        let unoriented_do2d = do2d_float_from(half_square_length, half_square_length);
        Vertices(
            (0..4)
                .map(|vert_index| Vertex(unoriented_do2d.orient_by_vert_index(vert_index)))
                .collect(),
        )
        .to_hull_scale(format!("SquareS{}", scale_index))
    };
    shapes.add_unmirrored_shape_from_scales(
        (1..=SQUARE_SCALE_COUNT)
            .map(|scale_index| scale_from(scale_index))
            .collect(),
    );
}

pub fn add_right_triangles_to_the(shapes: &mut Shapes) {
    let scale_from = |width_scale_factor: f32, height_scale_factor: f32| {
        scale_from_alternating_vertices_and_port_distributions!(
            vert!(0.0, 0.0),
            TowardsFromCurrentVert,
            vert!(0.0, MASTER_SCALE * height_scale_factor),
            Center,
            vert!(MASTER_SCALE * width_scale_factor * height_scale_factor, 0.0),
            BackwardsFromNextVert
            name: format!("{};{}rightTriS{}", 1, width_scale_factor, height_scale_factor)
        )
    };
    shapes.add_mirrored_shape_from_scales(
        RIGHT_TRIANGLE_HEIGHT_SCALE_FACTORS
            .iter()
            .flat_map(|&height_scale_factor| {
                RIGHT_TRIANGLE_WIDTH_SCALE_FACTORS
                    .iter()
                    .map(move |&width_scale_factor| {
                        scale_from(width_scale_factor, height_scale_factor)
                    })
            })
            .collect::<Vec<_>>(),
    );
}

pub fn add_rectangles_to_the(shapes: &mut Shapes) {
    let scale_from = |scale_factor_float_2d_and_name: &(f32, f32, String)| {
        let unoriented_do2d = do2d_float_from(
            scale_factor_float_2d_and_name.0 * MASTER_SCALE * 0.5,
            scale_factor_float_2d_and_name.1 * MASTER_SCALE * 0.5,
        );
        Vertices(
            (0..4)
                .map(|vert_index| Vertex(unoriented_do2d.orient_by_vert_index(vert_index)))
                .collect(),
        )
        .to_hull_scale(scale_factor_float_2d_and_name.2.clone())
    };
    shapes.add_unmirrored_shape_from_scales(
        RECTANGLE_SCALE_FACTORS_AND_NAMES
            .iter()
            .map(|scale_factor_float_2d| scale_from(scale_factor_float_2d))
            .collect(),
    );
}

#[rustfmt::skip]
pub fn add_octagons_to_the(shapes: &mut Shapes) {
    let scale_from = |scale_index: usize| {
        let (half_octagon_bounding_box_length, half_octagon_side_length) = if scale_index == 0 {
            (
                0.5 * MASTER_SCALE,
                (-0.5 + 1.0 / SQRT_2) * MASTER_SCALE,
            )
        } else {
            (
                (0.5 + 1.0 / SQRT_2) * MASTER_SCALE * scale_index as f32,
                0.5 * MASTER_SCALE * scale_index as f32,
            )
        };
        Vertices(
            (0..8)
                .map(|vert_index| {
                    Vertex(
                        do2d_float_from(
                            half_octagon_bounding_box_length,
                            half_octagon_side_length * if vert_index % 2 == 0 { -1.0 } else { 1.0 },
                        )
                        .rotate_by_vert_index((vert_index as i32 / 2) as usize),
                    )
                })
                .collect(),
        )
        .to_hull_scale(if scale_index == 0 { "GridOctagon".to_string() } else { format!("OctagonS{}", scale_index ) })
    };
    shapes.add_unmirrored_shape_from_scales(
        (0..=OCTAGON_SCALE_COUNT)
            .map(|scale_index| scale_from(scale_index))
            .collect(),
    );
}
