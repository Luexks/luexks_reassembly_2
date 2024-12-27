use crate::display_oriented_number::*;
use crate::shape_configs::{FUNKY_PORT_FORMATING, MASTER_SCALE, PORT_SPACING};
use crate::utils::*;
use std::fmt::{self, Display};

macro_rules! scale_from_alternating_vertices_and_port_distributions {
    (name: $name:expr, $($vertex:expr, $port_distribution_variant:ident $(: $last_port_distribution:expr)?),*,) => {
        Vertices(
            vec![$($vertex),*]
        )
        .to_hull_scale_with_distributions(
            default_port_distribution_from_variants!(
                $($port_distribution_variant $(: $last_port_distribution)?),*),
            $name
        )
    };
}
pub(crate) use scale_from_alternating_vertices_and_port_distributions;

macro_rules! default_port_distribution_from_variants {
    (None) => {
        vec![$(default_port_distribution_from_variant!(None)),*]
    };
    ($($port_distribution_variant:ident $(: $last_port_distribution:expr)?),* $(,)?) => {
        vec![$(default_port_distribution_from_variant!($port_distribution_variant $(: $last_port_distribution)?)),*]
    };
}
pub(crate) use default_port_distribution_from_variants;

macro_rules! default_port_distribution_from_variant {
    (None) => {
        None
    };
    (Center) => {
        Some(PortDistribution::Center {
            courtesy_port_distribution_option: None,
        })
    };
    (Center: $courtesy_port_distribution:expr) => {
        Some(PortDistribution::Center {
            courtesy_port_distribution_option: None,
        })
    };
    (TowardsFromCurrentVert) => {
        Some(PortDistribution::TowardsFromCurrentVert {
            distance_from_current_vert: don_float_from(PORT_SPACING_FROM_VERT),
            courtesy_port_distribution_option: None,
        })
    };
    (TowardsFromCurrentVert: $courtesy_port_distribution_option:expr) => {
        Some(PortDistribution::TowardsFromCurrentVert {
            distance_from_current_vert: don_float_from(PORT_SPACING_FROM_VERT),
            courtesy_port_distribution_option: $courtesy_port_distribution_option,
        })
    };
    (BackwardsFromNextVert) => {
        Some(PortDistribution::BackwardsFromNextVert {
            distance_from_next_vert: don_float_from(PORT_SPACING_FROM_VERT),
            courtesy_port_distribution_option: None,
        })
    };
    (BackwardsFromNextVert: $courtesy_port_distribution_option:expr) => {
        Some(PortDistribution::BackwardsFromNextVert {
            distance_from_next_vert: don_float_from(PORT_SPACING_FROM_VERT),
            courtesy_port_distribution_option: $courtesy_port_distribution_option,
        })
    };
    (JoinWithNext) => {
        Some(PortDistribution::JoinWithNext)
    };
}
pub(crate) use default_port_distribution_from_variant;

