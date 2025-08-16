use std::fmt::{self, Display};

#[macro_export]
macro_rules! funky_string {
    ($value:expr) => {
        FunkyString($value.to_string())
    };
}
pub(crate) use funky_string;

#[derive(Clone, Debug)]
pub struct FunkyString(pub String);

impl Display for FunkyString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self.0)
    }
}
