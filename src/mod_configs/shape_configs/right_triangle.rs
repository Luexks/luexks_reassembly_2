use crate::shapes::{
    courtesy_port_distribution::CourtesyPortDistribution,
    nomenclature::shape_name_with_dimensions,
    port_distribution::scale_from_alternating_vertices_and_port_distributions,
    scale::Scale,
    shapes::Shapes,
    vertex::{vert, Vertex},
    vertices::Vertices,
};

use super::MASTER_SCALE;

pub const RIGHT_TRIANGLE_WIDTH_SCALE_FACTORS: [f32; 8] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
pub const RIGHT_TRIANGLE_HEIGHT_SCALE_FACTORS: [f32; 3] = [0.5, 1.0, 2.0];

pub fn add_right_triangles_to_the(shapes: &mut Shapes) -> (usize, usize) {
    shapes.add_mirrored_shape_from_scales(
        RIGHT_TRIANGLE_HEIGHT_SCALE_FACTORS
            .iter()
            .flat_map(|height_scale_factor| {
                RIGHT_TRIANGLE_WIDTH_SCALE_FACTORS
                    .iter()
                    .map(|width_scale_factor| scale_from(*width_scale_factor, *height_scale_factor))
            })
            .collect::<Vec<_>>(),
    );
    (shapes.0.len() - 2, shapes.0.len() - 1)
}

fn scale_from(width_scale_factor: f32, height_scale_factor: f32) -> Scale {
    scale_from_alternating_vertices_and_port_distributions!(
        name: shape_name_with_dimensions("rightTri", 1.0, width_scale_factor, Some(height_scale_factor as usize)),
        vert!(0.0, 0.0),
        TowardsFromCurrentVert: CourtesyPortDistribution::HalfwayToEnd,
        vert!(0.0, MASTER_SCALE * height_scale_factor),
        Center: None,
        vert!(MASTER_SCALE * width_scale_factor * height_scale_factor, 0.0),
        BackwardsFromNextVert: CourtesyPortDistribution::HalfwayToEnd,
    )
}
