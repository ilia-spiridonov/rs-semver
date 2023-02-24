use super::super::Version;
use super::RangeComparator;

#[derive(Debug, PartialEq)]
pub struct RangeBound(pub RangeComparator, pub Version);
