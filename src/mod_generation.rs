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
        id: 0,
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
    let square_base = BlockId::next();
    blocks.0.push(Block {
        id: square_base,
        extends: None,
        group: Some(GROUP),
        sort: Some(BlockSort::next()),
        feautures: Some(Flags(vec![
            Feature::Palette,
            Feature::Thruster {
                force: Some(100.0),
                boost: Some(100.0),
                boost_time: Some(100.0),
                color_1: Some(Color::new_rrggbb("555555")),
                color_2: Some(Color::new_rrggbb("999999")),
            },
        ])),
        capacity: None,
        elasticity: None,
        binding_id: None,
        color_1: None,
        color_2: None,
        line_color: None,
        shape: Some(shapes.0.last().unwrap().id),
        scale: None,
        name: None,
        points: None,
        durability: None,
        armor: None,
        density: None,
        blurb: None,
    });
    let _: Vec<_> = (1..=3)
        .map(|square_index| {
            blocks.0.push(Block {
                id: BlockId::next(),
                extends: Some(square_base),
                group: None,
                sort: None,
                feautures: None,
                capacity: None,
                elasticity: None,
                binding_id: None,
                color_1: None,
                color_2: None,
                line_color: None,
                shape: None,
                scale: Some(square_index),
                name: None,
                points: None,
                durability: None,
                armor: None,
                density: None,
                blurb: None,
            })
        })
        .collect();
}
