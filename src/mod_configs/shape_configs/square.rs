// use crate::{
//     shapes::{
//         nomenclature::shape_name, scale::Scale, shapes::Shapes, vertex::Vertex, vertices::Vertices,
//     },
//     utility::display_oriented_math::do2d_float_from,
// };

// use super::MASTER_SCALE;

// pub const SQUARE_SCALE_COUNT: usize = 4;

// pub fn add_squares_to_the(shapes: &mut Shapes) -> usize {
//     shapes.add_unmirrored_shape_from_scales(
//         (1..=SQUARE_SCALE_COUNT)
//             .map(|scale_index| scale_from(scale_index))
//             .collect(),
//     );
//     shapes.0.len() - 1
// }

// fn scale_from(scale_index: usize) -> Scale {
//     let half_square_length = 0.5 * MASTER_SCALE * (scale_index as f32);
//     let unoriented_do2d = do2d_float_from(half_square_length, half_square_length);
//     Vertices(
//         (0..4)
//             .map(|vert_index| Vertex(unoriented_do2d.orient_by_vert_index(vert_index)))
//             .collect(),
//     )
//     .to_hull_scale(shape_name("Square", Some(scale_index)))
// }
