use std::f32::consts::TAU;

use crate::{
    shapes::{
        nomenclature::shape_name_with_ratio, scale::Scale, shapes::Shapes, vertex::Vertex,
        vertices::Vertices,
    },
    utility::{angle::Angle, display_oriented_math::do2d_float_from},
};

use super::shape_generation::MASTER_SCALE;

pub fn add_generic_rhombi_to_the(
    shapes: &mut Shapes,
    angle_ratio: (f32, f32),
    scale_count: usize,
    antecedent_forwards: bool,
) -> usize {
    shapes.add_unmirrored_shape_from_scales(
        (1..=scale_count)
            .map(|scale_index| scale_from(angle_ratio, scale_index, antecedent_forwards))
            .collect(),
    );
    shapes.0.len() - 1
}

fn scale_from(angle_ratio: (f32, f32), scale_index: usize, antecedent_forwards: bool) -> Scale {
    let angle_ratio_sum = angle_ratio.0 + angle_ratio.1;
    let fraction_radians = Angle::Radian(TAU / angle_ratio_sum);
    let forwards_angle = fraction_radians.get_value()
        * 0.25
        * if antecedent_forwards {
            angle_ratio.0
        } else {
            angle_ratio.1
        };
    let half_width = MASTER_SCALE * forwards_angle.cos() * scale_index as f32;
    let half_height: f32 = MASTER_SCALE * forwards_angle.sin() * scale_index as f32;
    let unoriented_do2d = do2d_float_from(half_width, half_height);
    // dbg!(fraction_radians.as_degrees().get_value());
    // dbg!(angle_ratio);
    // dbg!(angle_ratio.0 * fraction_radians.as_degrees().get_value());
    // dbg!(angle_ratio.1 * fraction_radians.as_degrees().get_value());
    Vertices(
        (0..=3)
            .map(|vert_index| Vertex(unoriented_do2d.diagonal_orient_by_vert_index(vert_index)))
            .collect(),
    )
    .to_hull_scale(shape_name_with_ratio(
        "rhom",
        angle_ratio.0,
        angle_ratio.1,
        Some(scale_index),
    ))
}
