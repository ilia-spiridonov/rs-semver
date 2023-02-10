use std::{cmp, fmt};

use self::core::VersionCore;
use pre_release::VersionPreRelease;

mod common;
mod core;
mod pre_release;

#[derive(Debug, PartialEq)]
pub struct Version<'a> {
    pub core: VersionCore,
    pub pre_release: Option<VersionPreRelease<'a>>,
    pub build: Option<&'a str>,
}

impl fmt::Display for Version<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.core)?;

        if let Some(pre_release) = &self.pre_release {
            write!(f, "{}", pre_release)?;
        }

        if let Some(build) = self.build {
            write!(f, "+{}", build)?;
        }

        Ok(())
    }
}

impl<'a> Version<'a> {
    fn new(
        core: VersionCore,
        pre_release: Option<VersionPreRelease<'a>>,
        build: Option<&'a str>,
    ) -> Self {
        Self {
            core,
            pre_release,
            build,
        }
    }
}

#[test]
fn test_display() {
    let core = VersionCore::new(1, 2, 3);

    assert_eq!("1.2.3", Version::new(core.clone(), None, None).to_string());
    assert_eq!(
        "1.2.3-foo",
        Version::new(core.clone(), Some(VersionPreRelease("foo")), None).to_string()
    );
    assert_eq!(
        "1.2.3+foo",
        Version::new(core.clone(), None, Some("foo")).to_string()
    );
    assert_eq!(
        "1.2.3-foo.bar+baz",
        Version::new(core, Some(VersionPreRelease("foo.bar")), Some("baz")).to_string()
    );
}

impl PartialOrd for Version<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        use cmp::Ordering::*;

        if let ord @ Some(Less | Greater) = self.core.partial_cmp(&other.core) {
            return ord;
        }

        match (&self.pre_release, &other.pre_release) {
            (None, None) => Some(Equal),
            (None, Some(_)) => Some(Greater),
            (Some(_), None) => Some(Less),
            (Some(pre), Some(other_pre)) => pre.partial_cmp(other_pre),
        }
    }
}

#[test]
fn test_ord() {
    let core = VersionCore::new(1, 0, 0);

    assert!(
        Version::new(VersionCore::new(0, 1, 2), None, None)
            < Version::new(VersionCore::new(1, 0, 0), None, None)
    );
    assert!(
        Version::new(VersionCore::new(1, 1, 2), None, None)
            < Version::new(VersionCore::new(1, 2, 3), None, None)
    );
    assert!(
        Version::new(VersionCore::new(1, 1, 2), None, None)
            < Version::new(VersionCore::new(1, 1, 3), None, None)
    );
    assert!(
        Version::new(core.clone(), Some(VersionPreRelease("alpha")), None)
            < Version::new(core.clone(), Some(VersionPreRelease("alpha.1")), None)
    );
    assert!(
        Version::new(core.clone(), Some(VersionPreRelease("alpha.1")), None)
            < Version::new(core.clone(), Some(VersionPreRelease("alpha.beta")), None)
    );
    assert!(
        Version::new(core.clone(), Some(VersionPreRelease("alpha.beta")), None)
            < Version::new(core.clone(), Some(VersionPreRelease("beta")), None)
    );
    assert!(
        Version::new(core.clone(), Some(VersionPreRelease("beta")), None)
            < Version::new(core.clone(), Some(VersionPreRelease("beta.2")), None)
    );
    assert!(
        Version::new(core.clone(), Some(VersionPreRelease("beta.2")), None)
            < Version::new(core.clone(), Some(VersionPreRelease("beta.11")), None)
    );
    assert!(
        Version::new(core.clone(), Some(VersionPreRelease("beta.11")), None)
            < Version::new(core.clone(), Some(VersionPreRelease("rc.1")), None)
    );
    assert!(
        Version::new(core.clone(), Some(VersionPreRelease("rc.1")), None)
            < Version::new(core, None, None)
    );
}
