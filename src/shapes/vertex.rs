use crate::utility::display_oriented_math::DisplayOriented2D;

#[derive(Clone, Debug)]
pub struct Vertex(pub DisplayOriented2D);

macro_rules! vert {
    ($x:expr, $y:expr) => {
        Vertex(crate::utility::display_oriented_math::DisplayOriented2D::simple($x, $y))
    };
}
pub(crate) use vert;
