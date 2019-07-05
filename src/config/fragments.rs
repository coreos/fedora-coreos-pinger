//! TOML configuration fragments.

use serde::Deserialize;

/// Pinger config.
#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct ConfigFragment {
    pub(crate) collecting: Option<CollectingFragment>,
    pub(crate) reporting: Option<ReportingFragment>,
}

/// Collecting config group.
#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct CollectingFragment {
    /// Collection level, may be `"minimal"` or `"full"` (default: "minimal").
    pub(crate) level: Option<String>,
}

/// Reporting config group.
#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct ReportingFragment {
    /// Reporting enablement flag (required).
    pub(crate) enabled: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    #[test]
    fn basic_dist_config_default() {
        let fp = std::fs::File::open("dist/00-default.toml").unwrap();
        let mut bufrd = std::io::BufReader::new(fp);
        let mut content = vec![];
        bufrd.read_to_end(&mut content).unwrap();
        let cfg: ConfigFragment = toml::from_slice(&content).unwrap();

        let expected = ConfigFragment {
            collecting: Some(CollectingFragment {
                level: Some("minimal".to_string()),
            }),
            reporting: Some(ReportingFragment {
                enabled: Some(true),
            }),
        };

        assert_eq!(cfg, expected);
    }
}
