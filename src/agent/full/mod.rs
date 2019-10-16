//! Module to collect data under `Full` level

mod hardware;
mod lsblk;
mod lscpu;
mod lsmem;
mod network;

#[cfg(test)]
mod mock_tests;

use crate::agent::minimal;
use failure::Fallible;
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct IdentityFull {
    hardware: Option<hardware::HardwareJSON>,
    network: HashMap<String, String>,
}

impl IdentityFull {
    pub(crate) fn new() -> Fallible<IdentityFull> {
        // only collect hardware information on bare-metal systems
        let platform = minimal::platform::get_platform(minimal::KERNEL_ARGS_FILE)?;
        let network = network::get_network()?;
        match platform.as_str() {
            "metal" => {
                let hw = hardware::HardwareJSON::new()?;
                Ok(IdentityFull {
                    hardware: Some(hw),
                    network,
                })
            }
            _ => Ok(IdentityFull {
                hardware: None,
                network,
            }),
        }
    }
}
