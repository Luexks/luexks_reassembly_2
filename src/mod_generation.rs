use crate::block_types::*;
use crate::configs::*;
use crate::display_oriented_number::*;
use crate::shape_types::*;
use crate::utils::*;

pub fn create_mod_specifics(blocks: &mut Blocks, shapes: &mut Shapes) {
    create_squares(blocks, shapes);
}

fn create_squares(blocks: &mut Blocks, shapes: &mut Shapes) {
    shapes.0.push(Shape {
        id: ShapeId::next(),
        scales: (0..SQUARE_SCALE_COUNT)
            .map(|square_index| {
                Verts(
                    (0..4)
                        .map(|vert_index| {
                            Vert(
                                DisplayOriented2D {
                                    x: DisplayOrientedNumber::Float(
                                        TOTAL_SCALE * 0.5 * (1.0 + square_index as f32),
                                    ),
                                    y: DisplayOrientedNumber::Float(
                                        TOTAL_SCALE * 0.5 * (1.0 + square_index as f32),
                                    ),
                                }
                                .orient_by_index(vert_index),
                            )
                        })
                        .collect(),
                )
                .to_hull_scale()
            })
            .collect(),
    });
    blocks.0.extend(
        new_block!(
            name: funky_string!("Hull"),
            group: GROUP,
            shape: shapes.0.last().unwrap().id,
            features: features!(
                Palette
            ),
            color_1: Color::new_aarrggbb("ee555555"),
            color_2: Color::new_aarrggbb("ffaaaaaa"),
            line_color: Color::new_aarrggbb("ffaaaaaa")
        )
        .get_scales(SQUARE_SCALE_COUNT)
    );
    // (1..=3)
    //     .map(|_| blocks.0.push(blocks.0.last().unwrap().get_next_scale()))
    //     .collect()
}
