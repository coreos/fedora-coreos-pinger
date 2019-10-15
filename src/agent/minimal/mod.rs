mod platform;
mod os_release;
mod instance_type;

use crate::rpm_ostree;
use failure::{Fallible, ResultExt};
use serde::Serialize;
#[cfg(test)]
use std::collections::HashMap;
#[cfg(test)]
use maplit;

/// Kernel arguments location
static KERNEL_ARGS_FILE: &str = "/proc/cmdline";
/// aleph version file
static OS_ALEPH_VERSION_FILE: &str = "/.coreos-aleph-version.json";
/// Afterburn cloud metadata location
static AFTERBURN_METADATA: &str = "/run/metadata/afterburn";

/// Agent identity.
#[derive(Debug, Serialize)]
pub(crate) struct IdentityMin {
    /// OS platform
    pub(crate) platform: String,
    /// Original OS version
    pub(crate) original_os_version: String,
    /// Current OS version
    pub(crate) current_os_version: String,
    /// Instance type if on cloud platform
    pub(crate) instance_type: Option<String>,
}

impl IdentityMin {
    pub(crate) fn new() -> Fallible<Self> {
        Ok(Self::collect_minimal_data().context(format!("failed to build 'minimal' identity"))?)
    }

    /// Try to fetch data in minimal level
    pub fn collect_minimal_data() -> Fallible<Self> {
        let platform = platform::get_platform(KERNEL_ARGS_FILE)?;
        let original_os_version = os_release::read_original_os_version(OS_ALEPH_VERSION_FILE)?;
        let current_os_version = rpm_ostree::booted()?.version;
        let instance_type: Option<String> = match platform.as_str() {
            "aliyun" | "aws" | "azure" | "gcp" | "openstack" => Some(instance_type::read_instance_type(AFTERBURN_METADATA, platform.as_str())?),
            _ => None,
        };

        Ok(Self {
            platform,
            original_os_version,
            current_os_version,
            instance_type,
        })
    }

    #[cfg(test)]
    /// Getter for collected data, returned as a HashMap
    fn get_data(&self) -> HashMap<String, String> {
        maplit::hashmap!{
            "platform".to_string() => self.platform.clone(),
            "original_os_version".to_string() => self.original_os_version.clone(),
            "current_os_version".to_string() => self.current_os_version.clone(),
            "instance_type".to_string() => match &self.instance_type {
                Some(v) => v.clone(),
                None => "".to_string(),
            },
        }
    }

    #[cfg(test)]
    pub(crate) fn mock_default() -> Self {
        Self {
            platform: "mock-qemu".to_string(),
            original_os_version: "30.20190923.dev.2-2".to_string(),
            current_os_version: "mock-os-version".to_string(),
            instance_type: Some("mock-instance-type".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimal() {
        let id = IdentityMin::mock_default();
        let vars = id.get_data();

        // check if the keys exist
        assert!(vars.contains_key("platform"));
        assert!(vars.contains_key("original_os_version"));
        assert!(vars.contains_key("current_os_version"));
        assert!(vars.contains_key("instance_type"));

        // check if the values match
        assert_eq!(vars.get("platform"), Some(&"mock-qemu".to_string()));
        assert_eq!(vars.get("original_os_version"), Some(&"30.20190923.dev.2-2".to_string()));
        assert_eq!(vars.get("current_os_version"), Some(&"mock-os-version".to_string()));
        assert_eq!(vars.get("instance_type"), Some(&"mock-instance-type".to_string()));
    }
}
