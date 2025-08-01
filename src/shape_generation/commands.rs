use std::f32::consts::SQRT_2;
pub const COMMAND_SCALE_COUNT: usize = 4;

use crate::{
    shapes::{
        nomenclature::shape_name,
        port::Port,
        port_distribution::{
            default_port_distribution_from_variant, default_port_distribution_from_variants,
        },
        port_module::PortModule,
        scale::Scale,
        shapes::Shapes,
        side::Side,
        vertex::Vertex,
        vertices::Vertices,
    },
    utility::display_oriented_math::do2d_float_from,
};

use super::MASTER_SCALE;

pub fn add_commands_to_the(shapes: &mut Shapes, scale_count: usize) -> usize {
    shapes.add_unmirrored_shape_from_scales(get_command_scales(scale_count));
    shapes.0.len() - 1
}

pub fn get_command_scales(scale_count: usize) -> Vec<Scale> {
    (1..=scale_count)
        .map(|scale_index| scale_from(scale_index))
        .collect()
}

fn scale_from(scale_index: usize) -> Scale {
    let vertices = vertices_from(scale_index);
    let (
        (bottom_intersecting_side, bottom_intersecting_ports),
        (top_intersecting_side, top_intersecting_ports),
        (right_intersecting_side, right_intersecting_ports),
    ) = intersecting_data(&vertices);

    vertices.clone().to_hull_scale_with_modules(
        PortModule::option_list_from_distributions(default_port_distribution_from_variants!(
            Center,
            JoinWithNext,
            UseIntersectingPortsFrom(top_intersecting_side, &top_intersecting_ports),
            Center,
            UseIntersectingPortsFrom(right_intersecting_side, &right_intersecting_ports),
            Center,
            JoinWithNext,
            UseIntersectingPortsFrom(bottom_intersecting_side, &bottom_intersecting_ports),
        )),
        shape_name("Command", Some(scale_index)),
    )
}

fn vertices_from(scale_index: usize) -> Vertices {
    let half_square_length = 0.5 * MASTER_SCALE * (scale_index as f32);
    let unoriented_left_corner = do2d_float_from(half_square_length, half_square_length);
    let cut_corner_indent_distance = MASTER_SCALE
        * match scale_index {
            1 => -0.5 + 1.0 / SQRT_2,
            _ => 0.25 * scale_index as f32,
        };
    let vertex_indices_not_on_the_left_side_in_clockwise_order = [4, 5, 6, 7, 0, 1];
    Vertices(
        (0..=1)
            .map(|vert_index| Vertex(unoriented_left_corner.orient_by_vert_index(vert_index)))
            .chain(
                vertex_indices_not_on_the_left_side_in_clockwise_order
                    .iter()
                    .map(|vert_index| {
                        Vertex(
                            do2d_float_from(
                                half_square_length,
                                cut_corner_indent_distance
                                    * if vert_index % 2 == 0 { -1.0 } else { 1.0 },
                            )
                            .rotate_by_vert_index((*vert_index as i32 / 2) as usize),
                        )
                    })
                    .collect::<Vec<_>>(),
            )
            .collect(),
    )
}

fn intersecting_data(
    vertices: &Vertices,
) -> ((Side, Vec<Port>), (Side, Vec<Port>), (Side, Vec<Port>)) {
    let bottom_intersecting_side_vertex_1 =
        Vertex(vertices.0.get(0).unwrap().0.orient_by_vert_index(3));
    let bottom_intersecting_side_vertex_2 =
        Vertex(vertices.0.get(1).unwrap().0.orient_by_vert_index(0));
    let bottom_intersecting_side = Side {
        vertex_1: bottom_intersecting_side_vertex_1,
        vertex_2: bottom_intersecting_side_vertex_2,
        index: 0,
    };
    let bottom_intersecting_ports = bottom_intersecting_side.to_ports_of(PortModule::no_flags(
        default_port_distribution_from_variant!(Center),
    ));

    let top_intersecting_side_vertex_1 =
        Vertex(vertices.0.get(0).unwrap().0.orient_by_vert_index(1));
    let top_intersecting_side_vertex_2 =
        Vertex(vertices.0.get(1).unwrap().0.orient_by_vert_index(2));
    let top_intersecting_side = Side {
        vertex_1: top_intersecting_side_vertex_1,
        vertex_2: top_intersecting_side_vertex_2,
        index: 0,
    };
    let top_intersecting_ports = top_intersecting_side.to_ports_of(PortModule::no_flags(
        default_port_distribution_from_variant!(Center),
    ));

    let bottom_intersecting_side_vertex_1 =
        Vertex(vertices.0.get(0).unwrap().0.orient_by_vert_index(2));
    let bottom_intersecting_side_vertex_2 =
        Vertex(vertices.0.get(1).unwrap().0.orient_by_vert_index(3));
    let right_intersecting_side = Side {
        vertex_1: bottom_intersecting_side_vertex_1,
        vertex_2: bottom_intersecting_side_vertex_2,
        index: 0,
    };
    let right_intersecting_ports = right_intersecting_side.to_ports_of(PortModule::no_flags(
        default_port_distribution_from_variant!(Center),
    ));

    (
        (bottom_intersecting_side, bottom_intersecting_ports),
        (top_intersecting_side, top_intersecting_ports),
        (right_intersecting_side, right_intersecting_ports),
    )
}
