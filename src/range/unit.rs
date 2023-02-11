use std::fmt;

use super::RangeComparator;
use crate::{Version, VersionPattern};

#[derive(Debug, PartialEq)]
pub enum RangeUnit<'a> {
    Pattern(RangeComparator, VersionPattern),
    Version(RangeComparator, Version<'a>),
}

// impl fmt::Display for RangeUnit {
//     // ...
// }
