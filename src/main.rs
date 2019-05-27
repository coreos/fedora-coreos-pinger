#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate glib;
#[macro_use]
extern crate log;

use glib::{KeyFile, KeyFileFlags};

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

quick_main!(run);

fn run() -> Result<()> {
    let config_file = KeyFile::new();
    config_file.load_from_file(CONFIG_FILE_PATH, KeyFileFlags::NONE)
               .chain_err(|| "Unable to open config file")?;

    let metrics_level = config_file.get_string("metrics", "level")
                                   .chain_err(|| "Unable to get metrics.level field")?
                                   .to_string();

    match metrics_level.as_str() {
        "off" | "minimal" | "full" => info!("Metrics collection set at level '{}'.", metrics_level),
        _ => return Err(errors::ErrorKind::InvalidMetricsLevel(metrics_level).into()),
    }

    Ok(())
}
