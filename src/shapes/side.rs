use crate::mod_configs::shape_configs::shape_configs::*;
use crate::shapes::courtesy_port_distribution::add_courtesy_ports;
use crate::shapes::port::*;
use crate::shapes::port_distribution::PortDistribution;
use crate::shapes::shape_constants::*;
use crate::shapes::vertex::Vertex;
use crate::utility::display_oriented_math::*;

use super::port_module::PortModule;

#[derive(Clone)]
pub struct Side {
    pub index: usize,
    pub vertex_1: Vertex,
    pub vertex_2: Vertex,
}

impl Side {
    pub fn get_side_length(&self) -> f32 {
        ((self.vertex_1.0.x.to_f32() - self.vertex_2.0.x.to_f32()).powi(2)
            + (self.vertex_1.0.y.to_f32() - self.vertex_2.0.y.to_f32()).powi(2))
        .sqrt()
    }

    pub fn to_ports_of(&self, port_module_option: Option<PortModule>) -> Vec<Port> {
        if port_module_option.is_none() {
            return Vec::new();
        }
        let port_module = port_module_option.unwrap();
        if let PortDistribution::Single { position } = &port_module.port_distribution {
            return vec![Port {
                side_index: self.index,
                position: position.clone(),
                flags: port_module.port_flags.clone(),
            }];
        }
        let side_length = self.get_side_length();
        let port_count = if let PortDistribution::UseIntersectingPortsFrom {
            possibly_intersecting_ports,
            ..
        } = port_module.port_distribution
        {
            possibly_intersecting_ports.len() as f32
        } else {
            ((side_length + PORT_COUNT_DECISION_TOLERANCE) / MASTER_SCALE).floor()
        };
        if port_module
            .port_distribution
            .should_add_a_single_halfway_port_to_if_side_length_is_less_than_master_scale()
            && side_length <= MASTER_SCALE
        {
            return vec![halfway_port(self.index)];
        }
        let mut ports: Vec<_> = (0..port_count as usize)
            .filter_map(|port_index| {
                let possible_port_position = get_port_position_of_distribution(
                    &Some(&port_module.port_distribution),
                    &self,
                    &port_count,
                    port_index,
                );
                possible_port_position.map(|port_position| Port {
                    side_index: self.index,
                    position: port_position,
                    flags: port_module.port_flags.clone(),
                })
            })
            .collect();
        add_courtesy_ports!(
            ports: ports,
            side: &self,
            port_count: port_count,
            port_module: port_module
        );
        ports
    }
}

#[rustfmt::skip]
fn get_port_position_of_distribution(
    port_distribution: &Option<&PortDistribution>,
    side: &Side,
    port_count: &f32,
    port_index: usize,
) -> Option<DisplayOrientedNumber> {
    let side_length = side.get_side_length();
    Some(DisplayOrientedNumber::Fraction {
        numerator: Box::new(match port_distribution.unwrap() {
            PortDistribution::JoinWithNext => panic!("Can't get port position of distribution type: join with next."),
            PortDistribution::Center { .. } => don_float_from(
                PortPosition::CENTER * side_length
                    - (PORT_SPACING * (port_count / 2.0 - 0.5))
                    + (PORT_SPACING * port_index as f32),
            ),
            PortDistribution::Single { position } => {
                position.clone()
            },
            PortDistribution::TowardsFromCurrentVert {
                distance_from_current_vert, ..
            } => don_float_from(
                PortPosition::CURRENT_VERT
                    + distance_from_current_vert.to_f32()
                    + (PORT_SPACING * port_index as f32),
            ),
            PortDistribution::BackwardsFromNextVert {
                distance_from_next_vert, ..
            } => don_float_from(
                PortPosition::NEXT_VERT * side_length
                    - distance_from_next_vert.to_f32()
                    - (PORT_SPACING * port_index as f32)
            ),
            PortDistribution::UseIntersectingPortsFrom { side_with_possibly_intersecting_ports, possibly_intersecting_ports } => {
                let possibly_intersecting_port = &possibly_intersecting_ports[port_index];
                let possibly_intersecting_port_do2d = do2d_float_from(
                    side_with_possibly_intersecting_ports.vertex_1.0.x.to_f32() + (side_with_possibly_intersecting_ports.vertex_2.0.x.to_f32() - side_with_possibly_intersecting_ports.vertex_1.0.x.to_f32()) * possibly_intersecting_port.position.to_f32(),
                    side_with_possibly_intersecting_ports.vertex_1.0.y.to_f32() + (side_with_possibly_intersecting_ports.vertex_2.0.y.to_f32() - side_with_possibly_intersecting_ports.vertex_1.0.y.to_f32()) * possibly_intersecting_port.position.to_f32(),
                );
                let intersecting_port_position = if side.vertex_1.0.x.to_f32() == side.vertex_2.0.x.to_f32() {
                    if possibly_intersecting_port_do2d.x.to_f32() != side.vertex_1.0.x.to_f32() {
                        return None;
                    }
                    let intersecting_port_do2d = possibly_intersecting_port_do2d;
                    let intersecting_port_position = don_float_from((intersecting_port_do2d.y.to_f32() - side.vertex_1.0.y.to_f32()) / (side.vertex_2.0.y.to_f32() - side.vertex_1.0.y.to_f32()));
                    if !is_port_position_valid(&intersecting_port_position) {
                        return None;
                    }
                    intersecting_port_position
                } else if side.vertex_1.0.y.to_f32() == side.vertex_2.0.y.to_f32() {
                    if possibly_intersecting_port_do2d.y.to_f32() != side.vertex_1.0.y.to_f32() {
                        return None;
                    }
                    let intersecting_port_do2d = possibly_intersecting_port_do2d;
                    let intersecting_port_position = don_float_from((intersecting_port_do2d.x.to_f32() - side.vertex_1.0.x.to_f32()) / (side.vertex_2.0.x.to_f32() - side.vertex_1.0.x.to_f32()));
                    if !is_port_position_valid(&intersecting_port_position) {
                        return None;
                    }
                    intersecting_port_position
                } else {
                    let line_gradient = (side.vertex_2.0.x.to_f32() - side.vertex_2.0.x.to_f32()) / (side.vertex_2.0.y.to_f32() - side.vertex_2.0.y.to_f32());
                    let line_y_intercept = side.vertex_1.0.y.to_f32() - side.vertex_1.0.x.to_f32() * line_gradient;
                    let line_y = line_gradient * possibly_intersecting_port_do2d.x.to_f32() + line_y_intercept;
                    println!("{}, {}", possibly_intersecting_port_do2d, line_gradient);
                    if possibly_intersecting_port_do2d.y.to_f32() != line_y {
                        return None;
                    }
                    let intersecting_port_do2d = possibly_intersecting_port_do2d;
                    let intersecting_port_position = don_float_from(((intersecting_port_do2d.x.to_f32() - side.vertex_1.0.x.to_f32()).powf(2.0) + (intersecting_port_do2d.y.to_f32() - side.vertex_1.0.y.to_f32()).powf(2.0)).sqrt() / ((side.vertex_2.0.x.to_f32() - side.vertex_1.0.x.to_f32()).powf(2.0) + (side.vertex_2.0.y.to_f32() - side.vertex_1.0.y.to_f32()).powf(2.0)).sqrt());
                    if !is_port_position_valid(&intersecting_port_position) {
                        return None;
                    }
                    intersecting_port_position
                };
                don_float_from(intersecting_port_position.to_f32() * side_length)
            },
        }),
        denominator: Box::new(don_float_from(side_length)),
    })
}
