use crate::block_types::*;
use crate::configs::*;
use crate::display_oriented_number::*;
use crate::shape_types::*;
use crate::utils::*;

pub fn create_mod_specifics(blocks: &mut Blocks, shapes: &mut Shapes) {
    create_square_shapes(shapes);
    create_right_triangle_shapes(shapes);
    blocks.0.extend(
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
            density: 0.1
        )
        .to_hull_blocks_from_shapes_and_varients(
            shapes,
            vec![
                block_without_id!(
                    color_1: Color::new_rrggbb("55aa55"),
                    color_2: Color::new_rrggbb("33aa33"),
                    line_color: Color::new_rrggbb("ffffff")
                ),
                block_without_id!(
                    name: funky_string!("Armor"),
                    features: implicit_features!(
                        Palette,
                        IntLines,
                    ),
                    color_1: Color::new_rrggbb("555555"),
                    color_2: Color::new_rrggbb("333333"),
                    line_color: Color::new_rrggbb("ffffff")
                ),
            ],
        ),
    );
}

fn create_square_shapes(shapes: &mut Shapes) {
    let scale_from = |scale_index: usize| {
        let half_square_length = 0.5 * TOTAL_SCALE * (1.0 + scale_index as f32);
        let unoriented_do2d = do2d_float_from(half_square_length, half_square_length);
        Vertices(
            (0..4)
                .map(|vert_index| Vertex(unoriented_do2d.orient_by_index(vert_index)))
                .collect(),
        )
        .to_hull_scale()
    };
    shapes.add_unmirrored_shape_from_scales(
        (0..SQUARE_SCALE_COUNT)
            .map(|scale_index| scale_from(scale_index))
            .collect(),
    );
}

// fn create_square_shapes(shapes: &mut Shapes) {
//     shapes.add_unmirrored_shape_from_scales(
//         (0..SQUARE_SCALE_COUNT)
//             .map(|scale_index| {
//                 let half_square_length = 0.5 * TOTAL_SCALE * (1.0 + scale_index as f32);
//                 let unoriented_do2d = do2d_float_from(half_square_length, half_square_length);
//                 Vertices(
//                     (0..4)
//                         .map(|vert_index| Vertex(unoriented_do2d.orient_by_index(vert_index)))
//                         .collect(),
//                 )
//                 .to_hull_scale()
//             })
//             .collect(),
//     );
// }

fn create_right_triangle_shapes(shapes: &mut Shapes) {
    let scale_from = |width_scale_factor: f32, height_scale_factor: f32| {
        scale_from_alternating_vertices_and_port_distributions!(
            vert!(0.0, 0.0),
            TowardsFromCurrentVert,
            vert!(0.0, TOTAL_SCALE * height_scale_factor),
            Center,
            vert!(TOTAL_SCALE * width_scale_factor * height_scale_factor, 0.0),
            BackwardsFromNextVert
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

// #[rustfmt::skip]
// fn create_right_triangle_shapes(shapes: &mut Shapes) {
//     shapes.add_mirrored_shape_from_scales(
//         RIGHT_TRIANGLE_HEIGHT_SCALE_FACTORS
//             .iter()
//             .flat_map(|height_scale_factor| {
//                 RIGHT_TRIANGLE_WIDTH_SCALE_FACTORS
//                     .iter()
//                     .map(move |width_scale_factor| {
//                         scale_from_alternating_vertices_and_port_distributions!(
//                             vert!(0.0, 0.0),
//                             TowardsFromCurrentVert { distance_from_current_vert: don_float_from(PORT_SPACING_FROM_VERT) },
//                             vert!(0.0, TOTAL_SCALE * height_scale_factor),
//                             Center,
//                             vert!(TOTAL_SCALE * width_scale_factor * height_scale_factor, 0.0),
//                             BackwardsFromNextVert { distance_from_next_vert: don_float_from(PORT_SPACING_FROM_VERT) }
//                         )
//                     })
//             })
//             .collect::<Vec<_>>(),
//     );
// }

// fn create_square_shapes(shapes: &mut Shapes) {
//     shapes.add_unmirrored_shape_from_scales(
//         (0..SQUARE_SCALE_COUNT)
//             .map(|square_index| {
//                 Vertices(
//                     (0..4)
//                         .map(|vert_index| {
//                             Vertex(
//                                 DisplayOriented2D {
//                                     x: DisplayOrientedNumber::Float(
//                                         TOTAL_SCALE * 0.5 * (1.0 + square_index as f32),
//                                     ),
//                                     y: DisplayOrientedNumber::Float(
//                                         TOTAL_SCALE * 0.5 * (1.0 + square_index as f32),
//                                     ),
//                                 }
//                                 .orient_by_index(vert_index),
//                             )
//                         })
//                         .collect(),
//                 )
//                 .to_hull_scale()
//             })
//             .collect(),
//     );
// }

// fn create_right_triangle_shapes(shapes: &mut Shapes) {
//     shapes.add_mirrored_shape_from_scales(
//         RIGHT_TRIANGLE_HEIGHT_SCALE_FACTORS
//             .iter()
//             .map(|right_triangle_height_scale_factor| {
//                 RIGHT_TRIANGLE_WIDTH_SCALE_FACTORS
//                     .iter()
//                     .map(|right_triangle_width_scale_factor| {
//                         Vertices(vec![
//                             vert!(0.0, 0.0),
//                             vert!(0.0, TOTAL_SCALE * right_triangle_height_scale_factor),
//                             vert!(TOTAL_SCALE * right_triangle_width_scale_factor * right_triangle_height_scale_factor, 0.0),
//                         ])
//                         .to_hull_scale()
//                     })
//                     .collect::<Vec<_>>()
//             })
//             .flatten()
//             .collect::<Vec<_>>(),
//     );
// }
