use std::f32::consts::PI;

use crate::{
    shapes::{
        nomenclature::shape_name_with_ratio, scale::Scale, shapes::Shapes, vertex::Vertex,
        vertices::Vertices,
    },
    utility::{angle::Angle, display_oriented_math::do2d_float_from},
};

use super::shape_generation::MASTER_SCALE;

pub fn add_generic_regular_trapezium_to_the(
    shapes: &mut Shapes,
    angle_ratio: (f32, f32),
    scale_count: usize,
    short_forwards: bool,
) -> usize {
    shapes.add_unmirrored_shape_from_scales(
        (1..=scale_count)
            .map(|scale_index| scale_from(angle_ratio, scale_index, short_forwards))
            .collect(),
    );
    shapes.0.len() - 1
}

#[rustfmt::skip]
fn scale_from(angle_ratio: (f32, f32), scale_index: usize, short_forwards: bool) -> Scale {
    let angle_ratio_sum = angle_ratio.0 + angle_ratio.1;
    let fraction_radians = Angle::Radian(PI / angle_ratio_sum);
    // let width = MASTER_SCALE * (fraction_radians.get_value() * angle_ratio.1 as f32).sin();
    // println!("God speed.");
    // dbg!(fraction_radians.as_degrees().get_value() * angle_ratio.0);
    let width = MASTER_SCALE * 0.5 * (fraction_radians.get_value() * angle_ratio.0).tan();
    // let width = (MASTER_SCALE * 0.5) / (fraction_radians.get_value() * angle_ratio.0).tan();
    // dbg!(width);
    // println!("Touch down.");
    let right_height = MASTER_SCALE * (scale_index as f32);
    let left_height = MASTER_SCALE + right_height;
    let (left_height, right_height) = if short_forwards {
        (left_height, right_height)
    } else {
        (right_height, left_height)
    };
    // dbg!(fraction_radians.as_degrees().get_value());
    // dbg!(fraction_radians.as_degrees().get_value() * angle_ratio.0 as f32);
    Vertices(vec![
        Vertex(do2d_float_from(-0.5 * width, -0.5 * left_height)),
        Vertex(do2d_float_from(-0.5 * width,  0.5 * left_height)),
        Vertex(do2d_float_from( 0.5 * width,  0.5 * right_height)),
        Vertex(do2d_float_from( 0.5 * width, -0.5 * right_height)),
    ])
    .to_hull_scale(shape_name_with_ratio(
        "trapezium",
        angle_ratio.0,
        angle_ratio.1,
        Some(scale_index),
    ))
}
