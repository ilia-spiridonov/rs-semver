use std::{cmp, fmt};

use super::common::parse_num_id;

#[derive(Debug, Eq, PartialEq, PartialOrd)]
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
    // TODO
}
