use std::fmt;

use super::RangeComparator;
use crate::{Version, VersionPattern};

type Bound<'a> = (RangeComparator, Version<'a>);

enum ParsedComparator {
    Simple(RangeComparator),
    Caret,
    Tilde,
}

#[derive(Debug, PartialEq)]
pub struct RangeUnit<'a> {
    pub bound: Bound<'a>,
    pub extra_bound: Option<Bound<'a>>,
}

impl<'a> RangeUnit<'a> {
    pub fn new(bound: Bound<'a>, extra_bound: Option<Bound<'a>>) -> Self {
        Self { bound, extra_bound }
    }
}

impl fmt::Display for RangeUnit<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (comp, ver) = &self.bound;

        if let Some((ex_comp, ex_ver)) = &self.extra_bound {
            write!(f, "{}{} {}{}", comp, ver, ex_comp, ex_ver)
        } else {
            write!(f, "{}{}", comp, ver)
        }
    }
}

#[test]
fn test_to_string() {
    use crate::VersionCore;

    let ver1 = Version::new(VersionCore::new(1, 2, 3), None, None);
    let ver2 = Version::new(VersionCore::new(4, 5, 6), None, None);

    assert_eq!(
        ">=1.2.3",
        RangeUnit::new((RangeComparator::GreaterOrEqual, ver1.clone()), None).to_string()
    );
    assert_eq!(
        "1.2.3 <4.5.6",
        RangeUnit::new(
            (RangeComparator::Equal, ver1),
            Some((RangeComparator::Less, ver2))
        )
        .to_string()
    );
}

impl<'a> RangeUnit<'a> {
    pub(crate) fn parse(s: &'a str) -> Option<(Self, &'a str)> {
        let mut r = s;

        let comp = if let Some((comp, t)) = RangeComparator::parse(r) {
            r = t;
            Some(ParsedComparator::Simple(comp))
        } else if let Some(t) = r.strip_prefix('^') {
            r = t;
            Some(ParsedComparator::Caret)
        } else if let Some(t) = r.strip_prefix('~') {
            r = t;
            Some(ParsedComparator::Tilde)
        } else {
            None
        };

        if let Some((ver, t)) = Version::parse(r) {
            // TODO
            return None;
        }

        if let Some((pat, t)) = VersionPattern::parse(r) {
            use RangeComparator::*;

            let unit = match (comp, pat.to_bounds()) {
                (None, (lower, upper)) => Some(Self::new(
                    (GreaterOrEqual, lower),
                    upper.map(|ver| (Less, ver)),
                )),
                (Some(ParsedComparator::Simple(comp)), bounds) => match (comp, bounds) {
                    (LessOrEqual | Equal | GreaterOrEqual, (lower, None)) => {
                        Some(Self::new((GreaterOrEqual, lower), None))
                    }
                    (comp @ (Greater | GreaterOrEqual), (lower, Some(upper))) => {
                        let bound = match comp {
                            Greater => upper,
                            _ => lower,
                        };
                        Some(Self::new((GreaterOrEqual, bound), None))
                    }
                    (Equal, (lower, Some(upper))) => {
                        Some(Self::new((GreaterOrEqual, lower), Some((Less, upper))))
                    }
                    (comp @ (Less | LessOrEqual), (lower, Some(upper))) => {
                        let bound = match comp {
                            Less => lower,
                            _ => upper,
                        };
                        Some(Self::new((Less, bound), None))
                    }
                    _ => None,
                },
                _ => None,
            };

            return unit.map(|u| (u, t));
        }

        None
    }
}

#[test]
fn test_parse() {
    let parse = |s: &'static str| RangeUnit::parse(s).expect(s).0.to_string();

    // pattern, no comparator
    assert_eq!(">=0.0.0", parse("*"));
    assert_eq!(">=1.0.0 <2.0.0", parse("1"));
    assert_eq!(">=1.2.0 <1.3.0", parse("1.2"));
    // major pattern, with comparator
    assert_eq!(None, RangeUnit::parse(">*"));
    assert_eq!(">=0.0.0", parse(">=*"));
    assert_eq!(">=0.0.0", parse("=*"));
    assert_eq!(">=0.0.0", parse("<=*"));
    assert_eq!(None, RangeUnit::parse("<*"));
    // minor pattern, with comparator
    assert_eq!("<1.0.0", parse("<1"));
    assert_eq!("<2.0.0", parse("<=1"));
    assert_eq!(">=1.0.0 <2.0.0", parse("=1"));
    assert_eq!(">=1.0.0", parse(">=1"));
    assert_eq!(">=2.0.0", parse(">1"));
    // patch pattern, with comparator
    assert_eq!("<1.2.0", parse("<1.2"));
    assert_eq!("<1.3.0", parse("<=1.2"));
    assert_eq!(">=1.2.0 <1.3.0", parse("=1.2"));
    assert_eq!(">=1.2.0", parse(">=1.2"));
    assert_eq!(">=1.3.0", parse(">1.2"));
}
