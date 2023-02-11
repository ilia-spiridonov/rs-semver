use std::{cmp, fmt};

use super::common::parse_dot_sep_list;

#[derive(Clone, Debug)]
pub struct VersionBuild<'a>(pub &'a str);

impl fmt::Display for VersionBuild<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "+{}", self.0)
    }
}

#[test]
fn test_to_string() {
    assert_eq!("+foo", VersionBuild("foo").to_string());
}

impl PartialEq for VersionBuild<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for VersionBuild<'_> {}

impl PartialOrd for VersionBuild<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for VersionBuild<'_> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.0.cmp(other.0)
    }
}

impl<'a> VersionBuild<'a> {
    pub fn parse(s: &'a str) -> Option<(Option<Self>, &'a str)> {
        if let Some(r) = s.strip_prefix('+') {
            parse_dot_sep_list(r, |_| true).map(|(s, r)| (Some(Self(s)), r))
        } else {
            Some((None, s))
        }
    }
}

#[test]
fn test_parse() {
    assert_eq!(Some((None, "foo")), VersionBuild::parse("foo"));
    assert_eq!(Some((None, "-foo")), VersionBuild::parse("-foo"));
    assert_eq!(None, VersionBuild::parse("+"));
    assert_eq!(
        Some((Some(VersionBuild("foo")), "")),
        VersionBuild::parse("+foo")
    );
}
