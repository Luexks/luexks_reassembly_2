use crate::{
    blocks::shroud_layer::ShroudLayer, utility::component_formatting::format_bracket_layer_multiline,
};
use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct Shroud(pub Vec<ShroudLayer>);

impl Display for Shroud {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format_bracket_layer_multiline(
                self.0
                    .iter()
                    .map(|shroud_layer| shroud_layer.to_string())
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        )
    }
}

impl Default for Shroud {
    fn default() -> Self {
        Shroud(Vec::new())
    }
}