macro_rules! add_courtesy_ports {
    (ports: $ports:expr, side_index: $side_index:expr, side_length: $side_length:expr, port_count: $port_count:expr, port_distribution_option: $port_distribution_option:expr) => {
        if let Some(port_distribution) = $port_distribution_option {
            if $side_length > $port_count * MASTER_SCALE {
                if let PortDistribution::TowardsFromCurrentVert { courtesy_port_distribution_option, .. } = port_distribution {
                    if let Some(courtesy_port_distribution) = courtesy_port_distribution_option {
                        match courtesy_port_distribution {}
                    }
                    $ports.push(Port {
                        side_index: $side_index,
                        position: DisplayOrientedNumber::Fraction {
                            numerator: Box::new(don_float_from(
                                PortPosition::CURRENT_VERT * $side_length
                                    + ($side_length + $port_count * MASTER_SCALE) * 0.5,
                            )),
                            denominator: Box::new(don_float_from($side_length)),
                        },
                        flags: Flags::<PortFlag>::default(),
                    });
                } else if let PortDistribution::BackwardsFromNextVert { courtesy_port_distribution_option, .. } = port_distribution {
                    $ports.push(Port {
                        side_index: $side_index,
                        position: DisplayOrientedNumber::Fraction {
                            numerator: Box::new(don_float_from(
                                PortPosition::NEXT_VERT * $side_length
                                    - ($side_length + $port_count * MASTER_SCALE) * 0.5,
                            )),
                            denominator: Box::new(don_float_from($side_length)),
                        },
                        flags: Flags::<PortFlag>::default(),
                    });
                } else if let PortDistribution::Center { courtesy_port_distribution_option } = port_distribution {
                }
            }
        }
    };
}
pub enum PortDistribution {
    Center {
        courtesy_port_distribution_option: Option<CourtesyPortDistribution>,
    },
    TowardsFromCurrentVert {
        distance_from_current_vert: DisplayOrientedNumber,
        courtesy_port_distribution_option: Option<CourtesyPortDistribution>,
    },
    BackwardsFromNextVert {
        distance_from_next_vert: DisplayOrientedNumber,
        courtesy_port_distribution_option: Option<CourtesyPortDistribution>,
    },
    JoinWithNext,
}

pub enum CourtesyPortDistribution {
    Halfway,
    ContinuePattern,
}

struct Side<'a> {
    side_index: usize,
    vertex_1: &'a Vertex,
    vertex_2: &'a Vertex,
}

impl<'a> Side<'_> {
    fn get_side_length(&self) -> f32 {
        ((self.vertex_1.0.x.to_f32() - self.vertex_2.0.x.to_f32()).powi(2)
            + (self.vertex_1.0.y.to_f32() - self.vertex_2.0.y.to_f32()).powi(2))
        .sqrt()
    }

    fn to_ports_of_distribution(&self, port_distribution: Option<&PortDistribution>) -> Vec<Port> {
        if port_distribution.is_none() {
            return Vec::new();
        }
        let side_length = self.get_side_length();
        let port_count = ((side_length + PORT_COUNT_DECISION_TOLERANCE) / MASTER_SCALE).floor();
        if side_length <= MASTER_SCALE {
            vec![Port {
                side_index: self.side_index,
                position: DisplayOrientedNumber::Float(PortPosition::CENTER),
                flags: Flags::<PortFlag>::default(),
            }]
        } else {
            let mut ports: Vec<_> = (0..port_count as usize)
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
                .collect();
            add_courtesy_ports!(
                ports: ports,
                side_index: self.side_index,
                side_length: side_length,
                port_count: port_count,
                port_distribution_option: port_distribution
            );
            // match port_distribution.unwrap() {
            //     PortDistribution::JoinWithNext => {
            //         panic!("Can't get port position of distribution type: join with next.")
            //     }
            //     PortDistribution::Center { .. } => (),
            //     PortDistribution::TowardsFromCurrentVert {
            //         courtesy_port_distribution,
            //         ..
            //     } => {
            //         if let Some(courtesy_port_distribution) = courtesy_port_distribution {
            //             match courtesy_port_distribution {
            //                 CourtesyPortDistribution::Halfway => add_courtesy_ports!(
            //                     ports: ports,
            //                     side_index: self.side_index,
            //                     side_length: side_length,
            //                     port_count: port_count,
            //                     port_distribution: port_distribution
            //                 ),
            //                 CourtesyPortDistribution::ContinuePattern => add_courtesy_ports!(
            //                     ports: ports,
            //                     side_index: self.side_index,
            //                     side_length: side_length,
            //                     port_count: port_count,
            //                     port_distribution: port_distribution
            //                 ),
            //             }
            //         }
            //     }
            //     PortDistribution::BackwardsFromNextVert {
            //         courtesy_port_distribution,
            //         ..
            //     } => {
            //         if let Some(courtesy_port_distribution) = courtesy_port_distribution {
            //             match courtesy_port_distribution {
            //                 CourtesyPortDistribution::Halfway => add_courtesy_ports!(
            //                     ports: ports,
            //                     side_index: self.side_index,
            //                     side_length: side_length,
            //                     port_count: port_count,
            //                     port_distribution: port_distribution
            //                 ),
            //                 CourtesyPortDistribution::ContinuePattern => add_courtesy_ports!(
            //                     ports: ports,
            //                     side_index: self.side_index,
            //                     side_length: side_length,
            //                     port_count: port_count,
            //                     port_distribution: port_distribution
            //                 ),
            //             }
            //         }
            //     }
            // };
            ports
        }
    }
}

