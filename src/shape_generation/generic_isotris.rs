use crate::{
    shapes::{
        nomenclature::shape_name_with_angle, scale::Scale, shapes::Shapes, vertex::Vertex,
        vertices::Vertices,
    },
    utility::{angle::Angle, display_oriented_math::do2d_float_from},
};

use super::shape_generation::MASTER_SCALE;

pub fn add_generic_isotris_to_the(
    shapes: &mut Shapes,
    angle: Angle,
    scale_count: usize,
    scale_multiplier: f32,
    name_option: Option<String>,
) -> usize {
    shapes.add_unmirrored_shape_from_scales(get_isotri_scales(
        angle,
        scale_count,
        scale_multiplier,
        name_option,
    ));
    shapes.0.len() - 1
}

pub fn get_isotri_scales(
    angle: Angle,
    scale_count: usize,
    scale_multiplier: f32,
    name_option: Option<String>,
) -> Vec<Scale> {
    (1..=scale_count)
        .map(|scale_index| {
            scale_from(
                angle.clone(),
                scale_index as f32,
                scale_multiplier,
                name_option.clone(),
            )
        })
        .collect()
}

fn scale_from(
    angle: Angle,
    scale_index: f32,
    scale_multiplier: f32,
    name_option: Option<String>,
) -> Scale {
    let isotri_width = MASTER_SCALE
        * scale_multiplier
        * (angle.as_radians().get_value() * 0.5).cos()
        * scale_index as f32;
    let half_isotri_height = MASTER_SCALE
        * scale_multiplier
        * (angle.as_radians().get_value() * 0.5).sin()
        * scale_index as f32;
    Vertices(
        (0..=1)
            .map(|vert_index| {
                Vertex(
                    do2d_float_from(isotri_width, half_isotri_height)
                        .orient_by_vert_index(vert_index),
                )
            })
            .chain(std::iter::once(Vertex(do2d_float_from(0.0, 0.0))))
            .collect(),
    )
    .to_hull_scale(
        name_option
            .unwrap_or_else(|| shape_name_with_angle("isotri", angle, Some(scale_index as usize))),
    )
}
