mod platform;

use crate::config::inputs;
use failure::{Fallible, ResultExt};
use serde::Serialize;
use std::collections::HashMap;
use maplit;

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
            level @ "minimal" | level @ "full" => Self::try_default(level).context(format!("failed to build '{}' identity", level))?,
            &_ => Self::try_default("minimal").context("failed to build 'minimal' identity")?,
        };

        Ok(id)
    }

    /// Try to fetch default data
    pub fn try_default(level: &str) -> Fallible<Self> {
        let platform = platform::read_id(KERNEL_ARGS_FILE)?;

        let id = match level {
                    "minimal" | "full" => Self {
                                    level: level.to_string(),
                                    platform,
                                },
                    &_ => Self {
                                    level: "minimal".to_string(),
                                    platform,
                                },
                };

        Ok(id)
    }

    /// Getter for collected data, returned as a HashMap
    pub fn get_data(&self) -> HashMap<String, String> {
        let vars = maplit::hashmap!{
            "level".to_string() => self.level.clone(),
            "platform".to_string() => self.platform.clone(),
        };

        // Insert data specific to different levels
        match self.level.as_str() {
            "minimal" | "full" => (),
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
        let vars = id.get_data();

        assert!(vars.contains_key("level"));
        assert!(vars.contains_key("platform"));
    }

    #[test]
    fn test_full() {
        let id = Identity::mock_default("full");
        let vars = id.get_data();

        assert!(vars.contains_key("level"));
        assert!(vars.contains_key("platform"));
    }
}
