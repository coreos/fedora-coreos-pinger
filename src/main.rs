#![recursion_limit = "1024"]

#[macro_use]
extern crate failure;
extern crate glob;
#[macro_use]
extern crate log;
extern crate serde;
extern crate toml;

use failure::{Fallible,ResultExt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ConfigInput {
    pub collecting: CollectingInput,
    pub reporting: ReportingInput,
}

impl ConfigInput {
    /// Read config fragments and merge them into a single config.
    pub fn read_configs(dirs: &[&str], app_name: &str) -> Fallible<Self> {
        use std::io::Read;

        let mut fragments = Vec::new();
        for prefix in dirs {
            let dir = format!("{}/{}/config.d", prefix, app_name);
            debug!("scanning configuration directory '{}'", dir);

            let wildcard = format!("{}/*.toml", dir);
            let toml_files = glob::glob(&wildcard)?;
            for fpath in toml_files.filter_map(Result::ok) {
                trace!("reading config fragment '{}'", fpath.display());

                let fp = std::fs::File::open(&fpath)
                    .context(format!("failed to open file '{}'", fpath.display()))?;
                let mut bufrd = std::io::BufReader::new(fp);
                let mut content = vec![];
                bufrd
                    .read_to_end(&mut content)
                    .context(format!("failed to read content of '{}'", fpath.display()))?;
                let frag: ConfigFragment =
                    toml::from_slice(&content).context("failed to parse TOML")?;

                fragments.push(frag);
            }
        }

        let cfg = Self::merge_fragments(fragments);
        Ok(cfg)
    }

    /// Merge multiple fragments into a single configuration.
    fn merge_fragments(fragments: Vec<ConfigFragment>) -> Self {
        let mut collecting_configs = vec![];
        let mut reporting_configs = vec![];

        for snip in fragments {
            collecting_configs.push(snip.collecting);
            reporting_configs.push(snip.reporting);
        }

        Self {
            collecting: CollectingInput::from_fragments(collecting_configs),
            reporting: ReportingInput::from_fragments(reporting_configs),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct CollectingInput {
    pub level: String,
}

impl CollectingInput {
    fn from_fragments(fragments: Vec<CollectingFragment>) -> Self {
        let mut cfg = Self {
            level: String::new(),
        };

        for snip in fragments {
            cfg.level = snip.level;
        }

        cfg
    }
}

#[derive(Debug, Serialize)]
pub struct ReportingInput {
    pub enabled: bool,
}

impl ReportingInput {
    fn from_fragments(fragments: Vec<ReportingFragment>) -> Self {
        let mut cfg = Self {
            enabled: true,
        };

        for snip in fragments {
            cfg.enabled = snip.enabled;
        }

        cfg
    }
}

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

/// Parse the reporting.enabled and collecting.level keys from config fragments,
/// and check that the keys are set to a valid telemetry setting. If not,
/// or in case of other error, return non-zero.
fn main() -> Fallible<()> {
    let dirs = vec!["/usr/lib", "/run", "/etc"];
    // TODO(rfairley): get "fedora-coreos-metrics-client" using crate_name! macro.
    let config = ConfigInput::read_configs(&dirs, "fedora-coreos-metrics-client")?;

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
