use crate::{
    shapes::{
        courtesy_port_distribution::CourtesyPortDistribution,
        port_distribution::default_port_distribution_from_variant, port_flags::PortFlag,
        port_module::PortModule, scale::Scale, shapes::Shapes, vertex::Vertex, vertices::Vertices,
    },
    utility::{
        angle::Angle,
        display_oriented_math::{
            distance_and_angle_to_do2d, do2d_float_from, DisplayOriented2D,
        },
        flags::flags,
    },
};

use super::MASTER_SCALE;

pub fn add_hardpoints_to_the_shapes(shapes: &mut Shapes, hardpoint_angle_per_side: Angle, hardpoint_scale_count: usize) -> usize {
    shapes.add_unmirrored_shape_from_scales(
        (1..=hardpoint_scale_count)
            .map(|scale_index| scale_from(scale_index, hardpoint_angle_per_side.clone()))
            .collect(),
    );
    shapes.0.len() - 1
}

fn scale_from(scale_index: usize, hardpoint_angle_per_side: Angle) -> Scale {
    let hardpoint_side_count = (360.0 / hardpoint_angle_per_side.as_degrees().get_value()).floor() as usize;
    let port_modules = port_modules(hardpoint_side_count);
    let half_square_length = 0.5 * MASTER_SCALE * (scale_index as f32);
    let unoriented_do2d = do2d_float_from(half_square_length, half_square_length);
    let radius = 0.5 * MASTER_SCALE * scale_index as f32;
    vertices_from(half_square_length, unoriented_do2d, radius, hardpoint_side_count, hardpoint_angle_per_side)
        .to_hull_scale_with_modules(port_modules, format!("HardpointS{}", scale_index))
}

fn port_modules(hardpoint_side_count: usize) -> Vec<Option<PortModule<'static>>> {
    (0..hardpoint_side_count - 1)
        .map(|_| PortModule::some(flags!(PortFlag::WeaponOut), default_port_distribution_from_variant!(Single)))
        .chain(std::iter::once(PortModule::explicit_none()))
        .chain(std::iter::once(PortModule::no_flags(default_port_distribution_from_variant!(BackwardsFromNextVert: CourtesyPortDistribution::HalfwayToEnd))))
        .chain(std::iter::once(PortModule::no_flags(default_port_distribution_from_variant!(Center))))
        .chain(std::iter::once(PortModule::no_flags(default_port_distribution_from_variant!(TowardsFromCurrentVert: CourtesyPortDistribution::HalfwayToEnd))))
        .chain(std::iter::once(PortModule::explicit_none()))
        .collect::<Vec<_>>()
}

fn vertices_from(
    half_square_length: f32,
    unoriented_do2d: DisplayOriented2D,
    radius: f32,
    hardpoint_side_count: usize,
    hardpoint_angle_per_side: Angle,
) -> Vertices {
    Vertices(
        (0..hardpoint_side_count)
            .map(|vertex_index| {
                let angle = Angle::Degree(
                    45.0 + -0.25
                        * hardpoint_angle_per_side.get_value()
                        * (0.5 + vertex_index as f32),
                );
                Vertex(distance_and_angle_to_do2d(radius, angle))
            })
            .chain(std::iter::once(Vertex(do2d_float_from(
                0.0,
                -half_square_length,
            ))))
            .chain(std::iter::once(Vertex(
                unoriented_do2d.orient_by_vert_index(0),
            )))
            .chain(std::iter::once(Vertex(
                unoriented_do2d.orient_by_vert_index(1),
            )))
            .chain(std::iter::once(Vertex(do2d_float_from(
                0.0,
                half_square_length,
            ))))
            .collect(),
    )
}
