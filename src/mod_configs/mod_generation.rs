use crate::blocks::block::*;
use crate::blocks::blocks::*;
use crate::blocks::feature::*;
use crate::mod_configs::mod_metadata::*;
use crate::mod_configs::shape_configs::*;
use crate::shapes::shapes::Shapes;
use crate::utility::color::Color;
use crate::utility::flags::*;
use crate::utility::funky_string::*;

pub fn create_mod_specifics(blocks: &mut Blocks, shapes: &mut Shapes) {
    let _square = add_squares_to_the(shapes);
    let rect_long = add_rect_longs_to_the(shapes);
    let right_triangle = add_right_triangles_to_the(shapes);
    let rectangle = add_rectangles_to_the(shapes);
    let adapter = add_adapters_to_the(shapes);
    let isotri = add_isotris_to_the(shapes);
    let octagon = add_octagons_to_the(shapes);
    let command = add_commands_to_the(shapes);
    blocks.add_blocks(
        block!(
            name: funky_string!("Exo Prime"),
            blurb: funky_string!("Chitin-clad alloys."),
            features: explicit_features!(
                Palette,
            ),
            group: GROUP,
            color_1: Color::new_rrggbb("fc4807"),
            color_2: Color::new_rrggbb("301400"),
            line_color: Color::new_rrggbb("FFFFFF"),
            durability: 4.0,
            density: 0.2,
        )
        .to_extended_blocks_from_plural_shapes_and_plural_variants(
            &[
                rect_long,
                right_triangle.0,
                right_triangle.1,
                rectangle,
                adapter,
                isotri,
            ]
            .iter()
            .map(|&shape_index| shapes.0[shape_index].clone())
            .collect(),
            block_variants!(
                (
                    name: funky_string!("Exo Titus"),
                    color_1: Color::new_rrggbb("55aa55"),
                    color_2: Color::new_rrggbb("33aa33"),
                    line_color: Color::new_rrggbb("ffffff"),
                ),
                (
                    name: funky_string!("Gamma Compounds"),
                    blurb: funky_string!("External spines linking the structure with life."),
                    features: explicit_features!(
                        Palette,
                        IntLines,
                    ),
                    durability: 2.0,
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
                name: funky_string!(""),
                blurb: funky_string!("A heart, of a sort"),
                features: explicit_features!(
                    Command,
                    Generator { capacity: 50.0, capacity_per_sec: 50.0 },
                ),
                capacity: 50.0,
                color_1: Color::new_rrggbb("5555aa"),
                color_2: Color::new_rrggbb("3333aa"),
                line_color: Color::new_rrggbb("ffffff"),
            ))
            .to_extended_blocks_from_singular_shape_and_plural_variants(
                &shapes.0.get(command).unwrap(),
                block_variants!(
                    (
                        capacity: 200.0,
                        features: implicit_features!(
                            Generator { capacity: 200.0, capacity_per_sec: 200.0 },
                        ),
                    ),
                    (
                        capacity: 450.0,
                        features: implicit_features!(
                            Generator { capacity: 450.0, capacity_per_sec: 450.0 },
                        ),
                    ),
                    (
                        capacity: 800.0,
                        features: implicit_features!(
                            Generator { capacity: 800.0, capacity_per_sec: 800.0 },
                        ),
                    ),
                ),
            ),
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
            .to_extended_blocks_from_singular_shape_and_plural_variants(
                &shapes.get(octagon).get_scales(0..3),
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

pub fn _old_create_mod_specifics(blocks: &mut Blocks, shapes: &mut Shapes) {
    let _square = add_squares_to_the(shapes);
    let rect_long = add_rect_longs_to_the(shapes);
    let right_triangle = add_right_triangles_to_the(shapes);
    let rectangle = add_rectangles_to_the(shapes);
    let adapter = add_adapters_to_the(shapes);
    let isotri = add_isotris_to_the(shapes);
    let octagon = add_octagons_to_the(shapes);
    let command = add_commands_to_the(shapes);
    let hardpoint = add_hardpoints_to_the_shapes(shapes);
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
        .to_extended_blocks_from_plural_shapes_and_plural_variants(
            &[
                rect_long,
                right_triangle.0,
                right_triangle.1,
                rectangle,
                adapter,
                isotri,
                hardpoint,
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
                    Generator { capacity: 50.0, capacity_per_sec: 50.0 },
                ),
                capacity: 50.0,
                color_1: Color::new_rrggbb("5555aa"),
                color_2: Color::new_rrggbb("3333aa"),
                line_color: Color::new_rrggbb("ffffff"),
            ))
            .to_extended_blocks_from_singular_shape_and_plural_variants(
                &shapes.0.get(command).unwrap(),
                block_variants!(
                    (
                        capacity: 200.0,
                        features: implicit_features!(
                            Generator { capacity: 200.0, capacity_per_sec: 200.0 },
                        ),
                    ),
                    (
                        capacity: 450.0,
                        features: implicit_features!(
                            Generator { capacity: 450.0, capacity_per_sec: 450.0 },
                        ),
                    ),
                    (
                        capacity: 800.0,
                        features: implicit_features!(
                            Generator { capacity: 800.0, capacity_per_sec: 800.0 },
                        ),
                    ),
                ),
            ),
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
            .to_extended_blocks_from_singular_shape_and_plural_variants(
                &shapes.get(octagon).get_scales(0..3),
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
