use std::{cmp, fmt};

use super::common::{parse_dot_sep_list, parse_num_id};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct VersionPreRelease(pub String);

impl fmt::Display for VersionPreRelease {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "-{}", self.0)
    }
}

#[test]
fn test_to_string() {
    assert_eq!("-foo", VersionPreRelease("foo".to_string()).to_string());
}

impl PartialOrd for VersionPreRelease {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for VersionPreRelease {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        use cmp::Ordering::*;

        let parts = &self.0;
        let other_parts = &other.0;

        for (part, other_part) in parts.split('.').zip(other_parts.split('.')) {
            let ord = match (parse_num_id(part), parse_num_id(other_part)) {
                (Some((_, "")), None) => Less,
                (None, Some((_, ""))) => Greater,
                (Some((id, "")), Some((other_id, ""))) => id.cmp(&other_id),
                _ => part.cmp(other_part),
            };

            if let Less | Greater = ord {
                return ord;
            }
        }

        parts
            .split('.')
            .count()
            .cmp(&other_parts.split('.').count())
    }
}

#[test]
fn test_cmp() {
    assert!(VersionPreRelease("alpha".to_string()) < VersionPreRelease("alpha.1".to_string()));
    assert!(VersionPreRelease("alpha.1".to_string()) < VersionPreRelease("alpha.beta".to_string()));
    assert!(VersionPreRelease("alpha.beta".to_string()) < VersionPreRelease("beta".to_string()));
    assert!(VersionPreRelease("beta".to_string()) < VersionPreRelease("beta.2".to_string()));
    assert!(VersionPreRelease("beta.2".to_string()) < VersionPreRelease("beta.11".to_string()));
    assert!(VersionPreRelease("beta.11".to_string()) < VersionPreRelease("rc.1".to_string()));
}

impl VersionPreRelease {
    pub(crate) fn parse(s: &str) -> Option<(Option<Self>, &str)> {
        if let Some(r) = s.strip_prefix('-') {
            parse_dot_sep_list(r, |p| {
                p.chars().any(|c| !c.is_ascii_digit()) || p.len() == 1 || !p.starts_with('0')
            })
            .map(|(s, r)| (Some(Self(s)), r))
        } else {
            Some((None, s))
        }
    }
}

#[test]
fn test_parse() {
    assert_eq!(Some((None, "foo")), VersionPreRelease::parse("foo"));
    assert_eq!(Some((None, "+foo")), VersionPreRelease::parse("+foo"));
    assert_eq!(None, VersionPreRelease::parse("-foo.01"));
    assert_eq!(
        Some((Some(VersionPreRelease("foo".to_string())), "")),
        VersionPreRelease::parse("-foo")
    );
}
