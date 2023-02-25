use std::fmt;

use super::super::Version;
use super::RangeComparator;

#[derive(Debug, PartialEq)]
pub struct RangeBound(pub RangeComparator, pub Version);

impl fmt::Display for RangeBound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}
