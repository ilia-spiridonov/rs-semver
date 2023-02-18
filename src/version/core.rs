use std::{cmp, fmt};

use super::common::parse_num_id;

#[derive(Clone, Debug, Hash)]
pub struct VersionCore {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl fmt::Display for VersionCore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl VersionCore {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

#[test]
fn test_to_string() {
    assert_eq!("1.2.3", VersionCore::new(1, 2, 3).to_string());
}

impl PartialEq for VersionCore {
    fn eq(&self, other: &Self) -> bool {
        self.major == other.major && self.minor == other.minor && self.patch == other.patch
    }
}

impl Eq for VersionCore {}

impl PartialOrd for VersionCore {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for VersionCore {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        use cmp::Ordering::*;

        for (part, other_part) in [
            (self.major, other.major),
            (self.minor, other.minor),
            (self.patch, other.patch),
        ] {
            if let ord @ (Less | Greater) = part.cmp(&other_part) {
                return ord;
            }
        }

        Equal
    }
}

#[test]
fn test_cmp() {
    assert!(VersionCore::new(0, 1, 2) < VersionCore::new(1, 0, 0));
    assert!(VersionCore::new(1, 1, 2) < VersionCore::new(1, 2, 3));
    assert!(VersionCore::new(1, 1, 2) < VersionCore::new(1, 1, 3));
}

impl VersionCore {
    pub(crate) fn parse(s: &str) -> Option<(Self, &str)> {
        let mut r = s;
        let mut parts = [0_u32; 3];

        for (idx, part) in parts.iter_mut().enumerate() {
            if idx != 0 {
                r = r.strip_prefix('.')?;
            }

            (*part, r) = parse_num_id(r)?;
        }

        Some((Self::new(parts[0], parts[1], parts[2]), r))
    }
}

#[test]
fn test_parse() {
    assert_eq!(None, VersionCore::parse("1"));
    assert_eq!(None, VersionCore::parse("1."));
    assert_eq!(None, VersionCore::parse("1.2"));
    assert_eq!(None, VersionCore::parse("1.2."));
    assert_eq!(None, VersionCore::parse(".1.2.3"));
    assert_eq!(None, VersionCore::parse("1..2"));
    assert_eq!(None, VersionCore::parse("1.2.03"));
    assert_eq!(None, VersionCore::parse("1.2.f"));
    assert_eq!(None, VersionCore::parse("1.2.-3"));
    assert_eq!(
        Some((VersionCore::new(1, 20, 3), ".")),
        VersionCore::parse("1.20.3.")
    );
}