#[rustfmt::skip]
fn get_port_position_of_distribution(
    port_distribution: &Option<&PortDistribution>,
    side_length: &f32,
    port_count: &f32,
    port_index: usize,
) -> DisplayOrientedNumber {
    DisplayOrientedNumber::Fraction {
        numerator: Box::new(match port_distribution.unwrap() {
            PortDistribution::JoinWithNext => panic!("Can't get port position of distribution type: join with next."),
            PortDistribution::Center { .. } => don_float_from(
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
        let [new_shape, mirrored_new_shape] = new_shape.with_mirror();

        self.0.push(new_shape);
        self.0.push(mirrored_new_shape);
    }
}

#[derive(Clone)]
pub enum Shape {
    Standard {
        id: ShapeId,
        scales: Vec<Scale>,
    },
    Mirror {
        id: ShapeId,
        mirror_of: ShapeId,
        scale_count: usize,
        scale_names: Vec<String>,
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
                Shape::Mirror { id, mirror_of, .. } =>
                    format!("{{{},{{}},mirror_of={}}}", id, mirror_of),
            }
        )
    }
}

impl Shape {
    pub fn mirrored(&self) -> Self {
        let mirror_of: &ShapeId;
        let scale_count;
        let scale_names;
        match self {
            Shape::Standard { id, scales } => {
                mirror_of = id;
                scale_count = scales.len();
                scale_names = scales
                    .iter()
                    .map(|scale| format!("{}R", scale.name.clone()))
                    .collect();
            }
            Shape::Mirror { .. } => panic!(),
        }
        Shape::Mirror {
            id: ShapeId::next(),
            mirror_of: *mirror_of,
            scale_count: scale_count,
            scale_names: scale_names,
        }
    }

    pub fn with_mirror(self) -> [Shape; 2] {
        let left = self.clone().format_names_as_left();
        let right = self.mirrored();

        [left, right]
    }

    fn format_names_as_left(self) -> Shape {
        let mut left = self;
        match left {
            Shape::Standard { ref mut scales, .. } => {
                scales.iter_mut().for_each(|scale| {
                    scale.name = format!("{}L", scale.name);
                });
            }
            Shape::Mirror { .. } => panic!(),
        };
        left
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

    pub fn get_scale_name(&self, scale_index: usize) -> String {
        match self {
            Shape::Standard { scales, .. } => scales.get(scale_index).unwrap().name.clone(),
            Shape::Mirror { scale_names, .. } => scale_names.get(scale_index).unwrap().clone(),
        }
    }

    pub fn get_scales(&self, range: std::ops::Range<usize>) -> Shape {
        match self {
            Shape::Standard { id, scales } => Shape::Standard {
                id: *id,
                scales: scales[range].to_vec(),
            },
            Shape::Mirror {
                id,
                mirror_of,
                scale_names,
                ..
            } => Shape::Mirror {
                id: *id,
                mirror_of: *mirror_of,
                scale_count: range.end + 1,
                scale_names: scale_names[range].to_vec(),
            },
        }
    }
}

#[derive(Clone)]
pub struct Scale {
    verts: Vertices,
    ports: Ports,
    name: String,
}

impl Display for Scale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{}{}}}", self.verts, self.ports)
    }
}

#[derive(Clone)]
pub struct Vertices(pub Vec<Vertex>);

