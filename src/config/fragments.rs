use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct ConfigFragment {
    pub(crate) collecting: CollectingFragment,
    pub(crate) reporting: ReportingFragment,
}

#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct CollectingFragment {
    pub(crate) level: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct ReportingFragment {
    pub(crate) enabled: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    #[test]
    fn basic_dist_config_default() {
        let fp = std::fs::File::open("dist/0000-client-default.toml").unwrap();
        let mut bufrd = std::io::BufReader::new(fp);
        let mut content = vec![];
        bufrd.read_to_end(&mut content).unwrap();
        let cfg: ConfigFragment = toml::from_slice(&content).unwrap();

        let expected = ConfigFragment {
            collecting: CollectingFragment {
                level: String::from("minimal")
            },
            reporting: ReportingFragment {
                enabled: true,
            },
        };

        assert_eq!(cfg, expected);
    }
}
