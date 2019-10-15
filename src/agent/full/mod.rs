//! Module to collect data under `Full` level

mod hardware;
mod lsblk;
mod lscpu;
mod lsmem;

#[cfg(test)]
mod mock_tests;

use failure::Fallible;

#[derive(Debug)]
pub(crate) struct IdentityFull {
    hardware: hardware::HardwareJSON,
}

impl IdentityFull {
    pub(crate) fn new() -> Fallible<IdentityFull> {
        let hw = hardware::HardwareJSON::new()?;
        Ok(IdentityFull {
            hardware: hw,
        })
    }
}
