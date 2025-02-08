use std::f32::consts::SQRT_2;

use crate::{
    shapes::{
        nomenclature::shape_name, scale::Scale, shapes::Shapes, vertex::Vertex, vertices::Vertices,
    },
    utility::display_oriented_math::do2d_float_from,
};

use super::MASTER_SCALE;

pub const OCTAGON_SCALE_COUNT: usize = 3;

pub fn add_octagons_to_the(shapes: &mut Shapes) -> usize {
    shapes.add_unmirrored_shape_from_scales(
        (0..OCTAGON_SCALE_COUNT)
            .map(|scale_index| scale_from(scale_index))
            .collect(),
    );
    shapes.0.len() - 1
}

fn scale_from(scale_index: usize) -> Scale {
    match scale_index {
        0 => grid_octagon_scale(),
        _ => non_grid_octagon_scale(scale_index),
    }
}

fn grid_octagon_scale() -> Scale {
    let (half_bounding_box_length, half_side_length) =
        (0.5 * MASTER_SCALE, (-0.5 + 1.0 / SQRT_2) * MASTER_SCALE);
    vertices_from(half_bounding_box_length, half_side_length)
        .to_hull_scale(shape_name("GridOctagon", None))
}

fn non_grid_octagon_scale(scale_index: usize) -> Scale {
    let (half_bounding_box_length, half_side_length) = (
        (0.5 + 1.0 / SQRT_2) * MASTER_SCALE * scale_index as f32,
        0.5 * MASTER_SCALE * scale_index as f32,
    );
    vertices_from(half_bounding_box_length, half_side_length)
        .to_hull_scale(shape_name("Octagon", Some(scale_index)))
}

fn vertices_from(half_bounding_box_length: f32, half_side_length: f32) -> Vertices {
    Vertices(
        (0..8)
            .map(|vert_index| {
                let pre_rotation_octant_turn_parity = if vert_index % 2 == 0 { -1.0 } else { 1.0 };
                let unrotated_do2d = do2d_float_from(
                    half_bounding_box_length,
                    half_side_length * pre_rotation_octant_turn_parity,
                );
                Vertex(unrotated_do2d.rotate_by_vert_index(vert_index / 2 as usize))
            })
            .collect(),
    )
}
