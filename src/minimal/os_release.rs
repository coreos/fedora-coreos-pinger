//! OS version parsing - utility functions

use failure::{bail, format_err, Fallible, ResultExt};
use std::io::Read;
use std::{fs, io};
use serde_json;

/// OS version flag.
static OS_VERSION_FLAG: &str = "VERSION";

/// Read aleph version info from os version json file.
pub(crate) fn read_original_os_version<T>(file_path: T) -> Fallible<String>
where
    T: AsRef<str>,
{
    // open the os release file
    let fpath = file_path.as_ref();
    let file = fs::File::open(fpath)
        .with_context(|e| format_err!("failed to open aleph version file {}: {}", fpath, e))?;

    // parse the content
    let json: serde_json::Value = serde_json::from_reader(file)
    .expect("failed to parse aleph version file as JSON");
    let build: String = json.get("build")
    .expect("aleph version file does not contain 'build' key")
    .to_string();

    Ok(build)

}

/// Read current os version info from os release file.
pub(crate) fn read_current_os_version<T>(file_path: T) -> Fallible<String>
where
    T: AsRef<str>,
{
    // open the os release file
    let fpath = file_path.as_ref();
    let file = fs::File::open(fpath)
        .with_context(|e| format_err!("failed to open os-release file {}: {}", fpath, e))?;

    // read content
    let mut bufrd = io::BufReader::new(file);
    let mut contents = String::new();
    bufrd
        .read_to_string(&mut contents)
        .with_context(|e| format_err!("failed to read os-release file {}: {}", fpath, e))?;

    // lookup flag by key name
    match find_flag_value(OS_VERSION_FLAG, &contents) {
        Some(version) => {
            log::trace!("found os version: {}", version);
            Ok(version)
        }
        None => bail!(
            "could not find flag '{}' in {}",
            OS_VERSION_FLAG,
            fpath
        ),
    }
}

/// Find VERSION flag in os-release contents.
fn find_flag_value(flagname: &str, contents: &str) -> Option<String> {
    // split contents into elements and keep key-value tuples only.
    let params: Vec<(&str, &str)> = contents
        .split('\n')
        .filter_map(|s| {
            let v: Vec<&str> = s.splitn(2, '=').collect();
            match v.len() {
                2 => Some((v[0], v[1])),
                _ => None,
            }
        })
        .collect();

    // find the OS release flag
    for (key, val) in params {
        if key != flagname {
            continue;
        }
        let bare_val = val.trim();
        if !bare_val.is_empty() {
            return Some(bare_val.to_string());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_flag_os_version() {
        let flagname = "VERSION";
        let tests = vec![
            ("", None),
            ("foo=bar", None),
            ("VERSION", None),
            ("VERSION=", None),
            ("VERSION=\t", None),
            ("VERSION=\"30.20190905.dev.2 (CoreOS preview)\"", Some("\"30.20190905.dev.2 (CoreOS preview)\"".to_string())),
            ("VERSION=\t\"30.20190905.dev.2 (CoreOS preview)\"", Some("\"30.20190905.dev.2 (CoreOS preview)\"".to_string())),
            ("VERSION=\"30.20190905.dev.2 (CoreOS preview)\"\n", Some("\"30.20190905.dev.2 (CoreOS preview)\"".to_string())),
            ("foo=bar\nVERSION=\"30.20190905.dev.2 (CoreOS preview)\"", Some("\"30.20190905.dev.2 (CoreOS preview)\"".to_string())),
            ("VERSION=\"30.20190905.dev.2 (CoreOS preview)\"\nfoo=bar", Some("\"30.20190905.dev.2 (CoreOS preview)\"".to_string())),
            ("foo=bar\nVERSION=\"30.20190905.dev.2 (CoreOS preview)\"\nfoo=bar", Some("\"30.20190905.dev.2 (CoreOS preview)\"".to_string())),
        ];
        for (tcase, tres) in tests {
            let res = find_flag_value(flagname, tcase);
            assert_eq!(res, tres, "failed testcase: '{}'", tcase);
        }
    }
}