impl Vertices {
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
                            side_index: side_index,
                            vertex_1: vert_1,
                            vertex_2: vert_2,
                        }
                        .to_ports_of_distribution(Some(
                            &PortDistribution::Center {
                                courtesy_port_distribution: None,
                            },
                        ))
                    })
                    .collect(),
            ),
            name: name,
        }
    }

    pub fn to_hull_scale_with_distributions(
        self,
        port_distributions: Vec<Option<PortDistribution>>,
        name: String,
    ) -> Scale {
        Scale {
            verts: self.clone(),
            ports: Ports({
                let mut ports = Vec::new();
                let mut join_with_next_vertices = Vec::new();
                for ((side_index, vertex_1), vertex_2) in
                    self.0.iter().enumerate().zip(self.0.iter().cycle().skip(1))
                {
                    if let Some(port_distribution) = &port_distributions[side_index] {
                        if join_with_next_vertices.is_empty()
                            && !matches!(port_distribution, PortDistribution::JoinWithNext)
                        {
                            ports.extend(
                                Side {
                                    side_index: side_index,
                                    vertex_1: vertex_1,
                                    vertex_2: vertex_2,
                                }
                                .to_ports_of_distribution(Some(&port_distribution)),
                            );
                        } else if matches!(port_distribution, PortDistribution::JoinWithNext) {
                            join_with_next_vertices.push(vertex_1);
                        } else if !join_with_next_vertices.is_empty()
                            && !matches!(port_distribution, PortDistribution::JoinWithNext)
                        {
                            join_with_next_vertices.push(vertex_1);
                            join_with_next_vertices.push(vertex_2);
                            let entire_side = Side {
                                side_index: 0, // Null value hehe
                                vertex_1: join_with_next_vertices.first().unwrap(),
                                vertex_2: vertex_2,
                            };
                            let sub_sides: Vec<_> = join_with_next_vertices
                                .iter()
                                .enumerate()
                                .zip(join_with_next_vertices.iter().skip(1))
                                .map(|((sub_side_index, sub_vertex_1), sub_vertex_2)| Side {
                                    side_index: 2 + side_index - join_with_next_vertices.len()
                                        + sub_side_index,
                                    vertex_1: sub_vertex_1,
                                    vertex_2: sub_vertex_2,
                                })
                                .collect();
                            let new_undistributed_ports =
                                entire_side.to_ports_of_distribution(Some(&port_distribution));
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
                                    side_index: sub_side_to_distribute_into.unwrap().side_index,
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
                // self.0
                //     .iter()
                //     .enumerate()
                //     .zip(self.0.iter().cycle().skip(1))
                //     .flat_map(|((side_index, vert_1), vert_2)| {
                //         Side {
                //             side_index: side_index,
                //             vert_1: vert_1,
                //             vert_2: vert_2,
                //         }
                //         .to_ports_of_distribution(&port_distributions[side_index])
                //     })
                //     .collect(),
            }),
            name: name,
        }
    }
    // pub fn to_hull_scale_with_distributions(
    //     self,
    //     port_distributions: Vec<PortDistribution>,
    //     name: String,
    // ) -> Scale {
    //     Scale {
    //         verts: self.clone(),
    //         ports: Ports(
    //             self.0
    //                 .iter()
    //                 .enumerate()
    //                 .zip(self.0.iter().cycle().skip(1))
    //                 .flat_map(|((side_index, vert_1), vert_2)| {
    //                     Side {
    //                         side_index: side_index,
    //                         vert_1: vert_1,
    //                         vert_2: vert_2,
    //                     }
    //                     .to_ports_of_distribution(&port_distributions[side_index])
    //                 })
    //                 .collect(),
    //         ),
    //         name: name,
    //     }
    // }
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

impl Port {
    fn has_valid_position(&self) -> bool {
        0.0 <= self.position.to_f32() && self.position.to_f32() <= 0.0
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
