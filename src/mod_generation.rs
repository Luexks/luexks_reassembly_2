use std::f32::consts::SQRT_2;

use crate::block_types::*;
use crate::configs::*;
use crate::display_oriented_number::*;
use crate::shape_types::*;
use crate::utils::*;

pub fn create_mod_specifics(blocks: &mut Blocks, shapes: &mut Shapes) {
    add_squares_to_the(shapes);
    add_right_triangles_to_the(shapes);
    add_rectangles_to_the(shapes);
    add_octagons_to_the(shapes);
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
            durability: 1.0,
            density: 0.1,
        )
        .to_hull_blocks_from_shapes_and_variants(
            shapes,
            vec![
                block_variant!(
                    color_1: Color::new_rrggbb("55aa55"),
                    color_2: Color::new_rrggbb("33aa33"),
                    line_color: Color::new_rrggbb("ffffff"),
                ),
                block_variant!(
                    name: funky_string!("Armor"),
                    features: explicit_features!(
                        Palette,
                        IntLines,
                    ),
                    color_1: Color::new_rrggbb("555555"),
                    color_2: Color::new_rrggbb("333333"),
                    line_color: Color::new_rrggbb("ffffff"),
                ),
            ],
        ),
    );
}

fn add_squares_to_the(shapes: &mut Shapes) {
    let scale_from = |scale_index: usize| {
        let half_square_length = 0.5 * MASTER_SCALE * (scale_index as f32);
        let unoriented_do2d = do2d_float_from(half_square_length, half_square_length);
        Vertices(
            (0..4)
                .map(|vert_index| Vertex(unoriented_do2d.orient_by_vert_index(vert_index)))
                .collect(),
        )
        .to_hull_scale(format!("SquareS{}", scale_index))
    };
    shapes.add_unmirrored_shape_from_scales(
        (1..=SQUARE_SCALE_COUNT)
            .map(|scale_index| scale_from(scale_index))
            .collect(),
    );
}

fn add_right_triangles_to_the(shapes: &mut Shapes) {
    let scale_from = |width_scale_factor: f32, height_scale_factor: f32| {
        scale_from_alternating_vertices_and_port_distributions!(
            vert!(0.0, 0.0),
            TowardsFromCurrentVert,
            vert!(0.0, MASTER_SCALE * height_scale_factor),
            Center,
            vert!(MASTER_SCALE * width_scale_factor * height_scale_factor, 0.0),
            BackwardsFromNextVert
            name: format!("{};{}rightTriS{}", 1, width_scale_factor, height_scale_factor)
        )
    };
    shapes.add_mirrored_shape_from_scales(
        RIGHT_TRIANGLE_HEIGHT_SCALE_FACTORS
            .iter()
            .flat_map(|&height_scale_factor| {
                RIGHT_TRIANGLE_WIDTH_SCALE_FACTORS
                    .iter()
                    .map(move |&width_scale_factor| {
                        scale_from(width_scale_factor, height_scale_factor)
                    })
            })
            .collect::<Vec<_>>(),
    );
}

fn add_rectangles_to_the(shapes: &mut Shapes) {
    let scale_from = |scale_factor_float_2d_and_name: &(f32, f32, String)| {
        let unoriented_do2d = do2d_float_from(
            scale_factor_float_2d_and_name.0 * MASTER_SCALE * 0.5,
            scale_factor_float_2d_and_name.1 * MASTER_SCALE * 0.5,
        );
        Vertices(
            (0..4)
                .map(|vert_index| Vertex(unoriented_do2d.orient_by_vert_index(vert_index)))
                .collect(),
        )
        .to_hull_scale(scale_factor_float_2d_and_name.2.clone())
    };
    shapes.add_unmirrored_shape_from_scales(
        RECTANGLE_SCALE_FACTORS_AND_NAMES
            .iter()
            .map(|scale_factor_float_2d| scale_from(scale_factor_float_2d))
            .collect(),
    );
}

#[rustfmt::skip]
fn add_octagons_to_the(shapes: &mut Shapes) {
    let scale_from = |scale_index: usize| {
        let (half_octagon_bounding_box_length, half_octagon_side_length) = if scale_index == 0 {
            (
                0.5 * MASTER_SCALE,
                (-0.5 + 1.0 / SQRT_2) * MASTER_SCALE,
            )
        } else {
            (
                (0.5 + 1.0 / SQRT_2) * MASTER_SCALE * scale_index as f32,
                0.5 * MASTER_SCALE * scale_index as f32,
            )
        };
        Vertices(
            (0..8)
                .map(|vert_index| {
                    Vertex(
                        do2d_float_from(
                            half_octagon_bounding_box_length,
                            half_octagon_side_length * if vert_index % 2 == 0 { -1.0 } else { 1.0 },
                        )
                        .rotate_by_vert_index((vert_index as i32 / 2) as usize),
                    )
                })
                .collect(),
        )
        .to_hull_scale(if scale_index == 0 { "GridOctagon".to_string() } else { format!("OctagonS{}", scale_index ) })
    };
    shapes.add_unmirrored_shape_from_scales(
        (0..=OCTAGON_SCALE_COUNT)
            .map(|scale_index| scale_from(scale_index))
            .collect(),
    );
}
