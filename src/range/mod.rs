use std::fmt;

use unit::RangeUnit;

pub(crate) mod bound;
pub(crate) mod comparator;
pub(crate) mod matcher;
pub(crate) mod unit;

#[derive(Clone, Debug, PartialEq)]
pub enum Range {
    Just(RangeUnit), // allocating for one unit only most of the time would be a waste
    All(Vec<RangeUnit>),
    Any(Vec<Vec<RangeUnit>>),
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Just(unit) => write!(f, "{}", unit),
            Self::All(units) => units
                .iter()
                .enumerate()
                .try_for_each(|(idx, unit)| match idx {
                    0 => write!(f, "{}", unit),
                    _ => write!(f, " {}", unit),
                }),
            Self::Any(unit_groups) => {
                unit_groups.iter().enumerate().try_for_each(|(idx, units)| {
                    let prefix = if idx != 0 { write!(f, " || ") } else { Ok(()) };
                    prefix.and_then(|_| {
                        units
                            .iter()
                            .enumerate()
                            .try_for_each(|(idx, unit)| match idx {
                                0 => write!(f, "{}", unit),
                                _ => write!(f, " {}", unit),
                            })
                    })
                })
            }
        }
    }
}

#[test]
fn test_to_string() {
    let parse = |s| RangeUnit::parse(s).expect(s).0;

    assert_eq!(">=1.2.3 <2.0.0-0", Range::Just(parse("^1.2.3")).to_string());
    assert_eq!(
        "1.2.3 4.5.6",
        Range::All(vec![parse("1.2.3"), parse("4.5.6")]).to_string()
    );
    assert_eq!(
        ">=1.2.3 <2.0.0-0 || 4.5.6",
        Range::Any(vec![vec![parse("^1.2.3")], vec![parse("4.5.6")],]).to_string()
    );
}

impl Range {
    /// Attempts to parse a `node-semver`-like version range into a representation that can be matched against.
    ///
    /// Note that it is significantly stricter than the reference algorithm used by that library,
    /// and will reject many ambiguous inputs.
    pub fn from(s: &str) -> Option<Self> {
        let mut r = s;
        let mut out = None::<Self>;

        while !r.is_empty() {
            let (unit, t) = RangeUnit::parse(r.trim_start_matches(' '))?;

            out = match out {
                None => Some(Self::Just(unit)),
                Some(Self::Just(prev_unit)) => Some(Self::All(vec![prev_unit, unit])),
                Some(Self::All(mut units)) => {
                    units.push(unit);
                    Some(Self::All(units))
                }
                Some(Self::Any(mut unit_groups)) => match unit_groups.last_mut() {
                    Some(units) => {
                        units.push(unit);
                        Some(Self::Any(unit_groups))
                    }
                    None => return None,
                },
            };

            r = t.trim_start_matches(' ');

            if let Some(t) = r.strip_prefix("||") {
                r = t;
                out = match out {
                    None => return None,
                    Some(Self::Just(unit)) => Some(Self::Any(vec![vec![unit], vec![]])),
                    Some(Self::All(units)) => Some(Self::Any(vec![units, vec![]])),
                    Some(Self::Any(mut unit_groups)) => {
                        unit_groups.push(vec![]);
                        Some(Self::Any(unit_groups))
                    }
                };
            }
        }

        match out {
            Some(Self::Any(unit_groups)) => match unit_groups.last() {
                Some(units) if units.is_empty() => None,
                _ => Some(Self::Any(unit_groups)),
            },
            rest => rest,
        }
    }
}

#[test]
fn test_from() {
    let parse = |s| Range::from(s).expect(s).to_string();

    assert_eq!(None, Range::from("  "));
    assert_eq!("1.2.3", parse("1.2.3"));
    assert_eq!(None, Range::from("1.2.3 ???"));
    assert_eq!("1.2.3", parse("   1.2.3    "));
    assert_eq!("1.2.3 <2.0.0", parse("1.2.3 <2.0.0"));
    assert_eq!("1.2.3 4.5.6 7.8.9", parse("1.2.3 4.5.6 7.8.9"));
    assert_eq!("1.2.3 || 4.5.6", parse("1.2.3 || 4.5.6"));
    assert_eq!(None, Range::from("1.2.3 ||"));
    assert_eq!("1.2.3 4.5.6 || 7.8.9", parse("1.2.3 4.5.6 || 7.8.9"));
    assert_eq!("1.2.3 || 4.5.6 || 7.8.9", parse("1.2.3 || 4.5.6 || 7.8.9"));
}
