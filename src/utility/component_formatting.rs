macro_rules! format_components {
    ($($component:expr => $component_name:expr),*) => {
        format!(
            "{}",
            vec![$(crate::utility::component_formatting::format_component!($component => $component_name)),*].join("")
        )
    };
}
pub(crate) use format_components;

macro_rules! format_component {
    ($component:expr => $component_name:expr) => {
        match $component {
            Some(value) => format!(",{}={}", $component_name, value),
            None => "".to_string(),
        }
    };
}
pub(crate) use format_component;
