use crate::{
    shapes::shape_id::ShapeId,
    utility::{
        angle::Angle,
        component_formatting::format_components,
        display_oriented_math::{DisplayOriented2D, DisplayOriented3D},
    },
};
use std::fmt::Display;

#[derive(Clone)]
pub struct Shroud {
    shape: Option<ShapeId>,
    size: Option<DisplayOriented2D>,
    offset: Option<DisplayOriented3D>,
    color_1: Option<ShroudColor>,
    color_2: Option<ShroudColor>,
    line_color: Option<ShroudColor>,
    angle: Option<Angle>,
    taper: Option<f32>,
}

impl Display for Shroud {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{{}}}",
            format_components!(
                self.shape => "shape",
                &self.size => "size",
                &self.offset => "offset",
                &self.color_1 => "tri_color_id",
                &self.color_2 => "tri_color1_id",
                &self.line_color => "line_color_id",
                &self.angle => "angle",
                self.taper => "taper",
            )
        )
    }
}

#[derive(Clone)]
enum ShroudColor {
    Color1,
    Color2,
    LineColor,
}

impl Display for ShroudColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ShroudColor::Color1 => "0",
                ShroudColor::Color2 => "1",
                ShroudColor::LineColor => "2",
            }
        )
    }
}
