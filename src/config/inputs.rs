use crate::config::fragments;

use failure::ResultExt;
use log::debug;
use serde::Serialize;
use std::{collections, fs, path};

fn add_snippets_to_tree(
    dir: &path::PathBuf,
    tree: &mut collections::BTreeMap<String, path::PathBuf>
) -> failure::Fallible<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            debug!("found fragment '{}'", path.display());

            if !path.is_dir() && path.extension().unwrap() == "toml" {
                let key = path.file_name().unwrap().to_str().unwrap().to_owned();
                if !tree.contains_key(&key) {
                    debug!("adding fragment with filename '{}' to config", key);
                    tree.insert(key, path);
                }
            }
        }
    }
    Ok(())
}

#[derive(Debug, Serialize)]
pub(crate) struct ConfigInput {
    pub(crate) collecting: CollectingInput,
    pub(crate) reporting: ReportingInput,
}

impl ConfigInput {
    /// Read config fragments and merge them into a single config.
    pub(crate) fn read_configs(
        dirs: &[path::PathBuf],
        app_name: &str
    ) -> failure::Fallible<Self> {
        let mut fragments = collections::BTreeMap::new();
        for prefix in dirs {
            let dir = path::PathBuf::from(format!("{}/{}/config.d", prefix.as_path().display(), app_name));
            debug!("scanning configuration directory '{}'", dir.display());

            add_snippets_to_tree(&dir, &mut fragments)?;
        }

        let cfg = Self::merge_fragments(fragments)?;
        Ok(cfg)
    }

    /// Merge multiple fragments into a single configuration.
    fn merge_fragments(
        fragments: collections::BTreeMap<String, path::PathBuf>
    ) -> failure::Fallible<Self> {
        use std::io::Read;

        let mut collecting_configs = vec![];
        let mut reporting_configs = vec![];

        for (_snip, path) in fragments {
            let fp = std::fs::File::open(&path)
                .context(format!("failed to open file '{}'", path.display()))?;
            let mut bufrd = std::io::BufReader::new(fp);
            let mut content = vec![];
            bufrd
                .read_to_end(&mut content)
                .context(format!("failed to read content of '{}'", path.display()))?;
            let config: fragments::ConfigFragment =
                toml::from_slice(&content).context("failed to parse TOML")?;

            collecting_configs.push(config.collecting);
            reporting_configs.push(config.reporting);
        }

        let cfg = Self {
            collecting: CollectingInput::from_fragments(collecting_configs),
            reporting: ReportingInput::from_fragments(reporting_configs),
        };
        Ok(cfg)
    }
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct CollectingInput {
    pub(crate) level: String,
}

impl CollectingInput {
    fn from_fragments(fragments: Vec<fragments::CollectingFragment>) -> Self {
        let mut cfg = Self {
            level: String::new(),
        };

        for snip in fragments {
            cfg.level = snip.level;
        }

        cfg
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct ReportingInput {
    pub(crate) enabled: bool,
}

impl ReportingInput {
    fn from_fragments(fragments: Vec<fragments::ReportingFragment>) -> Self {
        let mut cfg = Self {
            enabled: true,
        };

        for snip in fragments {
            cfg.enabled = snip.enabled;
        }

        cfg
    }
}
