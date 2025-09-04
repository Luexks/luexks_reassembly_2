use crate::shapes::scale::Scale;
use crate::shapes::shape_id::*;
use crate::shapes::vertices::Vertices;
use std::fmt::Display;

#[derive(Clone, Debug)]
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
            mirror_of: mirror_of.clone(),
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
            Shape::Standard { id, .. } => id.clone(),
            Shape::Mirror { id, .. } => id.clone(),
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
                id: id.clone(),
                scales: scales[range].to_vec(),
            },
            Shape::Mirror {
                id,
                mirror_of,
                scale_names,
                ..
            } => Shape::Mirror {
                id: id.clone(),
                mirror_of: mirror_of.clone(),
                scale_count: range.end + 1,
                scale_names: scale_names[range].to_vec(),
            },
        }
    }

    pub fn get_first_scale_vertices(&self) -> Vertices {
        match self {
            Shape::Standard { id: _, scales } => {
                scales.first().unwrap().verts.clone()
            }
            _ => panic!()
        }
    }

    pub fn get_nth_scale_vertices(&self, n: usize) -> Vertices {
        match self {
            Shape::Standard { id: _, scales } => {
                scales.get(n).unwrap().verts.clone()
            }
            _ => panic!()
        }
    }
}
