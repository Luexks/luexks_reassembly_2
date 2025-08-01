use crate::{
    shapes::{
        nomenclature::shape_name_with_dimensions, scale::Scale, shapes::Shapes, vertex::Vertex,
        vertices::Vertices,
    },
    utility::display_oriented_math::do2d_float_from,
};

use super::shape_generation::MASTER_SCALE;

pub fn add_generic_rectangles_to_the(
    shapes: &mut Shapes,
    rectangle_ratios_and_name_options: Vec<(f32, f32, Option<String>)>,
) -> usize {
    // let scale_from =
    //     |width_scale_factor: f32, height_scale_factor: f32, name_option: Option<String>| {
    //         let half_rect_long_width = 0.5 * MASTER_SCALE * (width_scale_factor as f32);
    //         let half_rect_long_height = 0.5 * MASTER_SCALE * (height_scale_factor as f32);
    //         let unoriented_do2d = do2d_float_from(half_rect_long_width, half_rect_long_height);
    //         Vertices(
    //             (0..4)
    //                 .map(|vert_index| Vertex(unoriented_do2d.orient_by_vert_index(vert_index)))
    //                 .collect(),
    //         )
    //         .to_hull_scale(name_option.unwrap_or_else(|| {
    //             format!(
    //                 "{}x{}",
    //                 width_scale_factor,
    //                 height_scale_factor,
    //             )
    //         }))
    //     };
    shapes.add_unmirrored_shape_from_scales(get_generic_rectangles(
        rectangle_ratios_and_name_options,
    ));
    shapes.0.len() - 1
}

pub fn get_generic_rectangles(
    rectangle_ratios_and_name_options: Vec<(f32, f32, Option<String>)>,
) -> Vec<Scale> {
    rectangle_ratios_and_name_options
        .iter()
        .map(|rectangle_ratios_and_name_option| {
            scale_from(
                rectangle_ratios_and_name_option.0,
                rectangle_ratios_and_name_option.1,
                rectangle_ratios_and_name_option.2.clone(),
            )
        })
        .collect::<Vec<_>>()
}

fn scale_from(
    width_scale_factor: f32,
    height_scale_factor: f32,
    name_option: Option<String>,
) -> Scale {
    let half_width = 0.5 * MASTER_SCALE * (width_scale_factor as f32);
    let half_height = 0.5 * MASTER_SCALE * (height_scale_factor as f32);
    let unoriented_do2d = do2d_float_from(half_width, half_height);
    Vertices(
        (0..4)
            .map(|vert_index| Vertex(unoriented_do2d.orient_by_vert_index(vert_index)))
            .collect(),
    )
    .to_hull_scale(name_option.unwrap_or_else(|| {
        shape_name_with_dimensions("", width_scale_factor, height_scale_factor, None)
    }))
}
