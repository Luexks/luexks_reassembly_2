// use crate::{
//     shapes::{
//         courtesy_port_distribution::CourtesyPortDistribution,
//         port_distribution::default_port_distribution_from_variant, port_flags::PortFlag,
//         port_module::PortModule, scale::Scale, shapes::Shapes, vertex::Vertex, vertices::Vertices,
//     },
//     utility::{
//         angle::Angle,
//         display_oriented_math::{
//             distance_and_angle_to_do2d, do2d_float_from, DisplayOriented2D, DisplayOrientedNumber,
//         },
//         flags::flags,
//     },
// };
// use lazy_static::lazy_static;

// use super::MASTER_SCALE;

// pub const HARDPOINT_SCALE_COUNT: usize = 4;
// pub const HARDPOINT_ANGLE_PER_SIDE: Angle = Angle::Degree(15.0);
// lazy_static! {
//     static ref HARDPOINT_SIDE_COUNT: usize =
//         (360.0 / HARDPOINT_ANGLE_PER_SIDE.as_degrees().get_value()).floor() as usize;
// }

// pub fn add_hardpoints_to_the_shapes(shapes: &mut Shapes) -> usize {
//     shapes.add_unmirrored_shape_from_scales(
//         (1..=HARDPOINT_SCALE_COUNT)
//             .map(|scale_index| scale_from(scale_index))
//             .collect(),
//     );
//     shapes.0.len() - 1
// }

// fn scale_from(scale_index: usize) -> Scale {
//     let port_modules = port_modules();
//     let half_square_length = 0.5 * MASTER_SCALE * (scale_index as f32);
//     let unoriented_do2d = do2d_float_from(half_square_length, half_square_length);
//     let radius = 0.5 * MASTER_SCALE * scale_index as f32;
//     vertices_from(half_square_length, unoriented_do2d, radius)
//         .to_hull_scale_with_modules(port_modules, format!("HardpointS{}", scale_index))
// }

// fn port_modules() -> Vec<Option<PortModule<'static>>> {
//     (0..*HARDPOINT_SIDE_COUNT - 1)
//         .map(|_| PortModule::some(flags!(PortFlag::WeaponOut), default_port_distribution_from_variant!(Single)))
//         .chain(std::iter::once(PortModule::explicit_none()))
//         .chain(std::iter::once(PortModule::no_flags(default_port_distribution_from_variant!(BackwardsFromNextVert: CourtesyPortDistribution::HalfwayToEnd))))
//         .chain(std::iter::once(PortModule::no_flags(default_port_distribution_from_variant!(Center))))
//         .chain(std::iter::once(PortModule::no_flags(default_port_distribution_from_variant!(TowardsFromCurrentVert: CourtesyPortDistribution::HalfwayToEnd))))
//         .chain(std::iter::once(PortModule::explicit_none()))
//         .collect::<Vec<_>>()
// }

// fn vertices_from(
//     half_square_length: f32,
//     unoriented_do2d: DisplayOriented2D,
//     radius: f32,
// ) -> Vertices {
//     Vertices(
//         (0..*HARDPOINT_SIDE_COUNT)
//             .map(|vertex_index| {
//                 let angle = Angle::Degree(
//                     45.0 + -0.25
//                         * HARDPOINT_ANGLE_PER_SIDE.get_value()
//                         * (0.5 + vertex_index as f32),
//                 );
//                 Vertex(distance_and_angle_to_do2d(radius, angle))
//             })
//             .chain(std::iter::once(Vertex(do2d_float_from(
//                 0.0,
//                 -half_square_length,
//             ))))
//             .chain(std::iter::once(Vertex(
//                 unoriented_do2d.orient_by_vert_index(0),
//             )))
//             .chain(std::iter::once(Vertex(
//                 unoriented_do2d.orient_by_vert_index(1),
//             )))
//             .chain(std::iter::once(Vertex(do2d_float_from(
//                 0.0,
//                 half_square_length,
//             ))))
//             .collect(),
//     )
// }
