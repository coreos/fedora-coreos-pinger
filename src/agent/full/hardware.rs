//! Collect summary of hardware on bare metal machines

use failure::Fallible;
use super::{lsblk, lscpu, lsmem};

#[derive(Debug)]
pub(crate) struct HardwareJSON {
    disk: lsblk::LsblkJSON,
    cpu: lscpu::LscpuJSON,
    memory: lsmem::LsmemJSON,
}

impl HardwareJSON {
    /// disk_info from: `lsblk --fs --json`
    /// cpu_info from: `lscpu --json`
    /// mem_info from: `lsmem --json` and `lsmem --summary`
    pub fn new() -> Fallible<HardwareJSON> {
        let lsblk_struct = lsblk::LsblkJSON::new()?;
        let lscpu_struct = lscpu::LscpuJSON::new()?;
        let lsmem_struct = lsmem::LsmemJSON::new()?;

        Ok(HardwareJSON {
            disk: lsblk_struct,
            cpu: lscpu_struct,
            memory: lsmem_struct,
        })

    }
}

#[test]
fn test_get_hardware_info() {
    let hw_struct = HardwareJSON::new();
    println!("{:?}", hw_struct.unwrap());
}
