use crate::{
    shapes::{shapes::Shapes, vertex::Vertex, vertices::Vertices},
    utility::display_oriented_math::do2d_float_from,
};

use super::MASTER_SCALE;

pub const RECT_LONG_WIDTH_SCALE_FACTORS: [f32; 2] = [1.0, 2.0];
pub const RECT_LONG_HEIGHT_SCALE_FACTORS: [f32; 4] = [1.0, 2.0, 3.0, 4.0];

pub fn add_rect_longs_to_the(shapes: &mut Shapes) -> usize {
    let scale_from = |width_scale_factor: f32, height_scale_factor: f32| {
        let half_rect_long_width = 0.5 * MASTER_SCALE * (width_scale_factor as f32);
        let half_rect_long_height = half_rect_long_width * (height_scale_factor as f32);
        let unoriented_do2d = do2d_float_from(half_rect_long_width, half_rect_long_height);
        Vertices(
            (0..4)
                .map(|vert_index| Vertex(unoriented_do2d.orient_by_vert_index(vert_index)))
                .collect(),
        )
        .to_hull_scale(format!(
            "{}x{}",
            width_scale_factor,
            height_scale_factor as f32 * width_scale_factor as f32
        ))
    };
    shapes.add_unmirrored_shape_from_scales(
        RECT_LONG_WIDTH_SCALE_FACTORS
            .iter()
            .flat_map(|&width_scale_factor| {
                RECT_LONG_HEIGHT_SCALE_FACTORS
                    .iter()
                    .map(move |&height_scale_factor| {
                        scale_from(width_scale_factor, height_scale_factor)
                    })
            })
            .collect::<Vec<_>>(),
    );
    shapes.0.len() - 1
}
