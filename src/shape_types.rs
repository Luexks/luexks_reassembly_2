use crate::configs::*;
use crate::display_oriented_number::*;
use crate::utils::*;
use std::fmt::{self, Display};

macro_rules! scale_from_alternating_vertices_and_port_distributions {
    ($($vertex:expr, $port_distribution_varient:ident),*) => {
        Vertices(
            vec![$($vertex),*]
        )
        .to_hull_scale_with_distributions(
            vec![$(
                default_port_distribution_from_varient!($port_distribution_varient)
            ),*]
        )
    };
}
pub(crate) use scale_from_alternating_vertices_and_port_distributions;

macro_rules! default_port_distribution_from_varient {
    (Center) => {
        PortDistribution::Center
    };
    (TowardsFromCurrentVert) => {
        PortDistribution::TowardsFromCurrentVert {
            distance_from_current_vert: don_float_from(PORT_SPACING_FROM_VERT),
            add_courtesy_port: true,
        }
    };
    (BackwardsFromNextVert) => {
        PortDistribution::BackwardsFromNextVert {
            distance_from_next_vert: don_float_from(PORT_SPACING_FROM_VERT),
            add_courtesy_port: true,
        }
    };
}
pub(crate) use default_port_distribution_from_varient;

macro_rules! add_courtesy_port_to_ports {
    (ports: $ports:expr, side_index: $side_index:expr, side_length: $side_length:expr, port_count: $port_count:expr, port_distribution: $port_distribution:expr) => {
        if $side_length > $port_count * TOTAL_SCALE {
            if let PortDistribution::TowardsFromCurrentVert { .. } = $port_distribution {
                $ports.0.push(Port {
                    side_index: $side_index,
                    position: DisplayOrientedNumber::Fraction {
                        numerator: Box::new(don_float_from(
                            PortPosition::CURRENT_VERT * $side_length
                                + ($side_length + $port_count * TOTAL_SCALE) * 0.5,
                        )),
                        denominator: Box::new(don_float_from($side_length)),
                    },
                    flags: Flags::<PortFlag>::default(),
                })
            } else if let PortDistribution::BackwardsFromNextVert { .. } = $port_distribution {
                $ports.0.push(Port {
                    side_index: $side_index,
                    position: DisplayOrientedNumber::Fraction {
                        numerator: Box::new(don_float_from(
                            PortPosition::NEXT_VERT * $side_length
                                - ($side_length + $port_count * TOTAL_SCALE) * 0.5,
                        )),
                        denominator: Box::new(don_float_from($side_length)),
                    },
                    flags: Flags::<PortFlag>::default(),
                })
            } else if let PortDistribution::Center = $port_distribution {
                panic!("A side with port distribution center cannot have a courtesy port.")
            }
        }
    };
}

pub enum PortDistribution {
    Center,
    TowardsFromCurrentVert {
        distance_from_current_vert: DisplayOrientedNumber,
        add_courtesy_port: bool,
    },
    BackwardsFromNextVert {
        distance_from_next_vert: DisplayOrientedNumber,
        add_courtesy_port: bool,
    },
}

struct Side<'a> {
    side_index: usize,
    vert_1: &'a Vertex,
    vert_2: &'a Vertex,
}

impl<'a> Side<'_> {
    fn get_side_length(&self) -> f32 {
        ((self.vert_1.0.x.to_f32() - self.vert_2.0.x.to_f32()).powi(2)
            + (self.vert_1.0.y.to_f32() - self.vert_2.0.y.to_f32()).powi(2))
        .sqrt()
    }

    fn to_ports_of_distribution(self, port_distribution: &PortDistribution) -> Ports {
        let side_length = self.get_side_length();
        let port_count = ((side_length + PORT_COUNT_DECISION_TOLERANCE) / TOTAL_SCALE).floor();
        if side_length <= TOTAL_SCALE {
            Ports(vec![Port {
                side_index: self.side_index,
                position: DisplayOrientedNumber::Float(PortPosition::CENTER),
                flags: Flags::<PortFlag>::default(),
            }])
        } else {
            let mut ports = Ports(
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
            );
            match port_distribution {
                PortDistribution::Center => (),
                PortDistribution::TowardsFromCurrentVert {
                    add_courtesy_port, ..
                } => {
                    if *add_courtesy_port {
                        add_courtesy_port_to_ports!(
                            ports: ports,
                            side_index: self.side_index,
                            side_length: side_length,
                            port_count: port_count,
                            port_distribution: port_distribution
                        )
                    }
                }
                PortDistribution::BackwardsFromNextVert {
                    add_courtesy_port, ..
                } => {
                    if *add_courtesy_port {
                        add_courtesy_port_to_ports!(
                            ports: ports,
                            side_index: self.side_index,
                            side_length: side_length,
                            port_count: port_count,
                            port_distribution: port_distribution
                        )
                    }
                }
            };
            ports
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
            PortDistribution::Center => don_float_from(
                PortPosition::CENTER * side_length
                    - (PORT_SPACING * (port_count / 2.0 - 0.5))
                    + (PORT_SPACING * port_index as f32),
            ),
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
        }),
        denominator: Box::new(don_float_from(*side_length)),
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

impl Shapes {
    pub fn add_unmirrored_shape_from_scales(&mut self, scales: Vec<Scale>) {
        self.0.push(Shape::Standard {
            id: ShapeId::next(),
            scales: scales,
        });
    }

