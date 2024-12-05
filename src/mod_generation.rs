use crate::configs::*;
use crate::display_oriented_number::*;
use crate::shape_types::*;
use crate::utils::*;

pub fn create_mod_specifics(blocks: &mut String, shapes: &mut Shapes) {
    create_squares(blocks, shapes);
}

fn create_squares(blocks: &mut String, shapes: &mut Shapes) {
    shapes.0.push(Shape {
        id: 0,
        scales: (0..SQUARE_SCALE_COUNT)
            .map(|square_index| {
                Verts(
                    (0..4)
                        .map(|vert_index| {
                            Vert {
                                x: DisplayOrientedNumber::Float(
                                    TOTAL_SCALE * 0.5 * (1.0 + square_index as f32),
                                ),
                                y: DisplayOrientedNumber::Float(
                                    TOTAL_SCALE * 0.5 * (1.0 + square_index as f32),
                                ),
                            }
                            .orient_by_index(vert_index)
                        })
                        .collect(),
                )
                .to_hull_scale()
            })
            .collect(),
    });
}
