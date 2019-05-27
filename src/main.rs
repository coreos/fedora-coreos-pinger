#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate serde;
extern crate toml;

const CONFIG_FILE_PATH: &str = "/etc/fedora-coreos-metrics/client.conf";

mod errors {
    error_chain!{}
}

use errors::*;
use serde::Deserialize;
use std::io::Read;

#[derive(Debug, Deserialize)]
pub struct ConfigFragment {
    pub collecting: CollectingFragment,
    pub reporting: ReportingFragment,
}

#[derive(Debug, Deserialize)]
pub struct CollectingFragment {
    pub level: String,
}

#[derive(Debug, Deserialize)]
pub struct ReportingFragment {
    pub enabled: bool,
}

quick_main!(run);

/// Parse the reporting.enabled and collecting.level keys from config fragments,
/// and check that the keys are set to a valid telemetry setting. If not,
/// or in case of other error, return non-zero.
fn run() -> Result<()> {
    let fp = std::fs::File::open(CONFIG_FILE_PATH)
        .chain_err(|| "failed to open config file")?;
    let mut bufrd = std::io::BufReader::new(fp);
    let mut content = vec![];
    bufrd
        .read_to_end(&mut content)
        .chain_err(|| "failed to read config file")?;
    let config: ConfigFragment =
        toml::from_slice(&content)
        .chain_err(|| "failed to parse TOML")?;

    let reporting_enabled = config.reporting.enabled;
    if reporting_enabled {
        info!("Metrics reporting enabled.");

        let collecting_level = config.collecting.level;
        match collecting_level.as_str() {
            "minimal" | "full" => info!("Metrics collection set at level '{}'.", collecting_level),
            _ => bail!("invalid metrics collection level '{}'", collecting_level),
        }
    } else {
        info!("Metrics reporting disabled.");
    }

    Ok(())
}
