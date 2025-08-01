use crate::{
    shapes::{port::Port, port_distribution::PortDistribution},
    utility::{display_oriented_math::don_float_from, flags::Flags},
};

use super::{
    port_flags::{no_port_flags, PortFlag},
    shape_constants::PortPosition,
};

#[derive(Clone)]
// pub struct PortModule<'a, F: Fn(usize, f32) -> Vec<Port> + Clone> {
pub struct PortModule<'a> {
    pub port_flags: Flags<PortFlag>,
    pub port_distribution: PortDistribution<'a>,
}

// impl PortModule<'_> {
//     pub fn some(
//         port_flags: Flags<PortFlag>,
//         port_distribution: PortDistribution,
//     ) -> Option<PortModule> {
//         Some(PortModule {
//             port_flags,
//             port_distribution,
//         })
//     }
// impl<F: Fn(usize, f32) -> Vec<Port> + Clone> PortModule<'_, F> {
impl PortModule<'_> {
    pub fn some(
        port_flags: Flags<PortFlag>,
        // port_distribution: PortDistribution<F>,
        port_distribution: PortDistribution,
        // ) -> Option<PortModule<F>> {
    ) -> Option<PortModule> {
        Some(PortModule {
            port_flags,
            port_distribution,
        })
    }

    // pub fn no_flags(port_distribution: PortDistribution<F>) -> Option<PortModule<F>> {
    pub fn no_flags(port_distribution: PortDistribution) -> Option<PortModule> {
        Some(PortModule {
            port_flags: no_port_flags!(),
            port_distribution,
        })
    }

    // pub fn implicit_none() -> Option<PortModule<'static, F>> {
    pub fn implicit_none() -> Option<PortModule<'static>> {
        None
    }

    // pub fn explicit_none() -> Option<PortModule<'static, F>> {
    pub fn explicit_none() -> Option<PortModule<'static>> {
        Some(PortModule {
            port_flags: Flags(vec![PortFlag::None]),
            port_distribution: PortDistribution::Single {
                position: don_float_from(PortPosition::CENTER),
            },
        })
    }

    pub fn option_list_from_distributions(
        //     port_distributions: Vec<PortDistribution<F>>,
        // ) -> Vec<Option<PortModule<F>>> {
        port_distributions: Vec<PortDistribution>,
    ) -> Vec<Option<PortModule>> {
        port_distributions
            .iter()
            .map(|port_distribution| PortModule::no_flags(port_distribution.clone()))
            .collect::<Vec<_>>()
    }
}
