use std::fmt;

pub use comparator::RangeComparator;
pub use unit::RangeUnit;

mod comparator;
mod matcher;
mod unit;

#[derive(Debug, PartialEq)]
pub enum Range<'a> {
    Just(RangeUnit<'a>), // allocating for one unit only most of the time would be a waste
    All(Vec<RangeUnit<'a>>),
    Any(Vec<Vec<RangeUnit<'a>>>),
}

impl fmt::Display for Range<'_> {
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
    assert_eq!(
        ">=1.2.3 <2.0.0",
        Range::Just(RangeUnit::parse("^1.2.3").unwrap().0).to_string()
    );
    assert_eq!(
        "1.2.3 4.5.6",
        Range::All(vec![
            RangeUnit::parse("1.2.3").unwrap().0,
            RangeUnit::parse("4.5.6").unwrap().0
        ])
        .to_string()
    );
    assert_eq!(
        ">=1.2.3 <2.0.0 || 4.5.6",
        Range::Any(vec![
            vec![RangeUnit::parse("^1.2.3").unwrap().0],
            vec![RangeUnit::parse("4.5.6").unwrap().0],
        ])
        .to_string()
    );
}

impl<'a> Range<'a> {
    /// Attempts to parse a `node-semver`-like version range into a representation that can be matched against.
    ///
    /// Note that it is significantly stricter than the reference algorithm used by that library,
    /// and will reject many ambiguous inputs.
    ///
    /// Also note that it will not add the `0` pre-release tag to versions with certain comparators,
    /// since it's unnecessary due to the way the matching algorithm is implemented.
    pub fn from(s: &'a str) -> Option<Self> {
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
