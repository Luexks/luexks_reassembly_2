use crate::shapes::port::Port;
use crate::shapes::port_distribution::PortDistribution;
use crate::shapes::ports::*;
use crate::shapes::scale::*;
use crate::shapes::side::Side;
use crate::shapes::vertex::*;
use crate::utility::display_oriented_math::*;
use crate::utility::flags::Flags;
use std::fmt::Display;

use super::port_distribution::default_port_distribution_from_variant;
use super::port_module::PortModule;

#[derive(Clone, Debug, Default)]
pub struct Vertices(pub Vec<Vertex>);

impl Vertices {
    // pub fn to_hull_scale<F: Fn(usize, f32) -> Vec<Port> + Clone>(self, name: String) -> Scale {
    pub fn to_hull_scale(self, name: String) -> Scale {
        Scale {
            verts: self.clone(),
            ports: Ports(
                self.0
                    .iter()
                    .enumerate()
                    .zip(self.0.iter().cycle().skip(1))
                    .flat_map(|((side_index, vert_1), vert_2)| {
                        Side {
                            index: side_index,
                            vertex_1: vert_1.clone(),
                            vertex_2: vert_2.clone(),
                        }
                        // .to_ports_of(PortModule::<F>::no_flags(
                        .to_ports_of(PortModule::no_flags(
                            default_port_distribution_from_variant!(Center),
                        ))
                    })
                    .collect(),
            ),
            name: name,
        }
    }

    // pub fn to_hull_scale_with_modules<F: Fn(usize, f32) -> Vec<Port> + Clone>(
    pub fn to_hull_scale_with_modules(
        self,
        // port_module_options: Vec<Option<PortModule<F>>>,
        port_module_options: Vec<Option<PortModule>>,
        name: String,
    ) -> Scale {
        assert_eq!(self.0.len(), port_module_options.len());
        Scale {
            verts: self.clone(),
            ports: Ports({
                let mut ports = Vec::new();
                let mut join_with_next_vertices = Vec::new();
                for ((side_index, vertex_1), vertex_2) in
                    self.0.iter().enumerate().zip(self.0.iter().cycle().skip(1))
                {
                    if let Some(PortModule {
                        port_flags: flags,
                        port_distribution,
                    }) = &port_module_options[side_index]
                    {
                        if join_with_next_vertices.is_empty()
                            && !matches!(port_distribution, PortDistribution::JoinWithNext)
                        {
                            ports.extend(
                                Side {
                                    index: side_index,
                                    vertex_1: vertex_1.clone(),
                                    vertex_2: vertex_2.clone(),
                                }
                                .to_ports_of(Some(PortModule {
                                    port_flags: flags.clone(),
                                    port_distribution: port_distribution.clone(),
                                })),
                            );
                        } else if matches!(port_distribution, PortDistribution::JoinWithNext) {
                            join_with_next_vertices.push(vertex_1);
                        } else if !join_with_next_vertices.is_empty()
                            && !matches!(port_distribution, PortDistribution::JoinWithNext)
                        {
                            join_with_next_vertices.push(vertex_1);
                            join_with_next_vertices.push(vertex_2);
                            let entire_side = Side {
                                index: 0, // Null value hehe
                                vertex_1: (*join_with_next_vertices.first().unwrap()).clone(),
                                vertex_2: vertex_2.clone(),
                            };
                            let sub_sides: Vec<_> = join_with_next_vertices
                                .iter()
                                .enumerate()
                                .zip(join_with_next_vertices.iter().skip(1))
                                .map(|((sub_side_index, sub_vertex_1), sub_vertex_2)| Side {
                                    index: 2 + side_index - join_with_next_vertices.len()
                                        + sub_side_index,
                                    vertex_1: (*sub_vertex_1).clone(),
                                    vertex_2: (*sub_vertex_2).clone(),
                                })
                                .collect();
                            let new_undistributed_ports = entire_side
                                .to_ports_of(PortModule::no_flags(port_distribution.clone()));
                            let mut new_distributed_ports = Vec::<Port>::new();
                            for new_undistributed_port in new_undistributed_ports {
                                let mut sub_side_to_distribute_into = None;
                                let mut accumulated_sub_side_lengths_before_new_distributed_port =
                                    0.0;
                                for sub_side in sub_sides.iter() {
                                    accumulated_sub_side_lengths_before_new_distributed_port +=
                                        sub_side.get_side_length();
                                    if accumulated_sub_side_lengths_before_new_distributed_port
                                        >= new_undistributed_port.position.to_f32()
                                            * entire_side.get_side_length()
                                    {
                                        accumulated_sub_side_lengths_before_new_distributed_port -=
                                            sub_side.get_side_length();
                                        sub_side_to_distribute_into = Some(sub_side);
                                        break;
                                    }
                                }
                                let distributed_port = Port {
                                    side_index: sub_side_to_distribute_into.unwrap().index,
                                    position: {
                                        don_fraction_from(
                                            (new_undistributed_port.position.to_f32() * entire_side.get_side_length()) - accumulated_sub_side_lengths_before_new_distributed_port,
                                            sub_side_to_distribute_into.unwrap().get_side_length(),
                                        )
                                    },
                                    flags: Flags::default(),
                                };
                                new_distributed_ports.push(distributed_port);
                            }
                            join_with_next_vertices = Vec::new();
                            ports.extend(new_distributed_ports);
                        } else {
                            panic!()
                        }
                    }
                }
                ports
            }),
            name: name,
        }
    }

    pub fn get_nth_side(&self, side_index: usize) -> Option<Side> {
        if self.0.is_empty() {
            // panic!("What are you doing?");
            return None;
        }

        self.0.get(side_index).map(|vertex_1| {
            let vertex_2 = &self.0[(side_index + 1) % self.0.len()];
            Side {
                vertex_1: vertex_1.clone(),
                vertex_2: vertex_2.clone(),
                index: 0,
            }
        })
    }

    pub fn to_shroud_scale(self) -> Scale {
        Scale {
            verts: self,
            ports: Ports(vec![Port {
                side_index: 0,
                position: don_float_from(0.0),
                flags: Flags(Vec::new()),
            }]),
            name: "".to_string(),
        }
    }

    pub fn get_size(&self) -> DisplayOriented2D {
        let (x_min, x_max, y_min, y_max) = self.0.iter().skip(1).fold(
            (
                self.0[0].0.x.to_f32(),
                self.0[0].0.x.to_f32(),
                self.0[0].0.y.to_f32(),
                self.0[0].0.y.to_f32(),
            ),
            |(x_min, x_max, y_min, y_max), vert| {
                (
                    x_min.min(vert.0.x.to_f32()),
                    x_max.max(vert.0.x.to_f32()),
                    y_min.min(vert.0.y.to_f32()),
                    y_max.max(vert.0.y.to_f32()),
                )
            },
        );
        do2d_float_from(x_max - x_min, y_max - y_min)
    }
}

impl Display for Vertices {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format!(
                "verts={{{}}}",
                self.0
                    .iter()
                    .map(|vert| vert.0.to_string())
                    .collect::<Vec<_>>()
                    .join("")
            )
        )
    }
}
