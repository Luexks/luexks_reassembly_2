use crate::{
    shapes::{scale::Scale, shapes::Shapes, vertex::Vertex, vertices::Vertices},
    utility::display_oriented_math::do2d_float_from,
};
use lazy_static::lazy_static;
use std::f32::consts::SQRT_2;

use super::MASTER_SCALE;

pub fn add_rectangles_to_the(shapes: &mut Shapes) -> usize {
    shapes.add_unmirrored_shape_from_scales(
        RECTANGLE_SCALE_FACTORS_AND_NAMES
            .iter()
            .map(|scale_factor_float_2d_and_name| scale_from(scale_factor_float_2d_and_name))
            .collect(),
    );
    shapes.0.len() - 1
}

fn scale_from(scale_factor_float_2d_and_name: &(f32, f32, String)) -> Scale {
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
}

lazy_static! {
    static ref QUARTER: Vec<(f32, f32, String)> = (1..=4)
        .map(|scale_index| {
            (
                0.25,
                0.25 * scale_index as f32,
                format!("QuarterRectS{}", scale_index),
            )
        })
        .collect::<Vec<_>>();
    static ref HALF: Vec<(f32, f32, String)> = (1..=4)
        .map(|scale_index| {
            (
                0.5,
                0.5 * scale_index as f32,
                format!("HalfRectS{}", scale_index),
            )
        })
        .collect::<Vec<_>>();
    static ref COMPLEMENTARY_RECIPROCAL_ROOT_2: Vec<(f32, f32, String)> = (1..=4)
        .map(|scale_index| {
            (
                1.0,
                scale_index as f32 * (1.0 - 1.0 / SQRT_2),
                format!("ComplementaryReciprocalRoot2rectS{}", scale_index),
            )
        })
        .collect::<Vec<_>>();
    static ref RECIPROCAL_ROOT_2: Vec<(f32, f32, String)> = (1..=4)
        .map(|scale_index| {
            (
                1.0,
                scale_index as f32 * (1.0 / SQRT_2),
                format!("ReciprocalRoot2rectS{}", scale_index),
            )
        })
        .collect();
    static ref RECTANGLE_SCALE_FACTORS_AND_NAMES: Vec<(f32, f32, String)> = {
        Vec::new()
            .iter()
            .chain(QUARTER.iter())
            .chain(HALF.iter())
            .chain(COMPLEMENTARY_RECIPROCAL_ROOT_2.iter())
            .chain(RECIPROCAL_ROOT_2.iter())
            .cloned()
            .collect::<Vec<_>>()
    };
}
