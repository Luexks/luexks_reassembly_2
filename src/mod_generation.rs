use crate::block_types::*;
use crate::configs::*;
use crate::shape_configs::*;
use crate::shape_types::*;
use crate::utils::*;

pub fn create_mod_specifics(blocks: &mut Blocks, shapes: &mut Shapes) {
    let square_index = add_squares_to_the(shapes);
    let rect_long_index = add_rect_longs_to_the(shapes);
    let right_triangle_index = add_right_triangles_to_the(shapes);
    let rectangle_index = add_rectangles_to_the(shapes);
    let adapter_index = add_adapters_to_the(shapes);
    let isotri_index = add_isotris_to_the(shapes);
    let octagon_index = add_octagons_to_the(shapes);
    let command_index = add_commands_to_the(shapes);
    blocks.add_blocks(
        block!(
            name: funky_string!("Hull"),
            blurb: funky_string!("Basic structure"),
            features: explicit_features!(
                Palette,
            ),
            group: GROUP,
            color_1: Color::new_rrggbb("5555aa"),
            color_2: Color::new_rrggbb("3333aa"),
            line_color: Color::new_rrggbb("ffffff"),
            durability: 1.001,
            density: 0.1,
        )
        .to_extended_blocks_from_shapes_and_variants(
            &[
                rect_long_index,
                right_triangle_index,
                rectangle_index,
                adapter_index,
                isotri_index,
            ]
            .iter()
            .map(|&shape_index| shapes.0[shape_index].clone())
            .collect(),
            block_variants!(
                (
                    color_1: Color::new_rrggbb("55aa55"),
                    color_2: Color::new_rrggbb("33aa33"),
                    line_color: Color::new_rrggbb("ffffff"),
                ),
                (
                    name: funky_string!("Armor"),
                    blurb: funky_string!("High-mass defense compound"),
                    features: explicit_features!(
                        Palette,
                        IntLines,
                    ),
                    durability: 2.001,
                    density: 0.2,
                    color_1: Color::new_rrggbb("555555"),
                    color_2: Color::new_rrggbb("333333"),
                    line_color: Color::new_rrggbb("ffffff"),
                ),
            ),
        ),
    );
    blocks.add_blocks(
        blocks
            .extend_first_block(block!(
                name: funky_string!("Command"),
                blurb: funky_string!("A heart, of a sort"),
                features: explicit_features!(
                    Command,
                ),
                color_1: Color::new_rrggbb("5555aa"),
                color_2: Color::new_rrggbb("3333aa"),
                line_color: Color::new_rrggbb("ffffff"),
            ))
            .to_extended_blocks_from_shape(&shapes.0.get(command_index).unwrap()),
    );
    blocks.add_blocks(
        blocks
            .extend_first_block(block!(
                name: funky_string!("Container"),
                blurb: funky_string!("Resource storage"),
                capacity: 50.0,
                color_1: Color::new_rrggbb("5555aa"),
                color_2: Color::new_rrggbb("3333aa"),
                line_color: Color::new_rrggbb("ffffff"),
            ))
            .to_extended_blocks_from_one_shape_and_variants(
                &shapes.0.get(octagon_index).unwrap().get_scales(0..3),
                block_variants!(
                    (
                        capacity: 290.0,
                    ),
                    (
                        capacity: 1160.0,
                    ),
                ),
            ),
    );
}
