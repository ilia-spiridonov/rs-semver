use std::fmt;

use super::common::parse_num_id;
use super::Version;

#[derive(Debug, PartialEq)]
pub(crate) enum VersionPattern {
    Major,
    Minor(u32),
    Patch(u32, u32),
}

impl fmt::Display for VersionPattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Major => write!(f, "*"),
            Self::Minor(major) => write!(f, "{}.*", major),
            Self::Patch(major, minor) => write!(f, "{}.{}.*", major, minor),
        }
    }
}

#[test]
fn test_to_string() {
    use VersionPattern::*;

    assert_eq!("*", Major.to_string());
    assert_eq!("1.*", Minor(1).to_string());
    assert_eq!("1.2.*", Patch(1, 2).to_string());
}

impl VersionPattern {
    pub(crate) fn parse(s: &str) -> Option<(Self, &str)> {
        let mut parts = [None::<u32>; 3];
        let mut r = s;

        for idx in 0..parts.len() {
            if idx != 0 {
                if let Some(t) = r.strip_prefix('.') {
                    r = t;
                } else {
                    break;
                }
            }

            if let Some((p, t)) = parse_num_id(r) {
                if idx != 0 && parts[idx - 1].is_none() {
                    return None;
                }

                parts[idx] = Some(p);
                r = t;
                continue;
            }

            if let Some(t) = r.strip_prefix(['*', 'x', 'X']) {
                r = t;
                continue;
            }

            return None;
        }

        match parts {
            [Some(major), Some(minor), None] => Some((Self::Patch(major, minor), r)),
            [Some(major), None, None] => Some((Self::Minor(major), r)),
            [None, None, None] => Some((Self::Major, r)),
            _ => None,
        }
    }
}

#[test]
fn test_parse() {
    use VersionPattern::*;

    assert_eq!(None, VersionPattern::parse(""));
    assert_eq!(Some((Major, "")), VersionPattern::parse("*"));
    assert_eq!(Some((Major, "")), VersionPattern::parse("x"));
    assert_eq!(Some((Major, "")), VersionPattern::parse("X"));
    assert_eq!(None, VersionPattern::parse("01"));
    assert_eq!(Some((Minor(1), "")), VersionPattern::parse("1"));
    assert_eq!(None, VersionPattern::parse("1."));
    assert_eq!(None, VersionPattern::parse("1.?"));
    assert_eq!(Some((Minor(1), "")), VersionPattern::parse("1.*"));
    assert_eq!(Some((Minor(1), "*")), VersionPattern::parse("1.**"));
    assert_eq!(Some((Minor(1), ".")), VersionPattern::parse("1.*.*."));
    assert_eq!(Some((Patch(1, 2), "")), VersionPattern::parse("1.2"));
    assert_eq!(Some((Patch(1, 2), "")), VersionPattern::parse("1.2.*"));
    assert_eq!(None, VersionPattern::parse("1.2."));
    assert_eq!(Some((Patch(1, 2), "")), VersionPattern::parse("1.2.*"));
    assert_eq!(None, VersionPattern::parse("1.2.3"));
}

impl VersionPattern {
    pub(crate) fn to_bounds(&self) -> (Version, Option<Version>) {
        match self {
            Self::Major => (Version::new(0, 0, 0), None),
            Self::Minor(major) => (
                Version::new(*major, 0, 0),
                Some(Version::new(major + 1, 0, 0)),
            ),
            Self::Patch(major, minor) => (
                Version::new(*major, *minor, 0),
                Some(Version::new(*major, minor + 1, 0)),
            ),
        }
    }
}

#[test]
fn test_to_bounds() {
    use VersionPattern::*;

    assert_eq!((Version::new(0, 0, 0), None), Major.to_bounds());
    assert_eq!(
        (Version::new(1, 0, 0), Some(Version::new(2, 0, 0))),
        Minor(1).to_bounds()
    );
    assert_eq!(
        (Version::new(1, 2, 0), Some(Version::new(1, 3, 0))),
        Patch(1, 2).to_bounds()
    );
}
