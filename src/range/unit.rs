use std::fmt;

use super::bound::RangeBound;
use super::comparator::RangeComparator;
use crate::version::difference::VersionDiff;
use crate::version::pattern::VersionPattern;
use crate::version::pre_release::VersionPreRelease;
use crate::version::Version;

enum ParsedComparator {
    Simple(RangeComparator),
    Caret,
    Tilde,
}

enum ParsedPart {
    Version(Version),
    Pattern(VersionPattern),
}

#[derive(Clone, Debug, PartialEq)]
pub struct RangeUnit {
    pub bound: RangeBound,
    pub extra_bound: Option<RangeBound>,
}

impl RangeUnit {
    pub(crate) fn new(bound: RangeBound, extra_bound: Option<RangeBound>) -> Self {
        Self { bound, extra_bound }
    }
}

impl fmt::Display for RangeUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.bound)?;

        if let Some(extra_bound) = &self.extra_bound {
            write!(f, " {}", extra_bound)
        } else {
            Ok(())
        }
    }
}

#[test]
fn test_to_string() {
    use RangeComparator::*;

    assert_eq!(
        ">=1.2.3",
        RangeUnit::new(RangeBound(GreaterOrEqual, Version::new(1, 2, 3)), None).to_string()
    );
    assert_eq!(
        "1.2.3 <4.5.6",
        RangeUnit::new(
            RangeBound(Equal, Version::new(1, 2, 3)),
            Some(RangeBound(Less, Version::new(4, 5, 6)))
        )
        .to_string()
    );
}

