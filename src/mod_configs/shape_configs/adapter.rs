use crate::{
    shapes::{shapes::Shapes, vertex::Vertex, vertices::Vertices},
    utility::display_oriented_math::do2d_float_from,
};

use super::MASTER_SCALE;

pub const ADAPTER_SCALE_COUNT: usize = 4;

pub fn add_adapters_to_the(shapes: &mut Shapes) -> usize {
    let scale_0 = || {
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
                .chain(vec![Vertex(
                    do2d_float_from(half_adapter_width, 0.0).orient_by_vert_index(2),
                )])
                .collect(),
        )
        .to_hull_scale("AdapterS0".to_string())
    };
    let scale_from = |scale_index: usize| {
        let half_adapter_width = 0.25 * MASTER_SCALE;
        let half_adapter_right_side_height = 0.5 * MASTER_SCALE * scale_index as f32;
        let half_adapter_left_side_height = 0.5 * MASTER_SCALE * (1.0 + scale_index as f32);
        Vertices(
            (0..4)
                .map(|vert_index| {
                    Vertex(
                        do2d_float_from(
                            half_adapter_width,
                            if vert_index <= 1 {
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
        .to_hull_scale(format!("AdapterS{}", scale_index))
    };
    shapes.add_unmirrored_shape_from_scales(
        vec![scale_0()]
            .into_iter()
            .chain(
                (1..=ADAPTER_SCALE_COUNT)
                    .map(|scale_index: usize| scale_from(scale_index))
                    .collect::<Vec<_>>(),
            )
            .collect(),
    );
    shapes.0.len() - 1
}
