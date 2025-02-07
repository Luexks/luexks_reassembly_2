use std::f32::consts::SQRT_2;
pub const COMMAND_SCALE_COUNT: usize = 4;

use crate::{
    shapes::{
        port_distribution::{
            default_port_distribution_from_variant, default_port_distribution_from_variants,
        },
        port_module::PortModule,
        shapes::Shapes,
        side::Side,
        vertex::Vertex,
        vertices::Vertices,
    },
    utility::display_oriented_math::do2d_float_from,
};

use super::MASTER_SCALE;

pub fn add_commands_to_the(shapes: &mut Shapes) -> usize {
    let scale_from = |scale_index: usize| {
        let half_square_length = 0.5 * MASTER_SCALE * (scale_index as f32);
        let unoriented_left_corner = do2d_float_from(half_square_length, half_square_length);
        do2d_float_from(half_square_length, half_square_length - 0.001);
        let cut_corner_indent_distance = match scale_index {
            1 => (-0.5 + 1.0 / SQRT_2) * MASTER_SCALE * scale_index as f32,
            _ => 0.25 * MASTER_SCALE * scale_index as f32,
        };
        let vertex_indices_not_on_the_left_side_in_clockwise_order = [4, 5, 6, 7, 0, 1];
        let vertices = Vertices(
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
        );
        let vertex_1_of_side_with_ports_to_intersect_with_the_bottom_side =
            &Vertex(vertices.0.get(0).unwrap().clone().0.orient_by_vert_index(3));
        let vertex_2_of_side_with_ports_to_intersect_with_the_bottom_side =
            &Vertex(vertices.0.get(1).unwrap().clone().0.orient_by_vert_index(0));
        let side_with_ports_to_intersect_with_the_bottom_side = Side {
            vertex_1: vertex_1_of_side_with_ports_to_intersect_with_the_bottom_side,
            vertex_2: vertex_2_of_side_with_ports_to_intersect_with_the_bottom_side,
            index: 0,
        };
        let ports_to_intersect_with_the_bottom_side =
            &side_with_ports_to_intersect_with_the_bottom_side.to_ports_of_module_option(
                PortModule::no_flags(default_port_distribution_from_variant!(Center)),
            );
        let vertex_1_of_side_with_ports_to_intersect_with_the_top_side =
            &Vertex(vertices.0.get(0).unwrap().clone().0.orient_by_vert_index(1));
        let vertex_2_of_side_with_ports_to_intersect_with_the_top_side =
            &Vertex(vertices.0.get(1).unwrap().clone().0.orient_by_vert_index(2));
        let side_with_ports_to_intersect_with_the_top_side = Side {
            vertex_1: vertex_1_of_side_with_ports_to_intersect_with_the_top_side,
            vertex_2: vertex_2_of_side_with_ports_to_intersect_with_the_top_side,
            index: 0,
        };
        let ports_to_intersect_with_the_top_side = &side_with_ports_to_intersect_with_the_top_side
            .to_ports_of_module_option(PortModule::no_flags(
                default_port_distribution_from_variant!(Center),
            ));
        let vertex_1_of_side_with_ports_to_intersect_with_the_right_side =
            &Vertex(vertices.0.get(0).unwrap().clone().0.orient_by_vert_index(2));
        let vertex_2_of_side_with_ports_to_intersect_with_the_right_side =
            &Vertex(vertices.0.get(1).unwrap().clone().0.orient_by_vert_index(3));
        let side_with_ports_to_intersect_with_the_right_side = Side {
            vertex_1: vertex_1_of_side_with_ports_to_intersect_with_the_right_side,
            vertex_2: vertex_2_of_side_with_ports_to_intersect_with_the_right_side,
            index: 0,
        };
        let ports_to_intersect_with_the_right_side =
            &side_with_ports_to_intersect_with_the_right_side.to_ports_of_module_option(
                PortModule::no_flags(default_port_distribution_from_variant!(Center)),
            );
        vertices.to_hull_scale_with_modules(
            PortModule::option_list_from_distributions(default_port_distribution_from_variants!(
                Center,
                JoinWithNext,
                UseIntersectingPortsFrom(
                    &side_with_ports_to_intersect_with_the_top_side,
                    ports_to_intersect_with_the_top_side
                ),
                Center,
                UseIntersectingPortsFrom(
                    &side_with_ports_to_intersect_with_the_right_side,
                    ports_to_intersect_with_the_right_side
                ),
                Center,
                JoinWithNext,
                UseIntersectingPortsFrom(
                    &side_with_ports_to_intersect_with_the_bottom_side,
                    ports_to_intersect_with_the_bottom_side
                ),
            )),
            format!("CommandS{}", scale_index),
        )
    };
    shapes.add_unmirrored_shape_from_scales(
        (1..=COMMAND_SCALE_COUNT)
            .map(|scale_index| scale_from(scale_index))
            .collect(),
    );
    shapes.0.len() - 1
}
