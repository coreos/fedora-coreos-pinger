//! Agent module for level `minimal` and `full`
pub mod full;
pub mod minimal;

use crate::config::inputs;
use failure::{bail, Fallible};

#[derive(Debug)]
pub(crate) struct Agent {
    /// Collecting level
    level: String,
    /// Minimal data
    minimal: minimal::IdentityMin,
    /// Full data
    full: Option<full::IdentityFull>,
}

impl Agent {
    pub(crate) fn new(cfg: &inputs::CollectingInput) -> Fallible<Agent> {
        let collecting_level = &cfg.level;
        match collecting_level.as_str() {
            "minimal" => {
                return Ok(Agent {
                    level: "minimal".to_string(),
                    minimal: minimal::IdentityMin::new()?,
                    full: None,
                })
            }
            "full" => {
                return Ok(Agent {
                    level: "full".to_string(),
                    minimal: minimal::IdentityMin::new()?,
                    full: Some(full::IdentityFull::new()?),
                })
            }
            _ => bail!("Invalid collecting level: {}", collecting_level),
        }
    }
}
