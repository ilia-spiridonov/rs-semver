use std::fmt;

use super::common::parse_dot_sep_list;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct VersionBuild(pub String);

impl fmt::Display for VersionBuild {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "+{}", self.0)
    }
}

#[test]
fn test_to_string() {
    assert_eq!("+foo", VersionBuild("foo".to_string()).to_string());
}

impl VersionBuild {
    pub(crate) fn parse(s: &str) -> Option<(Option<Self>, &str)> {
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
        Some((Some(VersionBuild("foo".to_string())), "")),
        VersionBuild::parse("+foo")
    );
}
