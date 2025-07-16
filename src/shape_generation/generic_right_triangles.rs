use crate::shapes::{
    courtesy_port_distribution::CourtesyPortDistribution,
    nomenclature::shape_name_with_dimensions,
    port_distribution::scale_from_alternating_vertices_and_port_distributions,
    scale::Scale,
    shapes::Shapes,
    vertex::{vert, Vertex},
    vertices::Vertices,
};

use super::shape_generation::MASTER_SCALE;

pub const RIGHT_TRIANGLE_WIDTH_SCALE_FACTORS: [f32; 8] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
pub const RIGHT_TRIANGLE_HEIGHT_SCALE_FACTORS: [f32; 3] = [0.5, 1.0, 2.0];

pub fn add_generic_right_triangles_to_the(
    shapes: &mut Shapes,
    right_triangle_ratios_and_name_options: Vec<(f32, f32, Option<String>)>,
) -> (usize, usize) {
    shapes.add_mirrored_shape_from_scales(
        right_triangle_ratios_and_name_options
            .iter()
            .map(|right_triangle_ratios_and_name_option| {
                scale_from(
                    right_triangle_ratios_and_name_option.0,
                    right_triangle_ratios_and_name_option.1,
                    right_triangle_ratios_and_name_option.2.clone(),
                )
            })
            .collect(),
    );
    (shapes.0.len() - 2, shapes.0.len() - 1)
}

fn scale_from(
    width_scale_factor: f32,
    height_scale_factor: f32,
    name_option: Option<String>,
) -> Scale {
    scale_from_alternating_vertices_and_port_distributions!(
        name: name_option.unwrap_or_else(|| shape_name_with_dimensions("rightTri", width_scale_factor, height_scale_factor, None)),
        vert!(0.0, 0.0),
        TowardsFromCurrentVert: CourtesyPortDistribution::HalfwayToEnd,
        vert!(0.0, MASTER_SCALE * height_scale_factor),
        Center: None,
        vert!(MASTER_SCALE * width_scale_factor, 0.0),
        BackwardsFromNextVert: CourtesyPortDistribution::HalfwayToEnd,
    )
}
