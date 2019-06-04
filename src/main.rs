mod config;

use config::inputs;
use failure::bail;
use log::info;
use std::path;

/// Parse the reporting.enabled and collecting.level keys from config fragments,
/// and check that the keys are set to a valid telemetry setting. If not,
/// or in case of other error, return non-zero.
fn check_metrics_config(config: inputs::ConfigInput) -> failure::Fallible<()> {
    if config.reporting.enabled.unwrap() {
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

fn main() -> failure::Fallible<()> {
    env_logger::Builder::from_default_env()
        .default_format_timestamp(false)
        .default_format_module_path(false)
        .default_format_level(false)
        // TODO(rfairley): Make log level a CLI option.
        .filter(None, log::LevelFilter::Info)
        .try_init()?;

    let dirs = vec![
        path::PathBuf::from("/etc"),
        path::PathBuf::from("/run"),
        path::PathBuf::from("/usr/lib"),
    ];
    // TODO(rfairley): get "fedora-coreos-metrics-client" using crate_name! macro.
    let config = inputs::ConfigInput::read_configs(&dirs, "fedora-coreos-metrics-client")?;

    check_metrics_config(config)?;

    Ok(())
}
