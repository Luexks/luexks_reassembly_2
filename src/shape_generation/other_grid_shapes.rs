use crate::shape_generation::*;
use crate::shapes::nomenclature::shape_name_with_angle;
use crate::shapes::port::Port;
use crate::shapes::port_distribution::{default_port_distribution_from_variant, PortDistribution};
use crate::shapes::port_flags::no_port_flags;
use crate::shapes::port_module::PortModule;
use crate::shapes::shapes::Shapes;
use crate::utility::display_oriented_math::{do2d_float_from, don_float_from, don_fraction_from};
use crate::utility::flags::Flags;
use crate::{
    shapes::{
        nomenclature::shape_name_with_ratio_and_dimensions,
        scale::Scale,
        vertex::{vert, Vertex},
        vertices::Vertices,
    },
    utility::angle::Angle,
};
use std::f32::consts::SQRT_2;
use std::rc::Rc;

pub fn add_other_grid_shapes_to_the(shapes: &mut Shapes) -> usize {
    shapes.add_unmirrored_shape_from_scales(get_other_grid_shape_scales());
    shapes.0.len() - 1
}

pub fn get_other_grid_shape_scales() -> Vec<Scale> {
    get_isotri_scales(Angle::Degree(90.0), 1, 0.5, Some("".to_string()))
        .into_iter()
        .chain(get_isotri_scales(
            Angle::Degree(90.0),
            1,
            0.5 * SQRT_2,
            Some("".to_string()),
        ))
        .chain(std::iter::once(get_1x1_right_tri_with_2_hypotenuse_ports(
            Angle::Degree(90.0),
            // Angle::Degree(90.0),
            1.0,
            1.0,
            Some("".to_string()),
        )))
        // .chain(get_isotri_scales(
        //     dbg!(Angle::Degree(90.0)),
        //     // Angle::Degree(90.0),
        //     1,
        //     1.0,
        //     Some("".to_string()),
        // ))
        .chain(generic_rectangles::get_generic_rectangles(vec![
            ((SQRT_2 * 0.5, SQRT_2 * 0.5, Some("½√2square".to_string()))),
            ((SQRT_2, SQRT_2, Some("√2square".to_string()))),
        ]))
        .collect()
}

fn get_1x1_right_tri_with_2_hypotenuse_ports(
    angle: Angle,
    scale_index: f32,
    scale_multiplier: f32,
    name_option: Option<String>,
) -> Scale {
    let isotri_width = MASTER_SCALE
        * scale_multiplier
        * (angle.as_radians().get_value() * 0.5).cos()
        * scale_index as f32;
    let half_isotri_height = MASTER_SCALE
        * scale_multiplier
        * (angle.as_radians().get_value() * 0.5).sin()
        * scale_index as f32;
    Vertices(
        (0..=1)
            .map(|vert_index| {
                Vertex(
                    do2d_float_from(isotri_width, half_isotri_height)
                        .orient_by_vert_index(vert_index),
                )
            })
            .chain(std::iter::once(Vertex(do2d_float_from(0.0, 0.0))))
            .collect(),
    )
    .to_hull_scale_with_modules(
        vec![
            Some(PortModule {
                port_flags: Flags::default(),
                port_distribution: PortDistribution::Custom {
                    port_function: Rc::new(|side_index, _side_length| {
                        vec![
                            Port {
                                side_index,
                                position: don_fraction_from(1.0, 4.0),
                                flags: no_port_flags!(),
                            },
                            Port {
                                side_index,
                                position: don_fraction_from(3.0, 4.0),
                                flags: no_port_flags!(),
                            },
                        ]
                    }),
                },
            }),
            Some(PortModule {
                port_flags: Flags::default(),
                port_distribution: default_port_distribution_from_variant!(Center),
            }),
            Some(PortModule {
                port_flags: Flags::default(),
                port_distribution: default_port_distribution_from_variant!(Center),
            }),
        ],
        name_option
            .unwrap_or_else(|| shape_name_with_angle("isotri", angle, Some(scale_index as usize))),
    )
}
