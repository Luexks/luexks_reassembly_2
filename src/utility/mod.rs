pub mod angle;
pub mod color;
pub mod display_oriented_number;
pub mod flags;
pub mod funky_string;
pub mod option_comparison_prioritising_some;
pub mod component_formatting;

// macro_rules! repeat_expression {
//     (0, $ident:ident) => {};
//     (1, $ident:ident) => {
//         $ident
//     };
//     ($count:expr, $ident:ident) => {
//         $ident, repeat_expression!(($count - 1), $ident)
//     };
// }
// pub(crate) use repeat_expression;
