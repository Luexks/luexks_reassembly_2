use std::fmt::{self, Display};

#[derive(Clone, Debug)]
pub struct Flags<T: Display>(pub Vec<T>);

impl<T: Display> Default for Flags<T> {
    fn default() -> Self {
        Flags(Vec::new())
    }
}

impl<T: Display> Display for Flags<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|flag| flag.to_string())
                .collect::<Vec<_>>()
                .join("|")
        )
    }
}

macro_rules! flags {
    ($($item:expr),* $(,)?) => {
        Flags(vec![$($item),*])
    };
}
pub(crate) use flags;
