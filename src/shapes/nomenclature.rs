use crate::utility::angle::Angle;

fn format_scale(scale_index_option: Option<usize>) -> String {
    scale_index_option
        .map(|scale_index| format!("S{}", scale_index))
        .unwrap_or_default()
}

pub fn shape_name(shape_name: &str, scale_index_option: Option<usize>) -> String {
    format!("{}{}", shape_name, format_scale(scale_index_option),)
}

pub fn shape_name_with_dimensions(
    shape_name: &str,
    width: f32,
    height: f32,
    scale_index_option: Option<usize>,
) -> String {
    let (width, height) = if width > height {
        (height, width)
    } else {
        (width, height)
    };
    format!(
        "{}x{}{}{}",
        width,
        height,
        shape_name,
        format_scale(scale_index_option),
    )
}

pub fn shape_name_with_ratio(
    shape_name: &str,
    antecedent: f32,
    consequent: f32,
    scale_index_option: Option<usize>,
) -> String {
    let (antecedent, consequent) = if antecedent > consequent {
        (consequent, antecedent)
    } else {
        (antecedent, consequent)
    };
    format!(
        "{};{}{}{}",
        antecedent,
        consequent,
        shape_name,
        format_scale(scale_index_option),
    )
    // if antecedent > consequent {
    //     let (antecedent, consequent) = (consequent, antecedent);
    // }
    // format!(
    //     "{};{}{}{}",
    //     antecedent,
    //     consequent,
    //     shape_name,
    //     format_scale(scale_index_option),
    // )
}

pub fn shape_name_with_angle(
    shape_name: &str,
    angle: Angle,
    scale_index_option: Option<usize>,
) -> String {
    format!(
        "{}°{}{}",
        angle.as_degrees().get_value(),
        shape_name,
        format_scale(scale_index_option),
    )
}
