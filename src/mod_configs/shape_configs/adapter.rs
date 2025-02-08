use crate::{
    shapes::{
        nomenclature::shape_name, scale::Scale, shapes::Shapes, vertex::Vertex, vertices::Vertices,
    },
    utility::display_oriented_math::do2d_float_from,
};

use super::MASTER_SCALE;

pub const ADAPTER_SCALE_COUNT: usize = 4;

pub fn add_adapters_to_the(shapes: &mut Shapes) -> usize {
    shapes.add_unmirrored_shape_from_scales(
        (0..=ADAPTER_SCALE_COUNT)
            .map(|scale_index: usize| scale_from(scale_index))
            .collect::<Vec<_>>(),
    );
    shapes.0.len() - 1
}

fn scale_from(scale_index: usize) -> Scale {
    if scale_index == 0 {
        scale_0_vertices()
    } else {
        scale_from_non_zero_vertices(scale_index)
    }
    .to_hull_scale(shape_name("Adapter", Some(scale_index)))
}

fn scale_0_vertices() -> Vertices {
    let half_adapter_width = 0.25 * MASTER_SCALE;
    let half_adapter_left_side_height = 0.5 * MASTER_SCALE;
    Vertices(
        (0..=1)
            .map(|vert_index| {
                Vertex(
                    do2d_float_from(half_adapter_width, half_adapter_left_side_height)
                        .orient_by_vert_index(vert_index),
                )
            })
            .chain(std::iter::once(Vertex(
                do2d_float_from(half_adapter_width, 0.0).orient_by_vert_index(2),
            )))
            .collect(),
    )
}

fn scale_from_non_zero_vertices(scale_index: usize) -> Vertices {
    let half_adapter_width = 0.25 * MASTER_SCALE;
    let half_adapter_right_side_height = 0.5 * MASTER_SCALE * scale_index as f32;
    let half_adapter_left_side_height = 0.5 * MASTER_SCALE * (1.0 + scale_index as f32);
    Vertices(
        (0..4)
            .map(|vert_index| {
                Vertex(
                    do2d_float_from(
                        half_adapter_width,
                        if [0, 1].contains(&vert_index) {
                            half_adapter_left_side_height
                        } else {
                            half_adapter_right_side_height
                        },
                    )
                    .orient_by_vert_index(vert_index),
                )
            })
            .collect(),
    )
}
