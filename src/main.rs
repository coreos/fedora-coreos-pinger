//! Telemetry service for FCOS.

extern crate base64;
#[macro_use]
extern crate error_chain;
extern crate openssh_keys;
extern crate openssl;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_xml_rs;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;
#[macro_use]
extern crate slog_scope;

#[cfg(test)]
extern crate mockito;

/// Collect config from files.
mod config;
/// `Minimal` Agent identity.
mod minimal;
/// Provider metadata
mod providers;
/// Generic retrying function
mod retry;
/// A library for consistent and reliable error handling
mod errors;
/// rpm-ostree client.
mod rpm_ostree;

use clap::{Arg, crate_authors, crate_description, crate_name, crate_version};
use config::inputs;
use failure::{bail, ResultExt};
use log::LevelFilter;

/// Parse the reporting.enabled and collecting.level keys from config fragments,
/// and check that the keys are set to a valid telemetry setting. If not,
/// or in case of other error, return non-zero.
fn check_config(config: &inputs::ConfigInput) -> failure::Fallible<()> {
    if config.reporting.enabled.unwrap() {
        println!("Reporting enabled.");

        let collecting_level = &config.collecting.level;
        match collecting_level.as_str() {
            "minimal" | "full" => println!("Collection set at level '{}'.", collecting_level),
            _ => bail!("invalid collection level '{}'", collecting_level),
        }
    } else {
        println!("Reporting disabled.");
    }

    Ok(())
}

fn send_data(id: &minimal::Identity) -> failure::Fallible<()> {
    // TODO: Send data to remote endpoint
    for (key, value) in id.get_data() {
        println!("{}: {}", key, value);
    }

    Ok(())
}

fn main() -> failure::Fallible<()> {
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

    check_config(&config)?;

    // Collect the data
    let id = minimal::Identity::new(&config.collecting)?;
    // Send to the remote endpoint
    send_data(&id)?;

    Ok(())
}
