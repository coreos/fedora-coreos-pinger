mod platform;

use crate::config::inputs;
use failure::{Fallible, ResultExt};
use serde::Serialize;
use std::collections::HashMap;

/// Kernel arguments location
static KERNEL_ARGS_FILE: &str = "/proc/cmdline";

/// Agent identity.
#[derive(Debug, Serialize)]
pub(crate) struct Identity {
    /// Collecting level
    pub(crate) level: String,
    /// OS platform.
    pub(crate) platform: String,
}

impl Identity {
    /// Create from configuration.
    pub(crate) fn new(cfg: &inputs::CollectingInput) -> Fallible<Self> {
        let collecting_level = &cfg.level;
        let id = match collecting_level.as_str() {
            "minimal" => Self::try_default("minimal").context("failed to build minimal (default) identity")?,
            "full" => Self::try_default("full").context("failed to build full identity")?,
            &_ => Self::try_default("minimal").context("failed to build minimal (default) identity")?,
        };

        Ok(id)
    }

    /// Try to fetch default data
    pub fn try_default(level: &str) -> Fallible<Self> {
        let platform = platform::read_id(KERNEL_ARGS_FILE)?;

        let id = match level {
                    "minimal" => Self {
                                    level: String::from("minimal"),
                                    platform,
                                },
                    "full" => Self {
                                    level: String::from("full"),
                                    platform
                                },
                    &_ => Self {
                                    level: String::from("minimal"),
                                    platform,
                                },
                };

        Ok(id)
    }

    /// Getter for collected metrics, returned as a HashMap
    pub fn get_metrics(&self) -> HashMap<String, String> {
        let mut vars = HashMap::new();

        vars.insert("level".to_string(), self.level.clone());
        vars.insert("platform".to_string(), self.platform.clone());

        match self.level.as_str() {
            "minimal" => (),
            "full" => (),
            &_ => (),
        };

        vars
    }

    #[cfg(test)]
    pub(crate) fn mock_default(level: &str) -> Self {
        match level {
            "minimal" => return Self {
                            level: String::from("minimal"),
                            platform: "mock-qemu".to_string(),
                        },
            "full" => return Self {
                            level: String::from("full"),
                            platform: "mock-gcp".to_string(),
                        },
            &_ => return Self {
                            level: String::from("minimal"),
                            platform: "mock-qemu".to_string(),
                        },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimal() {
        let id = Identity::mock_default("minimal");
        let vars = id.get_metrics();

        assert!(vars.contains_key("level"));
        assert!(vars.contains_key("platform"));
    }

    #[test]
    fn test_full() {
        let id = Identity::mock_default("full");
        let vars = id.get_metrics();

        assert!(vars.contains_key("level"));
        assert!(vars.contains_key("platform"));
    }
}
