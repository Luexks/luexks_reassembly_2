// use crate::{
//     shapes::{
//         nomenclature::shape_name, scale::Scale, shapes::Shapes, vertex::Vertex, vertices::Vertices,
//     },
//     utility::display_oriented_math::do2d_float_from,
// };
// use lazy_static::lazy_static;
// use std::f32::consts::SQRT_2;

// use super::MASTER_SCALE;

// pub fn add_rectangles_to_the(shapes: &mut Shapes) -> usize {
//     shapes.add_unmirrored_shape_from_scales(
//         RECTANGLE_SCALE_FACTORS_AND_NAMES
//             .iter()
//             .map(|scale_factor_float_2d_and_name| scale_from(scale_factor_float_2d_and_name))
//             .collect(),
//     );
//     shapes.0.len() - 1
// }

// fn scale_from(scale_factor_float_2d_and_name: &(f32, f32, String)) -> Scale {
//     let unoriented_do2d = do2d_float_from(
//         scale_factor_float_2d_and_name.0 * MASTER_SCALE * 0.5,
//         scale_factor_float_2d_and_name.1 * MASTER_SCALE * 0.5,
//     );
//     Vertices(
//         (0..4)
//             .map(|vert_index| Vertex(unoriented_do2d.orient_by_vert_index(vert_index)))
//             .collect(),
//     )
//     .to_hull_scale(scale_factor_float_2d_and_name.2.clone())
// }

// fn generate_rectangle_scale_factors_and_name(
//     name: &str,
//     scale_count: usize,
//     width_scale_fn: impl Fn(usize) -> f32,
//     height_scale_fn: impl Fn(usize) -> f32,
// ) -> Vec<(f32, f32, String)> {
//     (1..=scale_count)
//         .map(|scale_index| {
//             (
//                 width_scale_fn(scale_index),
//                 height_scale_fn(scale_index),
//                 match scale_count {
//                     1 => name.to_owned(),
//                     _ => shape_name(name, Some(scale_index)),
//                 },
//             )
//         })
//         .collect::<Vec<_>>()
// }

// lazy_static! {
//     static ref QUARTER: Vec<(f32, f32, String)> =
//         generate_rectangle_scale_factors_and_name("1/4rect", 1, |_| 0.25, |_| 1.0,);
//     static ref HALF: Vec<(f32, f32, String)> =
//         generate_rectangle_scale_factors_and_name("1/2rect", 1, |_| 0.5, |_| 1.0,);
//     static ref THREE_QUARTER: Vec<(f32, f32, String)> =
//         generate_rectangle_scale_factors_and_name("3/4rect", 1, |_| 0.75, |_| 1.0,);
//     static ref COMPLEMENTARY_RECIPROCAL_ROOT_2: Vec<(f32, f32, String)> =
//         generate_rectangle_scale_factors_and_name(
//             "ComplementaryReciprocalRoot2rect",
//             4,
//             |_| 1.0,
//             |scale_index| (1.0 - 1.0 / SQRT_2) * scale_index as f32
//         );
//     static ref RECIPROCAL_ROOT_2: Vec<(f32, f32, String)> =
//         generate_rectangle_scale_factors_and_name(
//             "ReciprocalRoot2rect",
//             4,
//             |_| 1.0,
//             |scale_index| scale_index as f32 * (1.0 / SQRT_2),
//         );
//     static ref RECTANGLE_SCALE_FACTORS_AND_NAMES: Vec<(f32, f32, String)> = {
//         QUARTER
//             .iter()
//             .chain(HALF.iter())
//             .chain(THREE_QUARTER.iter())
//             .chain(COMPLEMENTARY_RECIPROCAL_ROOT_2.iter())
//             .chain(RECIPROCAL_ROOT_2.iter())
//             .cloned()
//             .collect::<Vec<_>>()
//     };
// }

// // fn generate_rectangle_scale_factors_and_name(
// //     name: &str,
// //     scale_count: usize,
// //     width_scale_fn: impl Fn(usize) -> f32,
// //     height_scale_fn: impl Fn(usize) -> f32,
// // ) -> Vec<(f32, f32, String)> {
// //     (1..=scale_count)
// //         .map(|scale_index| {
// //             (
// //                 width_scale_fn(scale_index),
// //                 height_scale_fn(scale_index),
// //                 shape_name(name, Some(scale_index)),
// //             )
// //         })
// //         .collect::<Vec<_>>()
// // }

// // lazy_static! {
// //     static ref QUARTER: Vec<(f32, f32, String)> = generate_rectangle_scale_factors_and_name(
// //         "QuarterRect",
// //         4,
// //         |_| 0.25,
// //         |scale_index| 0.25 * scale_index as f32
// //     );
// //     static ref HALF: Vec<(f32, f32, String)> = generate_rectangle_scale_factors_and_name(
// //         "HalfRect",
// //         4,
// //         |_| 0.5,
// //         |scale_index| 0.5 * scale_index as f32
// //     );
// //     static ref COMPLEMENTARY_RECIPROCAL_ROOT_2: Vec<(f32, f32, String)> =
// //         generate_rectangle_scale_factors_and_name(
// //             "ComplementaryReciprocalRoot2rect",
// //             4,
// //             |_| 1.0,
// //             |scale_index| (1.0 - 1.0 / SQRT_2) * scale_index as f32
// //         );
// //     static ref RECIPROCAL_ROOT_2: Vec<(f32, f32, String)> =
// //         generate_rectangle_scale_factors_and_name(
// //             "ReciprocalRoot2rect",
// //             4,
// //             |_| 1.0,
// //             |scale_index| scale_index as f32 * (1.0 / SQRT_2),
// //         );
// //     static ref RECTANGLE_SCALE_FACTORS_AND_NAMES: Vec<(f32, f32, String)> = {
// //         QUARTER
// //             .iter()
// //             .chain(HALF.iter())
// //             .chain(COMPLEMENTARY_RECIPROCAL_ROOT_2.iter())
// //             .chain(RECIPROCAL_ROOT_2.iter())
// //             .cloned()
// //             .collect::<Vec<_>>()
// //     };
// // }
