use std::cmp;

use super::super::Version;
use super::{Range, RangeBound, RangeComparator, RangeUnit};

#[derive(Clone, Copy)]
pub enum MatchingAlg {
    Classic,
    Node,
}

impl RangeBound {
    fn is_matched_by(&self, alg: MatchingAlg, ver: &Version) -> bool {
        use RangeComparator::*;

        if matches!(alg, MatchingAlg::Node)
            && ver.pre_release.is_some()
            && (self.1.pre_release.is_none() || self.1.core != ver.core)
        {
            return false;
        }

        matches!(
            (ver.cmp(&self.1), &self.0),
            (cmp::Ordering::Less, Less | LessOrEqual)
                | (cmp::Ordering::Equal, LessOrEqual | Equal | GreaterOrEqual)
                | (cmp::Ordering::Greater, GreaterOrEqual | Greater)
        )
    }
}

impl RangeUnit {
    fn is_matched_by(&self, alg: MatchingAlg, ver: &Version) -> bool {
        if let Some(extra_bound) = &self.extra_bound {
            self.bound.is_matched_by(alg, ver) && extra_bound.is_matched_by(alg, ver)
        } else {
            self.bound.is_matched_by(alg, ver)
        }
    }
}

#[test]
fn test_range_unit_is_matched_by_alg_classic() {
    let test = |v, r| {
        RangeUnit::parse(r)
            .unwrap()
            .0
            .is_matched_by(MatchingAlg::Classic, &Version::from(v).unwrap())
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

    assert!(!test("1.2.3-0", ">=1.2.3-1"));
    assert!(test("1.2.3-1", ">=1.2.3-0"));
    assert!(test("1.2.3", ">=1.2.3-0"));
    assert!(test("1.2.4", ">=1.2.3-0"));
}

#[test]
fn test_range_unit_is_matched_by_alg_node() {
    let test = |v, r| {
        RangeUnit::parse(r)
            .unwrap()
            .0
            .is_matched_by(MatchingAlg::Node, &Version::from(v).unwrap())
    };

    assert!(!test("1.2.4-0", ">=1.2.3"));
    assert!(!test("1.2.3-0", ">=1.2.2-0"));
}

impl Range {
    /// Checks whether the given version matches the range.
    ///
    /// When `alg` is `Classic` all version comparisons will strictly follow the SemVer specification.
    ///
    /// When `alg` is `Node` then `node-semver`'s behaviour will be followed for pre-release versions:
    /// if `ver` has a pre-release tag, then it can only be compared to another version if it
    /// also has a pre-release tag AND the same version core.
    pub fn is_matched_by(&self, alg: MatchingAlg, ver: &Version) -> bool {
        match self {
            Self::Just(unit) => unit.is_matched_by(alg, ver),
            Self::All(units) => units.iter().all(|u| u.is_matched_by(alg, ver)),
            Self::Any(unit_groups) => unit_groups
                .iter()
                .any(|us| us.iter().all(|u| u.is_matched_by(alg, ver))),
        }
    }
}

#[test]
fn test_range_is_matched_by() {
    let test = |v, r| {
        Range::from(r)
            .unwrap()
            .is_matched_by(MatchingAlg::Classic, &Version::from(v).unwrap())
    };

    assert!(test("1.2.3", ">=1.0.0"));
    assert!(test("1.2.3", ">=1.0.0 <2.0.0"));
    assert!(!test("2.0.0", ">=1.0.0 <2.0.0"));
    assert!(test("2.0.0", ">=2.0.0 || >=1.0.0 <1.5.0"));
    assert!(test("1.2.3", ">=2.0.0 || >=1.0.0 <1.5.0"));
    assert!(!test("1.5.0", ">=2.0.0 || >=1.0.0 <1.5.0"));
}
