use std::{cmp, fmt};

use super::common::parse_num_id;

#[derive(Clone, Debug)]
pub struct VersionPreRelease<'a>(pub &'a str);

impl fmt::Display for VersionPreRelease<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "-{}", self.0)
    }
}

#[test]
fn test_display() {
    assert_eq!("-foo", VersionPreRelease("foo").to_string());
}

impl PartialEq for VersionPreRelease<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for VersionPreRelease<'_> {}

impl PartialOrd for VersionPreRelease<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for VersionPreRelease<'_> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        use cmp::Ordering::*;

        let parts = self.0;
        let other_parts = other.0;

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
fn test_ord() {
    assert!(VersionPreRelease("alpha") < VersionPreRelease("alpha.1"));
    assert!(VersionPreRelease("alpha.1") < VersionPreRelease("alpha.beta"));
    assert!(VersionPreRelease("alpha.beta") < VersionPreRelease("beta"));
    assert!(VersionPreRelease("beta") < VersionPreRelease("beta.2"));
    assert!(VersionPreRelease("beta.2") < VersionPreRelease("beta.11"));
    assert!(VersionPreRelease("beta.11") < VersionPreRelease("rc.1"));
}

impl<'a> VersionPreRelease<'a> {
    pub fn parse(s: &'a str) -> Option<(Self, &'a str)> {
        let start = s.strip_prefix('-')?;
        let mut end = start;

        loop {
            let mut num_only = true;
            let cnt = end
                .chars()
                .take_while(|c| {
                    num_only = num_only && c.is_ascii_digit();
                    *c == '-' || c.is_ascii_alphanumeric()
                })
                .count();

            if cnt == 0 {
                return None;
            }

            if num_only && cnt > 1 && end.starts_with('0') {
                return None;
            }

            end = &end[cnt..];

            if let Some(new_end) = end.strip_prefix('.') {
                end = new_end;
            } else {
                break;
            }
        }

        if start != end {
            Some((Self(&start[..(start.len() - end.len())]), end))
        } else {
            None
        }
    }
}

#[test]
fn test_parse() {
    assert_eq!(None, VersionPreRelease::parse(""));
    assert_eq!(None, VersionPreRelease::parse("+foo"));
    assert_eq!(None, VersionPreRelease::parse("-"));
    assert_eq!(None, VersionPreRelease::parse("-.foo"));
    assert_eq!(None, VersionPreRelease::parse("-foo."));
    assert_eq!(None, VersionPreRelease::parse("-foo..bar"));
    assert_eq!(None, VersionPreRelease::parse("-foo.💩"));
    assert_eq!(None, VersionPreRelease::parse("+foo.01"));
    assert_eq!(
        Some((VersionPreRelease("foo"), "")),
        VersionPreRelease::parse("-foo")
    );
    assert_eq!(
        Some((VersionPreRelease("-aB0.102"), " 3.4")),
        VersionPreRelease::parse("--aB0.102 3.4")
    );
}