    pub fn add_mirrored_shape_from_scales(&mut self, scales: Vec<Scale>) {
        let new_shape = Shape::Standard {
            id: ShapeId::next(),
            scales: scales,
        };
        let mirrored_new_shape = new_shape.mirrored();

        self.0.push(new_shape);
        self.0.push(mirrored_new_shape);
    }
}
pub enum Shape {
    Standard {
        id: ShapeId,
        scales: Vec<Scale>,
    },
    Mirror {
        id: ShapeId,
        mirror_of: ShapeId,
        scale_count: usize,
    },
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Shape::Standard { id, scales } => format!(
                    "{{{}{{\n{}\n}}}}",
                    id,
                    scales
                        .iter()
                        .map(|scale| scale.to_string())
                        .collect::<Vec<_>>()
                        .join("\n")
                ),
                Shape::Mirror {
                    id,
                    mirror_of,
                    scale_count: _,
                } => format!("{{{},{{}},mirror_of={}}}", id, mirror_of),
            }
        )
    }
}

impl Shape {
    pub fn mirrored(&self) -> Self {
        let mirror_of = match self {
            Shape::Standard { id, .. } => id,
            Shape::Mirror { .. } => panic!(),
        };
        let scale_count = match self {
            Shape::Standard { id: _, scales } => scales.len(),
            Shape::Mirror { .. } => panic!(),
        };
        Shape::Mirror {
            id: ShapeId::next(),
            mirror_of: *mirror_of,
            scale_count: scale_count,
        }
    }
    pub fn with_mirror(self) -> Vec<Shape> {
        let mirrored = self.mirrored();
        vec![self, mirrored]
    }

    pub fn get_id(&self) -> Option<ShapeId> {
        Some(match self {
            Shape::Standard { id, .. } => *id,
            Shape::Mirror { id, .. } => *id,
        })
    }

    pub fn get_scale_count(&self) -> usize {
        match self {
            Shape::Standard { scales, .. } => scales.len(),
            Shape::Mirror { scale_count, .. } => *scale_count,
        }
    }
}

pub struct Scale {
    verts: Vertices,
    ports: Ports,
}

impl Display for Scale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{}{}}}", self.verts, self.ports)
    }
}

#[derive(Clone)]
pub struct Vertices(pub Vec<Vertex>);

impl Vertices {
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
                        .to_ports_of_distribution(&PortDistribution::Center)
                        .0
                    })
                    .collect(),
            ),
        }
    }

    pub fn to_hull_scale_with_distributions(
        self,
        port_distributions: Vec<PortDistribution>,
    ) -> Scale {
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
                        .to_ports_of_distribution(&port_distributions[side_index])
                        .0
                    })
                    .collect(),
            ),
        }
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

#[derive(Clone)]
pub struct Vertex(pub DisplayOriented2D);
#[derive(Clone)]
pub struct Ports(Vec<Port>);

impl Display for Ports {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        if !FUNKY_PORT_FORMATING {
            write!(
                f,
                "ports={{{}}}",
                self.0
                    .iter()
                    .map(|port| format!("{}", port))
                    .collect::<Vec<_>>()
                    .join("")
            )
        } else {
            let mut ports = self.0.clone();
            ports.sort_by(|port_a, port_b| port_a.side_index.cmp(&port_b.side_index));
            let side_sorted_port_matrix = side_sorted_port_matrix_from(ports);
            let displayed_row_count = side_sorted_port_matrix
                .iter()
                .map(|row| row.len())
                .max()
                .unwrap_or(0);
            let funky_output = funky_output_from_side_sorted_port_matrix(
                displayed_row_count,
                side_sorted_port_matrix,
            );
            write!(f, "ports={{\n{}}}", funky_output)
        }
    }
}

fn side_sorted_port_matrix_from(ports: Vec<Port>) -> Vec<Vec<Port>> {
    let mut side_sorted_port_matrix: Vec<Vec<Port>> = Vec::new();
    let mut current_side_index = None;
    for port in ports {
        if Some(port.side_index) != current_side_index {
            side_sorted_port_matrix.push(Vec::new());
        }

        current_side_index = Some(port.side_index);
        side_sorted_port_matrix.last_mut().unwrap().push(port);
    }
    side_sorted_port_matrix
}

fn funky_output_from_side_sorted_port_matrix(
    displayed_row_count: usize,
    side_sorted_port_matrix: Vec<Vec<Port>>,
) -> String {
    (0..displayed_row_count)
        .map(|row_index| {
            side_sorted_port_matrix
                .iter()
                .map(|column| {
                    column
                        .get(row_index)
                        .map_or(" ".repeat(30), |port| format!("{:<30}", port.to_string()))
                })
                .collect::<Vec<_>>()
                .join("")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[derive(Clone)]
pub struct Port {
    side_index: usize,
    position: DisplayOrientedNumber,
    flags: Flags<PortFlag>,
}

impl Display for Port {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{},{}{}}}", self.side_index, self.position, self.flags)
    }
}

#[derive(Clone)]
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
