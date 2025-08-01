use crate::mod_configs::mod_metadata::FUNKY_PORT_FORMATING;
use crate::shapes::port::*;
use std::fmt::{self, Display};

#[derive(Clone, Debug)]
pub struct Ports(pub Vec<Port>);

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
