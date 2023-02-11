use std::fmt;

use crate::common::parse_num_id;

#[derive(Debug, PartialEq)]
pub struct Pattern {
    major: u32,
    minor: Option<u32>,
}

impl Pattern {
    fn new(major: u32, minor: Option<u32>) -> Self {
        Self { major, minor }
    }
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(minor) = self.minor {
            write!(f, "{}.{}.*", self.major, minor)
        } else {
            write!(f, "{}.*.*", self.major)
        }
    }
}

#[test]
fn test_to_string() {
    assert_eq!("1.*.*", Pattern::new(1, None).to_string());
    assert_eq!("1.2.*", Pattern::new(1, Some(2)).to_string());
}

impl Pattern {
    pub(crate) fn parse(s: &str) -> Option<(Self, &str)> {
        let mut parts = [None::<u32>; 3];
        let mut r = s;

        for (idx, part) in parts.iter_mut().enumerate() {
            if idx != 0 {
                if let Some(t) = r.strip_prefix('.') {
                    r = t;
                } else {
                    break;
                }
            }

            if let Some((p, t)) = parse_num_id(r) {
                *part = Some(p);
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
            [Some(major), minor, None] => Some((Self::new(major, minor), r)),
            _ => None,
        }
    }
}

#[test]
fn test_parse() {
    assert_eq!(None, Pattern::parse(""));
    assert_eq!(None, Pattern::parse("*"));
    assert_eq!(None, Pattern::parse("01"));
    assert_eq!(Some((Pattern::new(1, None), "")), Pattern::parse("1"));
    assert_eq!(None, Pattern::parse("1."));
    assert_eq!(None, Pattern::parse("1.?"));
    assert_eq!(Some((Pattern::new(1, None), "")), Pattern::parse("1.x"));
    assert_eq!(Some((Pattern::new(1, None), "")), Pattern::parse("1.X"));
    assert_eq!(Some((Pattern::new(1, None), "")), Pattern::parse("1.*"));
    assert_eq!(Some((Pattern::new(1, None), "*")), Pattern::parse("1.**"));
    assert_eq!(Some((Pattern::new(1, None), ".")), Pattern::parse("1.*.*."));
    assert_eq!(Some((Pattern::new(1, Some(2)), "")), Pattern::parse("1.2"));
    assert_eq!(
        Some((Pattern::new(1, Some(2)), "")),
        Pattern::parse("1.2.*")
    );
    assert_eq!(None, Pattern::parse("1.2."));
    assert_eq!(
        Some((Pattern::new(1, Some(2)), "")),
        Pattern::parse("1.2.*")
    );
    assert_eq!(None, Pattern::parse("1.2.3"));
}
