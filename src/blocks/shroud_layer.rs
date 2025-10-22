use crate::{
    mod_configs::shape_configs::SHAPE_ID_BASE,
    shapes::shape_id::ShapeId,
    utility::{
        angle::Angle,
        component_formatting::{format_bracket_layer, format_component_options},
        display_oriented_math::{
            do2d_float_from, do3d_float_from, DisplayOriented2D, DisplayOriented3D,
        },
    },
};
use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct ShroudLayer {
    pub shape: Option<ShapeId>,
    pub size: Option<DisplayOriented2D>,
    pub offset: Option<DisplayOriented3D>,
    pub color_1: Option<ShroudLayerColor>,
    pub color_2: Option<ShroudLayerColor>,
    pub line_color: Option<ShroudLayerColor>,
    pub angle: Option<Angle>,
    pub taper: Option<f32>,
}

impl Default for ShroudLayer {
    fn default() -> Self {
        ShroudLayer {
            shape: Some(ShapeId::Number(SHAPE_ID_BASE)),
            size: Some(do2d_float_from(0.0, 0.0)),
            offset: Some(do3d_float_from(0.0, 0.0, 0.01)),
            color_1: Some(ShroudLayerColor::Color1),
            color_2: Some(ShroudLayerColor::Color2),
            line_color: Some(ShroudLayerColor::LineColor),
            angle: Some(Angle::Degree(0.0)),
            taper: Some(1.0),
        }
    }
}

impl Display for ShroudLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\t{}",
            format_bracket_layer(format_component_options!(
                &self.color_1 => "tri_color_id",
                &self.color_2 => "tri_color1_id",
                &self.line_color => "line_color_id",
                &self.shape => "shape",
                &self.size => "size",
                &self.offset => "offset",
                &self.angle => "angle",
                self.taper => "taper",
            ))
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ShroudLayerColor {
    Color1,
    Color2,
    LineColor,
}

impl Display for ShroudLayerColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ShroudLayerColor::Color1 => "0",
                ShroudLayerColor::Color2 => "1",
                ShroudLayerColor::LineColor => "2",
            }
        )
    }
}

// #[macro_export]
// macro_rules! shroud_layer {
//     () => {
//         ShroudLayer::default()
//     };
//     {$($component_name:ident: $component_value:expr),* $(,)?} => {
//         {
//             let mut shroud_layer = ShroudLayer::default();
//             $(
//                 shroud_layer.$component_name = Some($component_value);
//             )*
//             shroud_layer
//         }
//     };
// }

#[macro_export] macro_rules! shroud_layer {
    ($($component_name:ident: $component_value:expr),* $(,)?) => {
        ShroudLayer {
            $($component_name: Some($component_value),)*
            ..ShroudLayer::default()
        }
    };
}
