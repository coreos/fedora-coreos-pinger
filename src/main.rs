#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate serde;
extern crate toml;

const CONFIG_FILE_PATH: &str = "/etc/fedora-coreos-metrics/client.conf";

mod errors {
    error_chain!{
        errors {
            InvalidMetricsLevel(p: String) {
                description("invalid metrics collection level")
                display("invalid metrics collection level '{}'", p)
            }
        }
    }
}

use errors::*;
use serde::Deserialize;
use std::io::Read;

quick_main!(run);

#[derive(Debug, Deserialize)]
pub struct ConfigFragment {
    pub metrics: Option<MetricsFragment>,
}

#[derive(Debug, Deserialize)]
pub struct MetricsFragment {
    pub level: Option<String>,
}

/// Parse the metrics.level key from CONFIG_FILE_PATH, and check that the key
/// is set to one of the accepted telemetry levels. If not an accepted level,
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
        .chain_err(||"failed to parse TOML")?;

    let metrics_level = config.metrics.unwrap().level.unwrap();

    match metrics_level.as_str() {
        "off" | "minimal" | "full" => info!("Metrics collection set at level '{}'.", metrics_level),
        _ => return Err(errors::ErrorKind::InvalidMetricsLevel(metrics_level).into()),
    }

    Ok(())
}
