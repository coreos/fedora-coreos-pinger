//! Kernel cmdline parsing - utility functions
//!
//! NOTE(lucab): this is not a complete/correct cmdline parser, as it implements
//!  just enough logic to extract the platform ID value. In particular, it does not
//!  handle separator quoting/escaping, list of values, and merging of repeated
//!  flags. Logic is taken from Afterburn, please backport any bugfix there too:
//!  https://github.com/coreos/afterburn/blob/v4.1.0/src/util/cmdline.rs

use failure::Fallible;
use crate::util;

/// Platform key.
#[cfg(not(feature = "cl-legacy"))]
const CMDLINE_PLATFORM_FLAG: &str = "ignition.platform.id";
/// Platform key (CL and RHCOS legacy name: "OEM").
#[cfg(feature = "cl-legacy")]
const CMDLINE_PLATFORM_FLAG: &str = "coreos.oem.id";

/// Read platform value from cmdline file.
pub(crate) fn get_platform<T>(cmdline_path: T) -> Fallible<String>
where
    T: AsRef<str>,
{
    let fpath = cmdline_path.as_ref();
    util::get_value_by_flag(CMDLINE_PLATFORM_FLAG, fpath)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_flag_platform() {
        let flagname = "ignition.platform.id";
        let tests = vec![
            ("", None),
            ("foo=bar", None),
            ("ignition.platform.id", None),
            ("ignition.platform.id=", None),
            ("ignition.platform.id=\t", None),
            ("ignition.platform.id=ec2", Some("ec2".to_string())),
            ("ignition.platform.id=\tec2", Some("ec2".to_string())),
            ("ignition.platform.id=ec2\n", Some("ec2".to_string())),
            ("foo=bar ignition.platform.id=ec2", Some("ec2".to_string())),
            ("ignition.platform.id=ec2 foo=bar", Some("ec2".to_string())),
        ];
        for (tcase, tres) in tests {
            let res = util::find_flag_value(flagname, tcase);
            assert_eq!(res, tres, "failed testcase: '{}'", tcase);
        }
    }
}
