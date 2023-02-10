use std::{cmp, fmt};

#[derive(Debug, PartialEq, Eq)]
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
    #[allow(dead_code)]
    fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

#[test]
fn test_display() {
    assert_eq!("1.2.3", VersionCore::new(1, 2, 3).to_string());
}

impl PartialOrd for VersionCore {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        use cmp::Ordering::*;

        for (part, other_part) in [
            (self.major, other.major),
            (self.minor, other.minor),
            (self.patch, other.patch),
        ] {
            match part.cmp(&other_part) {
                ord @ (Less | Greater) => return Some(ord),
                Equal => continue,
            };
        }

        Some(Equal)
    }
}

#[test]
fn test_ord() {
    assert!(VersionCore::new(1, 2, 3) == VersionCore::new(1, 2, 3));
    assert!(VersionCore::new(0, 1, 2) < VersionCore::new(1, 0, 0));
    assert!(VersionCore::new(1, 1, 2) < VersionCore::new(1, 2, 3));
    assert!(VersionCore::new(1, 1, 2) < VersionCore::new(1, 1, 3));
}

impl VersionCore {
    pub fn parse(s: &str) -> Option<(Self, &str)> {
        // TODO
        None
    }
}

#[test]
fn test_parse() {
    // TODO
}
