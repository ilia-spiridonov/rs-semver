use std::cmp;

use super::super::Version;
use super::comparator::RangeComparator;
use super::unit::{RangeBound, RangeUnit};

impl RangeUnit<'_> {
    pub(crate) fn matches(&self, ver: &Version) -> bool {
        if let Some(extra_bound) = &self.extra_bound {
            Self::matches_bound(&self.bound, ver) && Self::matches_bound(extra_bound, ver)
        } else {
            Self::matches_bound(&self.bound, ver)
        }
    }

    fn matches_bound((comp, bound_ver): &RangeBound, ver: &Version) -> bool {
        use RangeComparator::*;

        if ver.pre_release.is_some() {
            if bound_ver.pre_release.is_none() || bound_ver.core != ver.core {
                return false;
            }
        }

        match (ver.cmp(bound_ver), comp) {
            (cmp::Ordering::Less, Less | LessOrEqual) => true,
            (cmp::Ordering::Equal, LessOrEqual | Equal | GreaterOrEqual) => true,
            (cmp::Ordering::Greater, GreaterOrEqual | Greater) => true,
            _ => false,
        }
    }
}

#[test]
fn test_matches() {
    let matches = |v: &str, r: &str| {
        RangeUnit::parse(r)
            .unwrap()
            .0
            .matches(&Version::from(v).unwrap())
    };

    assert!(matches("1.2.4", ">1.2.3"));
    assert!(!matches("1.2.3", ">1.2.3"));
    assert!(!matches("1.2.2", ">1.2.3"));

    assert!(matches("1.2.4", ">=1.2.3"));
    assert!(matches("1.2.3", ">=1.2.3"));
    assert!(!matches("1.2.2", ">=1.2.3"));

    assert!(!matches("1.2.4", "=1.2.3"));
    assert!(matches("1.2.3", "=1.2.3"));
    assert!(!matches("1.2.2", "=1.2.3"));

    assert!(!matches("1.2.4", "<=1.2.3"));
    assert!(matches("1.2.3", "<=1.2.3"));
    assert!(matches("1.2.2", "<=1.2.3"));

    assert!(!matches("1.2.4", "<1.2.3"));
    assert!(!matches("1.2.3", "<1.2.3"));
    assert!(matches("1.2.2", "<1.2.3"));

    // with pre-release
    assert!(!matches("1.2.4-0", ">=1.2.3"));
    assert!(!matches("1.2.3-0", ">=1.2.2-0"));
    assert!(!matches("1.2.3-0", ">=1.2.3-1"));
    assert!(matches("1.2.3-1", ">=1.2.3-0"));
    assert!(matches("1.2.3", ">=1.2.3-0"));
    assert!(matches("1.2.4", ">=1.2.3-0"));
}
