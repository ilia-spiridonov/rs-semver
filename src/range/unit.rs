use std::fmt;

use super::RangeComparator;
use crate::{Version, VersionIncrement, VersionPattern};

type Bound<'a> = (RangeComparator, Version<'a>);

enum ParsedComparator {
    Simple(RangeComparator),
    Caret,
    Tilde,
}

enum ParsedPart<'a> {
    Version(Version<'a>),
    Pattern(VersionPattern),
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
        let (comp, r) = Self::parse_comparator(s);
        let (part, r) = Self::parse_part(r)?;

        if let Some(r) = Self::parse_hyphen(r) {
            let (second_part, r) = Self::parse_part(r)?;

            if comp.is_some() {
                return None;
            }

            return Some((Self::merge_parts(part, second_part), r));
        }

        Self::from_part(comp, part).map(|u| (u, r))
    }

    fn parse_comparator(s: &str) -> (Option<ParsedComparator>, &str) {
        use ParsedComparator::*;

        if let Some((comp, r)) = RangeComparator::parse(s) {
            (Some(Simple(comp)), r)
        } else if let Some(r) = s.strip_prefix('^') {
            (Some(Caret), r)
        } else if let Some(r) = s.strip_prefix('~') {
            (Some(Tilde), r)
        } else {
            (None, s)
        }
    }

    fn parse_hyphen(s: &str) -> Option<&str> {
        Some(
            s.strip_prefix(' ')?
                .trim_start_matches(' ')
                .strip_prefix('-')?
                .trim_start_matches(' '),
        )
    }

    fn parse_part(s: &str) -> Option<(ParsedPart, &str)> {
        if let Some((ver, r)) = Version::parse(s) {
            Some((ParsedPart::Version(ver), r))
        } else if let Some((pat, r)) = VersionPattern::parse(s) {
            Some((ParsedPart::Pattern(pat), r))
        } else {
            None
        }
    }

    fn from_part(comp: Option<ParsedComparator>, part: ParsedPart<'a>) -> Option<Self> {
        match part {
            ParsedPart::Version(ver) => Some(Self::from_version(comp, ver)),
            ParsedPart::Pattern(pat) => Self::from_pattern(comp, pat),
        }
    }

    fn from_version(comp: Option<ParsedComparator>, ver: Version<'a>) -> Self {
        use ParsedComparator::*;
        use RangeComparator::*;
        use VersionIncrement::*;

        match comp {
            None => Self::new((Equal, ver), None),
            Some(Simple(comp)) => Self::new((comp, ver), None),
            Some(Tilde) => {
                let mut upper = ver.clone();
                upper.increment(Minor, false);

                Self::new((GreaterOrEqual, ver), Some((Less, upper)))
            }
            Some(Caret) => {
                let inc = match (ver.core.major, ver.core.minor, ver.core.patch) {
                    (0, 0, _) => Patch,
                    (0, _, _) => Minor,
                    (_, _, _) => Major,
                };

                let mut upper = ver.clone();
                upper.increment(inc, false);

                Self::new((GreaterOrEqual, ver), Some((Less, upper)))
            }
        }
    }

    fn from_pattern(comp: Option<ParsedComparator>, pat: VersionPattern) -> Option<Self> {
        use ParsedComparator::*;
        use RangeComparator::*;

        match (comp, pat.to_bounds()) {
            (None, (lower, upper)) => Some(Self::new(
                (GreaterOrEqual, lower),
                upper.map(|ver| (Less, ver)),
            )),
            (Some(Simple(comp)), bounds) => match (comp, bounds) {
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
            (Some(Tilde), (lower, Some(upper))) => {
                Some(Self::new((GreaterOrEqual, lower), Some((Less, upper))))
            }
            _ => None,
        }
    }

    fn merge_parts(first: ParsedPart<'a>, second: ParsedPart<'a>) -> Self {
        use ParsedPart::*;
        use RangeComparator::*;

        match (first, second) {
            (Version(a), Version(b)) => Self::new((GreaterOrEqual, a), Some((LessOrEqual, b))),
            (Version(ver), Pattern(pat)) => {
                let (_, upper) = pat.to_bounds();
                Self::new((GreaterOrEqual, ver), upper.map(|v| (Less, v)))
            }
            (Pattern(pat), Version(ver)) => {
                let (lower, _) = pat.to_bounds();
                Self::new((GreaterOrEqual, lower), Some((LessOrEqual, ver)))
            }
            (Pattern(a), Pattern(b)) => {
                let (lower, _) = a.to_bounds();
                let (_, upper) = b.to_bounds();
                Self::new((GreaterOrEqual, lower), upper.map(|v| (Less, v)))
            }
        }
    }
}

#[test]
fn test_parse() {
    let parse = |s: &'static str| RangeUnit::parse(s).expect(s).0.to_string();

    // version, no comparator
    assert_eq!("1.2.3-foo+bar", parse("1.2.3-foo+bar"));
    // version, with comparator
    assert_eq!("<1.2.3", parse("<1.2.3"));
    assert_eq!("<=1.2.3", parse("<=1.2.3"));
    assert_eq!("1.2.3", parse("=1.2.3"));
    assert_eq!(">=1.2.3", parse(">=1.2.3"));
    assert_eq!(">1.2.3", parse(">1.2.3"));
    assert_eq!(">=1.2.3 <1.3.0", parse("~1.2.3"));
    assert_eq!(">=1.2.3 <2.0.0", parse("^1.2.3"));
    assert_eq!(">=0.1.2 <0.2.0", parse("^0.1.2"));
    assert_eq!(">=0.0.1 <0.0.2", parse("^0.0.1"));
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
    assert_eq!(None, RangeUnit::parse("~*"));
    assert_eq!(None, RangeUnit::parse("^*"));
    // minor pattern, with comparator
    assert_eq!("<1.0.0", parse("<1"));
    assert_eq!("<2.0.0", parse("<=1"));
    assert_eq!(">=1.0.0 <2.0.0", parse("=1"));
    assert_eq!(">=1.0.0", parse(">=1"));
    assert_eq!(">=2.0.0", parse(">1"));
    assert_eq!(">=1.0.0 <2.0.0", parse("~1"));
    assert_eq!(None, RangeUnit::parse("^1"));
    // patch pattern, with comparator
    assert_eq!("<1.2.0", parse("<1.2"));
    assert_eq!("<1.3.0", parse("<=1.2"));
    assert_eq!(">=1.2.0 <1.3.0", parse("=1.2"));
    assert_eq!(">=1.2.0", parse(">=1.2"));
    assert_eq!(">=1.3.0", parse(">1.2"));
    assert_eq!(">=1.2.0 <1.3.0", parse("~1.2"));
    assert_eq!(None, RangeUnit::parse("^1.2"));
    // hypen range
    assert_eq!(">=1.2.3 <=4.5.6", parse("1.2.3 - 4.5.6"));
    assert_eq!(">=1.2.3 <4.6.0", parse("1.2.3 - 4.5"));
    assert_eq!(">=1.2.3 <5.0.0", parse("1.2.3 - 4"));
    assert_eq!(">=1.2.3", parse("1.2.3 - *"));
    assert_eq!(">=1.2.0 <=3.4.5", parse("1.2 - 3.4.5"));
    assert_eq!(">=1.2.0 <3.5.0", parse("1.2 - 3.4"));
    assert_eq!(">=1.0.0 <=3.4.5", parse("1 - 3.4.5"));
    assert_eq!(">=0.0.0 <=3.4.5", parse("* - 3.4.5"));
    assert_eq!(None, RangeUnit::parse(">1 - 2"))
}
