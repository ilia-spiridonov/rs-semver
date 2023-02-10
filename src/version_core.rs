use std::{cmp, fmt};

#[derive(Clone, Debug, PartialEq)]
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
        let mut r = s;
        let mut parts = [0_u32; 3];

        for idx in 0..3 {
            if idx != 0 {
                r = r.strip_prefix('.')?;
            }

            let cnt = r.chars().take_while(|c| c.is_ascii_digit()).count();
            if cnt == 0 || (cnt > 1 && r.starts_with('0')) {
                return None;
            }

            parts[idx] = r[..cnt].parse().ok()?;
            r = &r[cnt..];
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
