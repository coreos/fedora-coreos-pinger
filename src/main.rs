//! Telemetry service for FCOS.

extern crate slog;
#[macro_use]
extern crate slog_scope;

#[cfg(test)]
extern crate mockito;
#[cfg(test)]
extern crate tempfile;

/// Collect config from files.
mod config;
/// rpm-ostree client.
mod rpm_ostree;
/// utility functions
mod util;
/// agent module
mod agent;

use clap::{Arg, crate_authors, crate_description, crate_name, crate_version};
use config::inputs;
use failure::{bail, ResultExt, Fallible};
use log::LevelFilter;

/// Parse the reporting.enabled and collecting.level keys from config fragments,
/// and check that the keys are set to a valid telemetry setting. If not,
/// or in case of other error, return non-zero.
fn check_config(config: &inputs::ConfigInput) -> Fallible<bool> {
    if config.reporting.enabled.unwrap() {
        println!("Reporting enabled.");

        let collecting_level = &config.collecting.level;
        match collecting_level.as_str() {
            "minimal" | "full" => println!("Collection set at level '{}'.", collecting_level),
            _ => bail!("invalid collection level '{}'", collecting_level),
        }

        Ok(true)
    } else {
        println!("Reporting disabled.");

        Ok(false)
    }
}

fn send_data(agent: &agent::Agent) -> Fallible<()> {
    // TODO: Send data to remote endpoint
    // Currently only prints the Agent struct
    println!("{:?}", agent);

    Ok(())
}

fn main() -> Fallible<()> {
    let matches = clap::app_from_crate!()
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets log verbosity level"))
        .get_matches();

    let log_level = match matches.occurrences_of("v") {
        0 => LevelFilter::Warn,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        3 | _ => LevelFilter::Trace,
    };
    env_logger::Builder::from_default_env()
        .default_format_timestamp(false)
        .default_format_module_path(false)
        .filter(None, log_level)
        .try_init()?;

    let dirs = vec![
        String::from("/usr/lib"),
        String::from("/run"),
        String::from("/etc"),
    ];
    let config = inputs::ConfigInput::read_configs(dirs, crate_name!())
        .context("failed to read configuration input")?;

    let is_enabled = check_config(&config)?;

    // Collect the data if enabled
    if is_enabled {
        let agent = agent::Agent::new(&config.collecting)?;
        // Send to the remote endpoint
        send_data(&agent)?;
    }

    Ok(())
}
