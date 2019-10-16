//! Struct for `lsmem --json`
use serde::{Deserialize, Deserializer};
use serde::de::{self, Unexpected};
use failure::{bail, format_err, Fallible, ResultExt};
use std::fmt;

#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct LsmemJSON {
    memory: Vec<MemoryJSON>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct MemoryJSON {
    size: String,
    state: String,
    #[serde(deserialize_with = "deserialize_bool_or_string")]
    removable: bool,
    block: String,
}

impl LsmemJSON {
    pub(crate) fn new() -> Fallible<LsmemJSON> {
        let mut cmd = std::process::Command::new("lsmem");
        let cmdrun = cmd
            .arg("--json")
            .output()
            .with_context(|e| format_err!("failed to run lsmem --json: {}", e))?;

        if !cmdrun.status.success() {
            bail!(
                "lsmem --json failed:\n{}",
                String::from_utf8_lossy(&cmdrun.stderr)
            );
        }
        Ok(serde_json::from_slice(&cmdrun.stdout)?)
    }
}

struct DeserializeBoolOrString;

impl<'de> de::Visitor<'de> for DeserializeBoolOrString {
    type Value = bool;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a bool or a string")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "yes" {
            Ok(true)
        } else if v == "no" {
            Ok(false)
        } else {
            Err(E::invalid_value(Unexpected::Str(v), &self))
        }
    }
}

fn deserialize_bool_or_string<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(DeserializeBoolOrString)
}