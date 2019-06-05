mod config;

use clap::{app_from_crate, Arg, crate_authors, crate_description, crate_name, crate_version};
use config::inputs;
use failure::bail;
use failure::ResultExt;
use std::path;

/// Parse the reporting.enabled and collecting.level keys from config fragments,
/// and check that the keys are set to a valid telemetry setting. If not,
/// or in case of other error, return non-zero.
fn check_metrics_config(config: inputs::ConfigInput) -> failure::Fallible<()> {
    if config.reporting.enabled.unwrap() {
        println!("Metrics reporting enabled.");

        let collecting_level = config.collecting.level;
        match collecting_level.as_str() {
            "minimal" | "full" => println!("Metrics collection set at level '{}'.", collecting_level),
            _ => bail!("invalid metrics collection level '{}'", collecting_level),
        }
    } else {
        println!("Metrics reporting disabled.");
    }

    Ok(())
}

fn main() -> failure::Fallible<()> {
    let matches = app_from_crate!()
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets log verbosity level"))
        .get_matches();

    let log_level = match matches.occurrences_of("v") {
        0 => log::LevelFilter::Warn,
        1 => log::LevelFilter::Info,
        2 => log::LevelFilter::Debug,
        3 | _ => log::LevelFilter::Trace,
    };
    env_logger::Builder::from_default_env()
        .default_format_timestamp(false)
        .default_format_module_path(false)
        .filter(None, log_level)
        .try_init()?;

    let dirs = vec![
        path::PathBuf::from("/etc"),
        path::PathBuf::from("/run"),
        path::PathBuf::from("/usr/lib"),
    ];
    let config = inputs::ConfigInput::read_configs(&dirs, crate_name!())
        .context("failed to read configuration input")?;

    check_metrics_config(config)?;

    Ok(())
}
