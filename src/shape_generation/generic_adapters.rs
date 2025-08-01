use crate::shapes::{nomenclature::shape_name, scale::Scale, shapes::Shapes, vertices::Vertices};

pub fn add_unmirrored_generic_adapter_to_the(
    shapes: &mut Shapes,
    parent_shape_scales: Vec<Scale>,
    first_vert_index: usize,
    name: String,
) -> usize {
    shapes.add_unmirrored_shape_from_scales(get_generic_adapter_scales(
        parent_shape_scales,
        first_vert_index,
        name,
    ));
    shapes.0.len() - 1
}

pub fn add_mirrored_generic_adapter_to_the(
    shapes: &mut Shapes,
    parent_scales: Vec<Scale>,
    first_vert_index: usize,
    name: String,
) -> (usize, usize) {
    shapes.add_mirrored_shape_from_scales(get_generic_adapter_scales(
        parent_scales,
        first_vert_index,
        name,
    ));
    (shapes.0.len() - 2, shapes.0.len() - 1)
}

pub fn get_generic_adapter_scales(
    parent_shape_scales: Vec<Scale>,
    first_vert_index: usize,
    name: String,
) -> Vec<Scale> {
    let vert_count = parent_shape_scales[0].verts.0.len();
    assert!(first_vert_index < vert_count);
    for scale in &parent_shape_scales {
        assert_eq!(vert_count, scale.verts.0.len())
    }

    parent_shape_scales
        .iter()
        .enumerate()
        .zip(parent_shape_scales.iter().skip(1))
        .map(|((scale_index, smaller_scale), larger_scale)| {
            scale_from(
                smaller_scale,
                larger_scale,
                first_vert_index,
                scale_index + 1,
                name.clone(),
            )
        })
        .collect()
}

#[rustfmt::skip]
fn scale_from(
    smaller_scale: &Scale,
    larger_scale: &Scale,
    first_vert_index: usize,
    scale_index: usize,
    name: String,
) -> Scale {
    Vertices(vec![
        larger_scale.verts.0.get(first_vert_index).unwrap().clone(),
        larger_scale.verts.0.get(first_vert_index + 1).unwrap().clone(),
        smaller_scale.verts.0.get(first_vert_index + 1).unwrap().clone(),
        smaller_scale.verts.0.get(first_vert_index).unwrap().clone(),
    ])
    .to_hull_scale(shape_name(&format!("{}Adapter", name), Some(scale_index)))
}
