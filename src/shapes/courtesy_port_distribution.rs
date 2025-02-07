#[derive(Clone)]
pub enum CourtesyPortDistribution {
    HalfwayToEnd,
    ContinuePattern,
}

macro_rules! add_courtesy_ports {
    (ports: $ports:expr, side: $side:expr, port_count: $port_count:expr, port_module: $port_module:expr) => {
        {
            use crate::shapes::courtesy_port_distribution::CourtesyPortDistribution;
            use crate::utility::flags::Flags;
            use crate::shapes::port_flags::PortFlag;

            let side_length = $side.get_side_length();

            if $side.get_side_length() > $port_count * MASTER_SCALE {
                if let PortDistribution::TowardsFromCurrentVert {
                    courtesy_port_distribution_option,
                    ..
                } = &$port_module.port_distribution
                {
                    if let Some(courtesy_port_distribution) = courtesy_port_distribution_option {
                        match courtesy_port_distribution {
                            CourtesyPortDistribution::HalfwayToEnd => {
                                $ports.push(Port {
                                    side_index: $side.index,
                                    position: don_fraction_from(
                                        PortPosition::CURRENT_VERT * side_length
                                            + (side_length + $port_count * MASTER_SCALE) * 0.5,
                                        side_length,
                                    ),
                                    flags: Flags::<PortFlag>::default(),
                                });
                            }
                            CourtesyPortDistribution::ContinuePattern => {
                                let possible_courtesy_port = Port {
                                    side_index: $side.index,
                                    position: get_port_position_of_distribution(
                                        &Some(&$port_module.port_distribution),
                                        &$side,
                                        &$port_count,
                                        $port_count as usize,
                                    )
                                    .unwrap(),
                                    flags: Flags::<PortFlag>::default(),
                                };
                                if possible_courtesy_port.has_valid_position() {
                                    $ports.push(possible_courtesy_port)
                                }
                            }
                        }
                    }
                } else if let PortDistribution::BackwardsFromNextVert {
                    courtesy_port_distribution_option,
                    ..
                } = &$port_module.port_distribution
                {
                    if let Some(courtesy_port_distribution) = courtesy_port_distribution_option {
                        match courtesy_port_distribution {
                            CourtesyPortDistribution::HalfwayToEnd => {
                                $ports.push(Port {
                                    side_index: $side.index,
                                    position: DisplayOrientedNumber::Fraction {
                                        numerator: Box::new(don_float_from(
                                            PortPosition::NEXT_VERT * side_length
                                                - (side_length + $port_count * MASTER_SCALE) * 0.5,
                                        )),
                                        denominator: Box::new(don_float_from(side_length)),
                                    },
                                    flags: Flags::<PortFlag>::default(),
                                });
                            }
                            CourtesyPortDistribution::ContinuePattern => {
                                let possible_courtesy_port = Port {
                                    side_index: $side.index,
                                    position: get_port_position_of_distribution(
                                        &Some(&$port_module.port_distribution),
                                        &$side,
                                        &$port_count,
                                        $port_count as usize,
                                    )
                                    .unwrap(),
                                    flags: Flags::<PortFlag>::default(),
                                };
                                if possible_courtesy_port.has_valid_position() {
                                    $ports.push(possible_courtesy_port)
                                }
                            }
                        }
                    }
                } else if let PortDistribution::Center {
                    // courtesy_port_distribution_option,
                    ..
                } = $port_module.port_distribution
                {
                    // if let Some(courtesy_port_distribution) = courtesy_port_distribution_option {
                    //     {}
                    // }
                }
            }
        }
    };
}
pub(crate) use add_courtesy_ports;
