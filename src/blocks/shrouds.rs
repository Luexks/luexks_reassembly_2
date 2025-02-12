use crate::blocks::shroud::Shroud;
use std::fmt::Display;

#[derive(Clone)]
pub struct Shrouds(Vec<Shroud>);

impl Display for Shrouds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{\n{}\n}}",
            self.0
                .iter()
                .map(|shroud| format!("{}", shroud))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}
