use std::fmt;

pub use unit::RangeUnit;

mod comparator;
mod matcher;
mod unit;

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
                .map(|(idx, unit)| match idx {
                    0 => write!(f, "{}", unit),
                    _ => write!(f, " {}", unit),
                })
                .collect(),
            Self::Any(unit_groups) => unit_groups
                .iter()
                .enumerate()
                .map(|(idx, units)| {
                    let prefix = if idx != 0 { write!(f, " || ") } else { Ok(()) };
                    prefix.and_then(|_| {
                        units
                            .iter()
                            .enumerate()
                            .map(|(idx, unit)| match idx {
                                0 => write!(f, "{}", unit),
                                _ => write!(f, " {}", unit),
                            })
                            .collect()
                    })
                })
                .collect(),
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
    pub fn from(s: &'a str) -> Option<Self> {
        // TODO
        None
    }
}

#[test]
fn test_from() {
    // TODO
}
