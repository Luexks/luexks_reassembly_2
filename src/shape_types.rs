use crate::configs::*;
use crate::display_oriented_number::*;
use crate::utils::*;
use std::fmt::{self, Display};

enum PortDistribution {
    Center,
    TowardsFromCurrentVert {
        distance_from_current_vert: DisplayOrientedNumber,
    },
    BackwardsFromNextVert {
        distance_from_next_vert: DisplayOrientedNumber,
    },
}

struct Side<'a> {
    side_index: usize,
    vert_1: &'a Vert,
    vert_2: &'a Vert,
}

impl<'a> Side<'_> {
    fn get_side_length(&self) -> f32 {
        ((self.vert_1.0.x.to_f32() - self.vert_2.0.x.to_f32()).powi(2)
            + (self.vert_1.0.y.to_f32() - self.vert_2.0.y.to_f32()).powi(2))
        .sqrt()
    }

    fn to_ports_of_distribution(self, port_distribution: PortDistribution) -> Ports {
        let side_length = self.get_side_length();
        let port_count = ((side_length + PORT_COUNT_DECISION_TOLERANCE) / TOTAL_SCALE).floor();
        if port_count <= 1.0 {
            Ports(vec![Port {
                side_index: self.side_index,
                position: DisplayOrientedNumber::Float(PortPosition::CENTER),
                flags: Flags::<PortFlag>::default(),
            }])
        } else {
            Ports(
                (0..port_count as usize)
                    .map(|port_index| Port {
                        side_index: self.side_index,
                        position: get_port_position_of_distribution(
                            &port_distribution,
                            &side_length,
                            &port_count,
                            port_index,
                        ),
                        flags: Flags::<PortFlag>::default(),
                    })
                    .collect(),
            )
        }
    }
}

#[rustfmt::skip]
fn get_port_position_of_distribution(
    port_distribution: &PortDistribution,
    side_length: &f32,
    port_count: &f32,
    port_index: usize,
) -> DisplayOrientedNumber {
    DisplayOrientedNumber::Fraction {
        numerator: Box::new(match &port_distribution {
            PortDistribution::Center => DisplayOrientedNumber::Float(
                PortPosition::CENTER * side_length
                    - (PORT_SPACING * (port_count / 2.0 - 0.5))
                    + (PORT_SPACING * port_index as f32),
            ),
            PortDistribution::TowardsFromCurrentVert {
                distance_from_current_vert,
            } => DisplayOrientedNumber::Float(
                PortPosition::CURRENT_VERT
                    + distance_from_current_vert.to_f32()
                    + (PORT_SPACING * port_index as f32),
            ),
            PortDistribution::BackwardsFromNextVert {
                distance_from_next_vert,
            } => DisplayOrientedNumber::Float(
                PortPosition::NEXT_VERT
                    - distance_from_next_vert.to_f32()
                    - (PORT_SPACING * port_index as f32),
            ),
        }),
        denominator: Box::new(DisplayOrientedNumber::Float(*side_length)),
    }
}

pub struct Shapes(pub Vec<Shape>);

impl Default for Shapes {
    fn default() -> Self {
        Shapes(Vec::new())
    }
}

impl Display for Shapes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{\n{}\n}}",
            self.0
                .iter()
                .map(|shape| shape.to_string())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

pub struct Shape {
    pub id: ShapeId,
    pub scales: Vec<Scale>,
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format!(
                "{{{}{{\n{}\n}}}}",
                self.id,
                self.scales
                    .iter()
                    .map(|scale| scale.to_string())
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        )
    }
}

pub struct Scale {
    verts: Verts,
    ports: Ports,
}

impl Display for Scale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{}{}}}", self.verts, self.ports)
    }
}

#[derive(Clone)]
pub struct Verts(pub Vec<Vert>);

impl Verts {
    pub fn to_hull_scale(self) -> Scale {
        Scale {
            verts: self.clone(),
            ports: Ports(
                self.0
                    .iter()
                    .enumerate()
                    .zip(self.0.iter().cycle().skip(1))
                    .flat_map(|((side_index, vert_1), vert_2)| {
                        Side {
                            side_index: side_index,
                            vert_1: vert_1,
                            vert_2: vert_2,
                        }
                        .to_ports_of_distribution(PortDistribution::Center)
                        .0
                    })
                    .collect(),
            ),
        }
    }
}

impl Display for Verts {
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

#[derive(Clone)]
pub struct Vert(pub DisplayOriented2D);
pub struct Ports(Vec<Port>);

impl Display for Ports {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            format!(
                "ports={{{}}}",
                self.0
                    .iter()
                    .map(|port| format!("{}", port))
                    .collect::<Vec<_>>()
                    .join("")
            )
        )
    }
}

pub struct Port {
    side_index: usize,
    position: DisplayOrientedNumber,
    flags: Flags<PortFlag>,
}

impl Display for Port {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            format!("{{{},{}{}}}", self.side_index, self.position, self.flags)
        )
    }
}

pub enum PortFlag {
    ThrusterOut,
    ThrusterIn,
    WeaponOut,
    WeaponIn,
    Launcher,
    Missile,
    Root,
    None,
    Normal,
}

impl Display for PortFlag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PortFlag::ThrusterOut => "THRUSTER_OUT",
                PortFlag::ThrusterIn => "THRUSTER_IN",
                PortFlag::WeaponOut => "WEAPON_OUT",
                PortFlag::WeaponIn => "WEAPON_IN",
                PortFlag::Launcher => "LAUNCHER",
                PortFlag::Missile => "MISSILE",
                PortFlag::Root => "ROOT",
                PortFlag::None => "NONE",
                PortFlag::Normal => "NORMAL",
            }
        )
    }
}
