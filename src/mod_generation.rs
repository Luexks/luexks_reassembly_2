use crate::block_types::*;
use crate::configs::*;
use crate::shape_configs::*;
use crate::shape_types::*;
use crate::utils::*;

pub fn create_mod_specifics(blocks: &mut Blocks, shapes: &mut Shapes) {
    add_squares_to_the(shapes);
    add_right_triangles_to_the(shapes);
    add_rectangles_to_the(shapes);
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
            &shapes.0,
            block_variants!(
                (
                    color_1: Color::new_rrggbb("55aa55"),
                    color_2: Color::new_rrggbb("33aa33"),
                    line_color: Color::new_rrggbb("ffffff"),
                ),
                (
                    name: funky_string!("Armor"),
                    features: explicit_features!(
                        Palette,
                        IntLines,
                    ),
                    color_1: Color::new_rrggbb("555555"),
                    color_2: Color::new_rrggbb("333333"),
                    line_color: Color::new_rrggbb("ffffff"),
                ),
            ),
        ),
    );
    add_octagons_to_the(shapes);
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
                &shapes.0.last().unwrap().clone().get_scales(0..3),
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
