use crate::shapes::scale::Scale;
use crate::shapes::shape::Shape;
use crate::shapes::shape_id::*;
use std::fmt::Display;

#[derive(Clone, Debug)]
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

    pub fn get(&self, shape_idx: usize) -> &Shape {
        &self.0.get(shape_idx).unwrap()
    }

    pub fn get_mirror_of_mirror_idx(&self, mirror_idx: usize) -> Option<&Shape> {
        let mirror_shape = self.0.get(mirror_idx).unwrap();
        match mirror_shape {
            Shape::Standard { .. } => None,
            Shape::Mirror {
                id: _,
                mirror_of,
                scale_count: _,
                scale_names: _,
            } => self
                .0
                .iter()
                .find(|shape| shape.get_id().unwrap() == *mirror_of),
        }
    }
}
