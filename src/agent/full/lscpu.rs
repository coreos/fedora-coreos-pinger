//! Struct for `lscpu --json`
use serde::Deserialize;
use failure::{bail, format_err, Fallible, ResultExt};


#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct LscpuJSON {
    lscpu: Vec<CPUInfoJSON>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct CPUInfoJSON {
    field: String,
    data: String,
}

impl LscpuJSON {
    pub(crate) fn new() -> Fallible<LscpuJSON> {
        let mut cmd = std::process::Command::new("lscpu");
        let cmdrun = cmd
            .arg("--json")
            .output()
            .with_context(|e| format_err!("failed to run lscpu --json: {}", e))?;

        if !cmdrun.status.success() {
            bail!(
                "lscpu --json failed:\n{}",
                String::from_utf8_lossy(&cmdrun.stderr)
            );
        }
        Ok(serde_json::from_slice(&cmdrun.stdout)?)
    }
}