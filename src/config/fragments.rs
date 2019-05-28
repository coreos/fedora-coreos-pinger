use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct ConfigFragment {
    pub(crate) collecting: CollectingFragment,
    pub(crate) reporting: ReportingFragment,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CollectingFragment {
    pub(crate) level: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ReportingFragment {
    pub(crate) enabled: bool,
}
