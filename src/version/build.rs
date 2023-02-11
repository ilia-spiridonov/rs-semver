use std::fmt;

#[derive(Clone, Debug)]
pub struct VersionBuild<'a>(pub &'a str);

impl fmt::Display for VersionBuild<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "+{}", self.0)
    }
}

#[test]
fn test_display() {
    assert_eq!("+foo", VersionBuild("foo").to_string());
}

impl PartialEq for VersionBuild<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<'a> VersionBuild<'a> {
    pub fn parse(s: &'a str) -> Option<(Self, &'a str)> {
        let start = s.strip_prefix('+')?;
        let mut end = start;

        loop {
            let cnt = end
                .chars()
                .take_while(|c| *c == '-' || c.is_ascii_alphanumeric())
                .count();

            if cnt == 0 {
                return None;
            }

            end = &end[cnt..];

            if !end.starts_with('.') {
                break;
            }

            end = end.strip_prefix('.')?;
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
    assert_eq!(None, VersionBuild::parse("foo"));
    assert_eq!(None, VersionBuild::parse("-foo"));
    assert_eq!(None, VersionBuild::parse("+"));
    assert_eq!(None, VersionBuild::parse("+.foo"));
    assert_eq!(None, VersionBuild::parse("+foo."));
    assert_eq!(None, VersionBuild::parse("+foo..bar"));
    assert_eq!(None, VersionBuild::parse("+foo.💩"));
    assert_eq!(Some((VersionBuild("foo"), "")), VersionBuild::parse("+foo"));
    assert_eq!(
        Some((VersionBuild("-Ab1"), "_")),
        VersionBuild::parse("+-Ab1_")
    );
    assert_eq!(
        Some((VersionBuild("1.2"), " 3.4.5")),
        VersionBuild::parse("+1.2 3.4.5")
    );
}
