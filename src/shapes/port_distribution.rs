use std::rc::Rc;

use crate::shapes::courtesy_port_distribution::CourtesyPortDistribution;
use crate::shapes::port::Port;
use crate::shapes::side::Side;
use crate::utility::display_oriented_math::*;

#[derive(Clone)]
// pub enum PortDistribution<'a, F>
pub enum PortDistribution<'a>
// where
//     F: Fn(usize, f32) -> Vec<Port> + Clone,
{
    Center {
        courtesy_port_distribution_option: Option<CourtesyPortDistribution>,
    },
    TowardsFromCurrentVert {
        distance_from_current_vert: DisplayOrientedNumber,
        courtesy_port_distribution_option: Option<CourtesyPortDistribution>,
    },
    BackwardsFromNextVert {
        distance_from_next_vert: DisplayOrientedNumber,
        courtesy_port_distribution_option: Option<CourtesyPortDistribution>,
    },
    JoinWithNext,
    UseIntersectingPortsFrom {
        side_with_possibly_intersecting_ports: Side,
        possibly_intersecting_ports: &'a Vec<Port>,
    },
    Single {
        position: DisplayOrientedNumber,
    },
    Custom {
        // port_function: F where F: Fn(f32) -> Vec<Port>,
        // port_function: Box<dyn Fn(f32) -> Vec<Port> + Clone + 'a>,
        // port_function: F,
        port_function: Rc<dyn Fn(usize, f32) -> Vec<Port> + 'a>,
    },
}

// impl<F: Fn(usize, f32) -> Vec<Port> + Clone> PortDistribution<'_, F> {
impl PortDistribution<'_> {
    pub fn should_add_a_single_halfway_port_to_if_side_length_is_less_than_master_scale(
        &self,
    ) -> bool {
        match self {
            PortDistribution::Center { .. } => true,
            PortDistribution::TowardsFromCurrentVert { .. } => true,
            PortDistribution::BackwardsFromNextVert { .. } => true,
            _ => false,
        }
    }
}

#[macro_export]
macro_rules! scale_from_alternating_vertices_and_port_distributions {
    (name: $name:expr, $($vertex:expr, $port_distribution_variant:ident $(: $last_port_distribution:expr)?),*,) => {
        {
            use crate::shapes::{ port_module::PortModule, port_distribution::default_port_distribution_from_variants};
            Vertices(
                vec![$($vertex),*]
            )
            .to_hull_scale_with_modules(
                PortModule::option_list_from_distributions(default_port_distribution_from_variants!(
                    $($port_distribution_variant $(: $last_port_distribution)?),*)),
                $name
            )
        }
    };
}
pub(crate) use scale_from_alternating_vertices_and_port_distributions;

#[macro_export]
macro_rules! default_port_distribution_from_variants {
    (None) => {
        vec![$(default_port_distribution_from_variant!(None)),*]
    };
    ($($port_distribution_variant:ident $(($side:expr, $ports:expr))? $(: $last_port_distribution:expr)?),* $(,)?) => {
        {
            use crate::shapes::port_distribution::default_port_distribution_from_variant;
            vec![$(default_port_distribution_from_variant!($port_distribution_variant $( ($side, $ports))? $(: $last_port_distribution)?)),*]
        }
    };
}
pub(crate) use default_port_distribution_from_variants;

#[macro_export]
macro_rules! default_port_distribution_from_variant {
    (Single) => {{
        use crate::{
            shapes::{port_distribution::PortDistribution, shape_constants::PortPosition},
            utility::display_oriented_math::don_float_from,
        };
        PortDistribution::Single {
            position: don_float_from(PortPosition::CENTER),
        }
    }};
    (Center) => {{
        use crate::shapes::port_distribution::PortDistribution;
        PortDistribution::Center {
            courtesy_port_distribution_option: None,
        }
    }};
    (Center: $courtesy_port_distribution:expr) => {{
        use crate::shapes::port_distribution::PortDistribution;
        PortDistribution::Center {
            courtesy_port_distribution_option: None,
        }
    }};
    (TowardsFromCurrentVert) => {{
        use crate::{
            shape_configs::shape_configs::PORT_SPACING_FROM_VERT,
            shapes::port_distribution::PortDistribution,
        };
        PortDistribution::TowardsFromCurrentVert {
            distance_from_current_vert: crate::utility::display_oriented_math::don_float_from(
                PORT_SPACING_FROM_VERT,
            ),
            courtesy_port_distribution_option: None,
        }
    }};
    (TowardsFromCurrentVert: $courtesy_port_distribution:expr) => {{
        use crate::{
            mod_configs::shape_configs::shape_configs::PORT_SPACING_FROM_VERT,
            shapes::port_distribution::PortDistribution,
        };
        PortDistribution::TowardsFromCurrentVert {
            distance_from_current_vert: crate::utility::display_oriented_math::don_float_from(
                PORT_SPACING_FROM_VERT,
            ),
            courtesy_port_distribution_option: Some($courtesy_port_distribution),
        }
    }};
    (BackwardsFromNextVert) => {{
        use crate::{
            mod_configs::shape_configs::shape_configs::PORT_SPACING_FROM_VERT,
            shapes::port_distribution::PortDistribution,
        };
        PortDistribution::BackwardsFromNextVert {
            distance_from_next_vert: crate::utility::display_oriented_math::don_float_from(
                PORT_SPACING_FROM_VERT,
            ),
            courtesy_port_distribution_option: None,
        }
    }};
    (BackwardsFromNextVert: $courtesy_port_distribution:expr) => {{
        use crate::{
            mod_configs::shape_configs::shape_configs::PORT_SPACING_FROM_VERT,
            shapes::port_distribution::PortDistribution,
        };
        PortDistribution::BackwardsFromNextVert {
            distance_from_next_vert: crate::utility::display_oriented_math::don_float_from(
                PORT_SPACING_FROM_VERT,
            ),
            courtesy_port_distribution_option: Some($courtesy_port_distribution),
        }
    }};
    (JoinWithNext) => {{
        use crate::shapes::port_distribution::PortDistribution;
        PortDistribution::JoinWithNext
    }};
    (UseIntersectingPortsFrom ($side:expr, $ports:expr)) => {{
        use crate::shapes::port_distribution::PortDistribution;
        PortDistribution::UseIntersectingPortsFrom {
            side_with_possibly_intersecting_ports: $side,
            possibly_intersecting_ports: $ports,
        }
    }};
}
pub(crate) use default_port_distribution_from_variant;