impl RangeUnit {
    pub(crate) fn parse(s: &str) -> Option<(Self, &str)> {
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

    fn from_part(comp: Option<ParsedComparator>, part: ParsedPart) -> Option<Self> {
        match part {
            ParsedPart::Version(ver) => Some(Self::from_version(comp, ver)),
            ParsedPart::Pattern(pat) => Self::from_pattern(comp, pat),
        }
    }

    fn from_version(comp: Option<ParsedComparator>, mut ver: Version) -> Self {
        use ParsedComparator::*;
        use RangeComparator::*;
        use VersionDiff::*;

        ver.build = None;

        match comp {
            None => Self::new(RangeBound(Equal, ver), None),
            Some(Simple(comp)) => Self::new(RangeBound(comp, ver), None),
            Some(Tilde) => {
                let upper_ver = ver.to_incremented(PreMinor);

                Self::new(
                    RangeBound(GreaterOrEqual, ver),
                    Some(RangeBound(Less, upper_ver)),
                )
            }
            Some(Caret) => {
                let diff = if ver.core.major == 0 && ver.core.minor == 0 {
                    PrePatch
                } else if ver.core.major == 0 {
                    PreMinor
                } else {
                    PreMajor
                };

                let upper_ver = ver.to_incremented(diff);

                Self::new(
                    RangeBound(GreaterOrEqual, ver),
                    Some(RangeBound(Less, upper_ver)),
                )
            }
        }
    }

    fn from_pattern(comp: Option<ParsedComparator>, pat: VersionPattern) -> Option<Self> {
        use ParsedComparator::*;
        use RangeComparator::*;

        match (comp, pat.to_bounds()) {
            (None, (lower, upper)) => Some(Self::new(
                RangeBound(GreaterOrEqual, lower),
                upper.map(|ver| {
                    RangeBound(Less, ver.with_pre_release(VersionPreRelease::default()))
                }),
            )),
            (Some(Simple(comp)), bounds) => match (comp, bounds) {
                (LessOrEqual | Equal | GreaterOrEqual, (lower, None)) => {
                    Some(Self::new(RangeBound(GreaterOrEqual, lower), None))
                }
                (comp @ (Greater | GreaterOrEqual), (lower, Some(upper))) => {
                    let bound = match comp {
                        Greater => upper,
                        _ => lower,
                    };

                    Some(Self::new(RangeBound(GreaterOrEqual, bound), None))
                }
                (Equal, (lower, Some(upper))) => Some(Self::new(
                    RangeBound(GreaterOrEqual, lower),
                    Some(RangeBound(
                        Less,
                        upper.with_pre_release(VersionPreRelease::default()),
                    )),
                )),
                (comp @ (Less | LessOrEqual), (lower, Some(upper))) => {
                    let bound = match comp {
                        Less => lower,
                        _ => upper,
                    };

                    Some(Self::new(
                        RangeBound(Less, bound.with_pre_release(VersionPreRelease::default())),
                        None,
                    ))
                }
                _ => None,
            },
            (Some(Tilde), (lower, Some(upper))) => Some(Self::new(
                RangeBound(GreaterOrEqual, lower),
                Some(RangeBound(
                    Less,
                    upper.with_pre_release(VersionPreRelease::default()),
                )),
            )),
            _ => None,
        }
    }

    fn merge_parts(first: ParsedPart, second: ParsedPart) -> Self {
        use ParsedPart::*;
        use RangeComparator::*;

        match (first, second) {
            (Version(a), Version(b)) => Self::new(
                RangeBound(GreaterOrEqual, a),
                Some(RangeBound(LessOrEqual, b)),
            ),
            (Version(ver), Pattern(pat)) => {
                let (_, upper) = pat.to_bounds();

                Self::new(
                    RangeBound(GreaterOrEqual, ver),
                    upper.map(|v| {
                        RangeBound(Less, v.with_pre_release(VersionPreRelease::default()))
                    }),
                )
            }
            (Pattern(pat), Version(ver)) => {
                let (lower, _) = pat.to_bounds();

                Self::new(
                    RangeBound(GreaterOrEqual, lower),
                    Some(RangeBound(LessOrEqual, ver)),
                )
            }
            (Pattern(a), Pattern(b)) => {
                let (lower, _) = a.to_bounds();
                let (_, upper) = b.to_bounds();

                Self::new(
                    RangeBound(GreaterOrEqual, lower),
                    upper.map(|v| {
                        RangeBound(Less, v.with_pre_release(VersionPreRelease::default()))
                    }),
                )
            }
        }
    }
}

#[test]
fn test_parse() {
    let parse = |s| RangeUnit::parse(s).expect(s).0.to_string();

    // version, no comparator
    assert_eq!("1.2.3-foo", parse("1.2.3-foo+bar"));
    // version, with comparator
    assert_eq!("<1.2.3", parse("<1.2.3"));
    assert_eq!("<1.2.3-foo", parse("<1.2.3-foo"));
    assert_eq!("<=1.2.3", parse("<=1.2.3"));
    assert_eq!("1.2.3", parse("=1.2.3"));
    assert_eq!(">=1.2.3", parse(">=1.2.3"));
    assert_eq!(">1.2.3", parse(">1.2.3"));
    assert_eq!(">=1.2.3 <1.3.0-0", parse("~1.2.3"));
    assert_eq!(">=1.2.3-foo <1.3.0-0", parse("~1.2.3-foo"));
    assert_eq!(">=1.2.3 <2.0.0-0", parse("^1.2.3"));
    assert_eq!(">=1.2.3-foo <2.0.0-0", parse("^1.2.3-foo"));
    assert_eq!(">=0.1.2 <0.2.0-0", parse("^0.1.2"));
    assert_eq!(">=0.0.1 <0.0.2-0", parse("^0.0.1"));
    // pattern, no comparator
    assert_eq!(">=0.0.0", parse("*"));
    assert_eq!(">=1.0.0 <2.0.0-0", parse("1"));
    assert_eq!(">=1.2.0 <1.3.0-0", parse("1.2"));
    // major pattern, with comparator
    assert_eq!(None, RangeUnit::parse(">*"));
    assert_eq!(">=0.0.0", parse(">=*"));
    assert_eq!(">=0.0.0", parse("=*"));
    assert_eq!(">=0.0.0", parse("<=*"));
    assert_eq!(None, RangeUnit::parse("<*"));
    assert_eq!(None, RangeUnit::parse("~*"));
    assert_eq!(None, RangeUnit::parse("^*"));
    // minor pattern, with comparator
    assert_eq!("<1.0.0-0", parse("<1"));
    assert_eq!("<2.0.0-0", parse("<=1"));
    assert_eq!(">=1.0.0 <2.0.0-0", parse("=1"));
    assert_eq!(">=1.0.0", parse(">=1"));
    assert_eq!(">=2.0.0", parse(">1"));
    assert_eq!(">=1.0.0 <2.0.0-0", parse("~1"));
    assert_eq!(None, RangeUnit::parse("^1"));
    // patch pattern, with comparator
    assert_eq!("<1.2.0-0", parse("<1.2"));
    assert_eq!("<1.3.0-0", parse("<=1.2"));
    assert_eq!(">=1.2.0 <1.3.0-0", parse("=1.2"));
    assert_eq!(">=1.2.0", parse(">=1.2"));
    assert_eq!(">=1.3.0", parse(">1.2"));
    assert_eq!(">=1.2.0 <1.3.0-0", parse("~1.2"));
    assert_eq!(None, RangeUnit::parse("^1.2"));
    // hypen range
    assert_eq!(">=1.2.3 <=4.5.6", parse("1.2.3 - 4.5.6"));
    assert_eq!(">=1.2.3 <4.6.0-0", parse("1.2.3 - 4.5"));
    assert_eq!(">=1.2.3 <5.0.0-0", parse("1.2.3 - 4"));
    assert_eq!(">=1.2.3", parse("1.2.3 - *"));
    assert_eq!(">=1.2.0 <=3.4.5", parse("1.2 - 3.4.5"));
    assert_eq!(">=1.2.0 <3.5.0-0", parse("1.2 - 3.4"));
    assert_eq!(">=1.0.0 <=3.4.5", parse("1 - 3.4.5"));
    assert_eq!(">=0.0.0 <=3.4.5", parse("* - 3.4.5"));
    assert_eq!(None, RangeUnit::parse(">1 - 2"))
}
