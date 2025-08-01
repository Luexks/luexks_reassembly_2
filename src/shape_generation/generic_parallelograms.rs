use std::f32::consts::PI;

use crate::{
    shapes::{
        nomenclature::shape_name_with_ratio_and_dimensions,
        scale::Scale,
        shapes::Shapes,
        vertex::{vert, Vertex},
        vertices::Vertices,
    },
    utility::angle::Angle,
};

use super::shape_generation::MASTER_SCALE;

pub fn add_generic_parallelograms_to_the(
    shapes: &mut Shapes,
    side_a_length: f32,
    side_b_length: f32,
    acute_angle: Angle,
    side_length_ratios_and_name_options: Vec<(f32, f32, Option<String>)>,
) -> (usize, usize) {
    shapes.add_mirrored_shape_from_scales(get_generic_parallelogram_scales(
        side_a_length,
        side_b_length,
        acute_angle,
        side_length_ratios_and_name_options,
    ));
    (shapes.0.len() - 2, shapes.0.len() - 1)
}

pub fn get_generic_parallelogram_scales(
    side_a_length: f32,
    side_b_length: f32,
    acute_angle: Angle,
    side_length_ratios_and_name_options: Vec<(f32, f32, Option<String>)>,
) -> Vec<Scale> {
    side_length_ratios_and_name_options
        .iter()
        .map(|side_length_ratios_and_name_option| {
            scale_from(
                side_a_length,
                side_b_length,
                &acute_angle,
                side_length_ratios_and_name_option,
            )
        })
        .collect()
}

#[rustfmt::skip]
fn scale_from(side_a_length: f32, side_b_length: f32, acute_angle: &Angle, side_length_ratios_and_name_option: &(f32, f32, Option<String>)) -> Scale {
    let side_a_length = side_a_length * MASTER_SCALE;
    let side_b_length = side_b_length * MASTER_SCALE;
    let side_length_ratio = (side_length_ratios_and_name_option.0, side_length_ratios_and_name_option.1);
    let name_option = &side_length_ratios_and_name_option.2;
    // dbg!(acute_angle);
    // dbg!(side_b_length);
    // dbg!(acute_angle.as_radians());
    // dbg!(acute_angle.as_radians().get_value().sin());
    // dbg!(acute_angle.as_radians().get_value().cos());
    Vertices((vec![
        vert!(-0.5 * side_b_length * acute_angle.as_radians().get_value().sin() * side_length_ratio.1, -0.5 * side_a_length * side_length_ratio.0),
        vert!(-0.5 * side_b_length * acute_angle.as_radians().get_value().sin() * side_length_ratio.1,  0.5 * side_a_length * side_length_ratio.0),
        vert!( 0.5 * side_b_length * acute_angle.as_radians().get_value().sin() * side_length_ratio.1,  0.5 * side_a_length * side_length_ratio.0 + side_b_length * acute_angle.as_radians().get_value().cos()),
        vert!( 0.5 * side_b_length * acute_angle.as_radians().get_value().sin() * side_length_ratio.1, -0.5 * side_a_length * side_length_ratio.0 + side_b_length * acute_angle.as_radians().get_value().cos()),
    ])).to_hull_scale(name_option.clone().unwrap_or_else(|| shape_name_with_ratio_and_dimensions(
        "parallelogram",
        side_length_ratio.0,
        side_length_ratio.1,
        PI / acute_angle.as_radians().get_value(),
        PI / (PI - acute_angle.as_radians().get_value())
    )))
}
