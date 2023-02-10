use std::{cmp, fmt};

use super::common::parse_num_id;

#[derive(Debug, PartialEq)]
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

impl PartialOrd for VersionPreRelease<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
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
                return Some(ord);
            }
        }

        parts
            .split('.')
            .count()
            .partial_cmp(&other_parts.split('.').count())
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
