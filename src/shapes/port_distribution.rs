use crate::shapes::courtesy_port_distribution::CourtesyPortDistribution;
use crate::shapes::port::Port;
use crate::shapes::port_flags::PortFlag;
use crate::shapes::side::Side;
use crate::utility::display_oriented_math::*;
use crate::utility::flags::Flags;

pub enum PortDistribution<'a> {
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
        side_with_possibly_intersecting_ports: &'a Side<'a>,
        possibly_intersecting_ports: &'a Vec<Port>,
    },
    Single { position: DisplayOrientedNumber},
}

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

macro_rules! scale_from_alternating_vertices_and_port_distributions {
    (name: $name:expr, $($vertex:expr, $port_distribution_variant:ident $(: $last_port_distribution:expr)?),*,) => {
        Vertices(
            vec![$($vertex),*]
        )
        .to_hull_scale_with_distributions(
            default_port_distribution_from_variants!(
                $($port_distribution_variant $(: $last_port_distribution)?),*),
            $name
        )
    };
}
pub(crate) use scale_from_alternating_vertices_and_port_distributions;

macro_rules! default_port_distribution_from_variants {
    (None) => {
        vec![$(default_port_distribution_from_variant!(None)),*]
    };
    ($($port_distribution_variant:ident $(($side:expr, $ports:expr))? $(: $last_port_distribution:expr)?),* $(,)?) => {
        vec![$(default_port_distribution_from_variant!($port_distribution_variant $( ($side, $ports))? $(: $last_port_distribution)?)),*]
    };
}
pub(crate) use default_port_distribution_from_variants;

macro_rules! default_port_distribution_from_variant {
    (None) => {
        None
    };
    (SingleWeaponInHalfWay) => {
        Some(PortDistribution::SingleWeaponInHalfWay)
    };
    (SingleWeaponOutHalfWay) => {
        Some(PortDistribution::SingleWeaponOutHalfWay)
    };
    (SingleWeaponInOutHalfWay) => {
        Some(PortDistribution::SingleWeaponInOutHalfWay)
    };
    (Center) => {
        Some(PortDistribution::Center {
            courtesy_port_distribution_option: None,
        })
    };
    (Center: $courtesy_port_distribution:expr) => {
        Some(PortDistribution::Center {
            courtesy_port_distribution_option: None,
        })
    };
    (TowardsFromCurrentVert) => {
        Some(PortDistribution::TowardsFromCurrentVert {
            distance_from_current_vert: crate::utility::display_oriented_math::don_float_from(
                PORT_SPACING_FROM_VERT,
            ),
            courtesy_port_distribution_option: None,
        })
    };
    (TowardsFromCurrentVert: $courtesy_port_distribution:expr) => {
        Some(PortDistribution::TowardsFromCurrentVert {
            distance_from_current_vert: crate::utility::display_oriented_math::don_float_from(
                PORT_SPACING_FROM_VERT,
            ),
            courtesy_port_distribution_option: Some($courtesy_port_distribution),
        })
    };
    (BackwardsFromNextVert) => {
        Some(PortDistribution::BackwardsFromNextVert {
            distance_from_next_vert: crate::utility::display_oriented_math::don_float_from(
                PORT_SPACING_FROM_VERT,
            ),
            courtesy_port_distribution_option: None,
        })
    };
    (BackwardsFromNextVert: $courtesy_port_distribution:expr) => {
        Some(PortDistribution::BackwardsFromNextVert {
            distance_from_next_vert: crate::utility::display_oriented_math::don_float_from(
                PORT_SPACING_FROM_VERT,
            ),
            courtesy_port_distribution_option: Some($courtesy_port_distribution),
        })
    };
    (JoinWithNext) => {
        Some(PortDistribution::JoinWithNext)
    };
    (UseIntersectingPortsFrom ($side:expr, $ports:expr)) => {
        Some(PortDistribution::UseIntersectingPortsFrom {
            side_with_possibly_intersecting_ports: $side,
            possibly_intersecting_ports: $ports,
        })
    };
}
pub(crate) use default_port_distribution_from_variant;
