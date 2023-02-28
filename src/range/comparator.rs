use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum RangeComparator {
    Less,
    LessOrEqual,
    Equal,
    GreaterOrEqual,
    Greater,
}

impl fmt::Display for RangeComparator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Less => write!(f, "<"),
            Self::LessOrEqual => write!(f, "<="),
            Self::Equal => write!(f, ""),
            Self::GreaterOrEqual => write!(f, ">="),
            Self::Greater => write!(f, ">"),
        }
    }
}

#[test]
fn test_to_string() {
    assert_eq!(">=", RangeComparator::GreaterOrEqual.to_string());
}

impl RangeComparator {
    pub(crate) fn parse(s: &str) -> Option<(Self, &str)> {
        let mut ch = s.chars();

        match (ch.next(), ch.next()) {
            (Some('<'), Some('=')) => Some((Self::LessOrEqual, &s[2..])),
            (Some('<'), _) => Some((Self::Less, &s[1..])),
            (Some('='), _) => Some((Self::Equal, &s[1..])),
            (Some('>'), Some('=')) => Some((Self::GreaterOrEqual, &s[2..])),
            (Some('>'), _) => Some((Self::Greater, &s[1..])),
            _ => None,
        }
    }
}

#[test]
fn test_parse() {
    use RangeComparator::*;

    assert_eq!(None, RangeComparator::parse("1"));
    assert_eq!(Some((Less, "1")), RangeComparator::parse("<1"));
    assert_eq!(Some((LessOrEqual, "1")), RangeComparator::parse("<=1"));
    assert_eq!(Some((Equal, "1")), RangeComparator::parse("=1"));
    assert_eq!(Some((GreaterOrEqual, "1")), RangeComparator::parse(">=1"));
    assert_eq!(Some((Greater, "1")), RangeComparator::parse(">1"));
}
