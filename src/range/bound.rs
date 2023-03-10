use std::fmt;

use super::comparator::RangeComparator;
use crate::version::Version;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct RangeBound(pub(crate) RangeComparator, pub(crate) Version);

impl fmt::Display for RangeBound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

#[test]
fn test_to_string() {
    assert_eq!(
        "<1.2.3",
        RangeBound(RangeComparator::Less, Version::new(1, 2, 3)).to_string()
    );
}
