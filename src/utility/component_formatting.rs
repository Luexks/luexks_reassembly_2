use std::fmt::Display;
// pub fn format_component_options<T: Display>(named_component_options: &[(Option<T>, &str)]) -> String {
//     named_component_options
//         .iter()
//         .map(|named_component_option| format_component_option(named_component_option))
//         .collect::<Vec<_>>()
//         .join("")
// }

// pub fn format_component_option<T: Display>(named_component_option: &(Option<T>, &str)) -> String {
//     match named_component_option {
//         (Some(component), component_name) => format_component(component, component_name),
//         (None, _) => String::new(),
//     }
// }

macro_rules! format_component_options {
    ($($component:expr => $component_name:expr),* $(,)?) => {
        format!(
            " {} ",
            vec![$(crate::utility::component_formatting::format_component_option!($component => $component_name)),*]
                .into_iter()
                .filter(|s| *s != "".to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
    };
}
pub(crate) use format_component_options;

macro_rules! format_component_option {
    ($component:expr => $component_name:expr $(,)?) => {
        match $component {
            Some(value) => format!("{}={}", $component_name, value),
            None => "".to_string(),
        }
    };
}
pub(crate) use format_component_option;

pub fn format_component<T: Display>(component: T, component_name: &str) -> String {
    format!("{}={}", component_name, component)
}

pub fn format_bracket_layer<T: Display>(component: T) -> String {
    format!("{{{}}}", component)
}

pub fn format_bracket_layer_multiline<T: Display>(component: T) -> String {
    format!("{{\n{}\n}}", component)
}
