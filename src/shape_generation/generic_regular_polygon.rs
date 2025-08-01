use std::f32::consts::{PI, TAU};

use crate::{
    shapes::{
        nomenclature::shape_name, scale::Scale, shapes::Shapes, vertex::Vertex, vertices::Vertices,
    },
    utility::display_oriented_math::do2d_float_from,
};

use super::shape_generation::MASTER_SCALE;

const POLYGON_NAMES: [&str; 33] = [
    "",
    "Point",
    "Line",
    "Triangle",
    "Square",
    "Pentagon",
    "Hexagon",
    "Heptagon",
    "Octagon",
    "Nonagon",
    "Decagon",
    "Hendecagon",
    "Dodecagon",
    "Triskaidecagon",
    "Tetradecagon",
    "Pentadecagon",
    "Hexadegagon",
    "Heptadecagon",
    "Octadecagon",
    "Enneadecagon",
    "Icosagon",
    "Icosihenagon",
    "Icosidigon",
    "Icostrigon",
    "Icositetragon",
    "Icosipentagon",
    "Icosihexagon",
    "Icosiheptagon",
    "Icosioctagon",
    "Icosienneagon",
    "Triacontagon",
    "Triacontahenagon",
    "Triacontadigon",
];

pub fn add_generic_regular_polygon_to_the(
    shapes: &mut Shapes,
    side_count: usize,
    scale_count: usize,
    point_forwards: bool,
) -> usize {
    shapes.add_unmirrored_shape_from_scales(get_regular_polygon_scales(
        side_count,
        scale_count,
        point_forwards,
    ));
    shapes.0.len() - 1
}

pub fn get_regular_polygon_scales(
    side_count: usize,
    scale_count: usize,
    point_forwards: bool,
) -> Vec<Scale> {
    (1..=scale_count)
        .map(|scale_index| scale_from(side_count, scale_index, point_forwards))
        .collect()
}

fn scale_from(side_count: usize, scale_index: usize, point_forwards: bool) -> Scale {
    assert!(
        side_count >= 3,
        "Regular polygon must have at least 3 sides."
    );
    let circumradius = (MASTER_SCALE * scale_index as f32) / (2.0 * (PI / side_count as f32).sin());
    let angle_step = TAU / side_count as f32;
    let angle_offset = if point_forwards {
        0.0
    } else {
        angle_step / 2.0
    };
    // let name = POLYGON_NAMES.get(side_count).clone().unwrap_or_else(|| &&format!("{}-gon", side_count));
    let name = match POLYGON_NAMES.get(side_count) {
        Some(name) => *name,
        None => &format!("{}-gon", side_count),
    };
    Vertices(
        (0..side_count)
            .map(|vert_index| {
                Vertex(do2d_float_from(
                    circumradius * (vert_index as f32 * -angle_step + angle_offset).cos(),
                    circumradius * (vert_index as f32 * -angle_step + angle_offset).sin(),
                ))
            })
            .collect(),
    )
    .to_hull_scale(shape_name(&name, Some(scale_index)))
}
