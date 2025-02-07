use crate::{
    shapes::{shapes::Shapes, vertex::Vertex, vertices::Vertices},
    utility::{angle::Angle, display_oriented_math::do2d_float_from},
};

use super::MASTER_SCALE;

const ISOTRI_ANGLES: [Angle; 3] = [
    Angle::Degree(10.0),
    Angle::Degree(20.0),
    Angle::Degree(30.0),
];
pub const ISOTRI_SCALE_COUNT: usize = 4;

pub fn add_isotris_to_the(shapes: &mut Shapes) -> usize {
    let scale_from = |angle: Angle, scale_index: usize| {
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
                .chain(vec![Vertex(do2d_float_from(0.0, 0.0))])
                .collect(),
        )
        .to_hull_scale(format!(
            "{}isotriS{}",
            angle.as_degrees().get_value(),
            scale_index
        ))
    };
    shapes.add_unmirrored_shape_from_scales(
        ISOTRI_ANGLES
            .iter()
            .flat_map(|angle| {
                (1..=ISOTRI_SCALE_COUNT).map(|scale_index| scale_from(angle.clone(), scale_index))
            })
            .collect(),
    );
    shapes.0.len() - 1
}
