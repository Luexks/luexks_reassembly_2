use crate::{
    shapes::{
        nomenclature::shape_name_with_angle, scale::Scale, shapes::Shapes, vertex::Vertex,
        vertices::Vertices,
    },
    utility::{angle::Angle, display_oriented_math::do2d_float_from},
};

use super::shape_generation::MASTER_SCALE;

pub fn add_generic_isotris_to_the(shapes: &mut Shapes, angle: Angle, scale_count: usize) -> usize {
    shapes.add_unmirrored_shape_from_scales(
        (1..=scale_count)
            .map(|scale_index| scale_from(angle.clone(), scale_index as f32))
            .collect(),
    );
    shapes.0.len() - 1
}

fn scale_from(angle: Angle, scale_index: f32) -> Scale {
    let isotri_width =
        MASTER_SCALE * (angle.as_radians().get_value() * 0.25).cos() * scale_index as f32;
    let half_isotri_height =
        MASTER_SCALE * (angle.as_radians().get_value() * 0.25).sin() * scale_index as f32;
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
    .to_hull_scale(shape_name_with_angle(
        "isotri",
        angle,
        Some(scale_index as usize),
    ))
}
