use std::cmp;

use super::super::Version;
use super::comparator::RangeComparator;
use super::unit::{RangeBound, RangeUnit};
use super::Range;

impl RangeUnit<'_> {
    pub(crate) fn is_satisfied_by(&self, ver: &Version) -> bool {
        if let Some(extra_bound) = &self.extra_bound {
            Self::is_bound_satisfied_by(&self.bound, ver)
                && Self::is_bound_satisfied_by(extra_bound, ver)
        } else {
            Self::is_bound_satisfied_by(&self.bound, ver)
        }
    }

    fn is_bound_satisfied_by((comp, bound_ver): &RangeBound, ver: &Version) -> bool {
        use RangeComparator::*;

        if ver.pre_release.is_some()
            && (bound_ver.pre_release.is_none() || bound_ver.core != ver.core)
        {
            return false;
        }

        matches!(
            (ver.cmp(bound_ver), comp),
            (cmp::Ordering::Less, Less | LessOrEqual)
                | (cmp::Ordering::Equal, LessOrEqual | Equal | GreaterOrEqual)
                | (cmp::Ordering::Greater, GreaterOrEqual | Greater)
        )
    }
}

#[test]
fn test_range_unit_is_satisfied_by() {
    let test = |v, r| {
        RangeUnit::parse(r)
            .unwrap()
            .0
            .is_satisfied_by(&Version::from(v).unwrap())
    };

    assert!(test("1.2.4", ">1.2.3"));
    assert!(!test("1.2.3", ">1.2.3"));
    assert!(!test("1.2.2", ">1.2.3"));

    assert!(test("1.2.4", ">=1.2.3"));
    assert!(test("1.2.3", ">=1.2.3"));
    assert!(!test("1.2.2", ">=1.2.3"));

    assert!(!test("1.2.4", "=1.2.3"));
    assert!(test("1.2.3", "=1.2.3"));
    assert!(!test("1.2.2", "=1.2.3"));

    assert!(!test("1.2.4", "<=1.2.3"));
    assert!(test("1.2.3", "<=1.2.3"));
    assert!(test("1.2.2", "<=1.2.3"));

    assert!(!test("1.2.4", "<1.2.3"));
    assert!(!test("1.2.3", "<1.2.3"));
    assert!(test("1.2.2", "<1.2.3"));

    // with pre-release
    assert!(!test("1.2.4-0", ">=1.2.3"));
    assert!(!test("1.2.3-0", ">=1.2.2-0"));
    assert!(!test("1.2.3-0", ">=1.2.3-1"));
    assert!(test("1.2.3-1", ">=1.2.3-0"));
    assert!(test("1.2.3", ">=1.2.3-0"));
    assert!(test("1.2.4", ">=1.2.3-0"));
}

impl Range<'_> {
    /// Checks whether the given `ver` satisfies the range.
    ///
    /// Note that it follows node-semver's behaviour when it comes to pre-release versions:
    /// if `ver` has a pre-release tag, it can only be matched by a version that
    /// also has a pre-release tag AND exactly the same version core.
    pub fn is_satisfied_by(&self, ver: &Version) -> bool {
        match self {
            Self::Just(unit) => unit.is_satisfied_by(ver),
            Self::All(units) => units.iter().all(|u| u.is_satisfied_by(ver)),
            Self::Any(unit_groups) => unit_groups
                .iter()
                .any(|us| us.iter().all(|u| u.is_satisfied_by(ver))),
        }
    }
}

#[test]
fn test_range_is_satisfied_by() {
    let test = |v, r| {
        Range::from(r)
            .unwrap()
            .is_satisfied_by(&Version::from(v).unwrap())
    };

    assert!(test("1.2.3", ">=1.0.0"));
    assert!(test("1.2.3", ">=1.0.0 <2.0.0"));
    assert!(!test("2.0.0", ">=1.0.0 <2.0.0"));
    assert!(test("2.0.0", ">=2.0.0 || >=1.0.0 <1.5.0"));
    assert!(test("1.2.3", ">=2.0.0 || >=1.0.0 <1.5.0"));
    assert!(!test("1.5.0", ">=2.0.0 || >=1.0.0 <1.5.0"));
}
