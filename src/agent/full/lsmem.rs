//! Struct for `lsmem --json`
use serde::Deserialize;
use failure::{bail, format_err, Fallible, ResultExt};


#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct LsmemJSON {
    memory: Vec<MemoryJSON>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct MemoryJSON {
    size: String,
    state: String,
    removable: bool,
    block: String,
}

impl LsmemJSON {
    pub(crate) fn new() -> Fallible<LsmemJSON> {
        let mut cmd = std::process::Command::new("lsmem");
        let cmdrun = cmd
            .arg("--json")
            .output()
            .with_context(|e| format_err!("failed to run lsmem --json: {}", e))?;

        if !cmdrun.status.success() {
            bail!(
                "lsmem --json failed:\n{}",
                String::from_utf8_lossy(&cmdrun.stderr)
            );
        }
        Ok(serde_json::from_slice(&cmdrun.stdout)?)
    }
}
