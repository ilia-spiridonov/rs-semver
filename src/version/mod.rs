use std::{cmp, fmt};

pub use self::{
    build::VersionBuild, core::VersionCore, increment::VersionIncrement,
    pre_release::VersionPreRelease,
};

mod build;
mod common;
mod core;
mod increment;
mod pattern;
mod pre_release;

#[derive(Clone, Debug)]
pub struct Version<'a> {
    pub core: VersionCore,
    pub pre_release: Option<VersionPreRelease<'a>>,
    pub build: Option<VersionBuild<'a>>,
}

impl fmt::Display for Version<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.core)?;

        if let Some(pre_release) = &self.pre_release {
            write!(f, "{}", pre_release)?;
        }

        if let Some(build) = &self.build {
            write!(f, "{}", build)?;
        }

        Ok(())
    }
}

impl<'a> Version<'a> {
    pub fn new(
        core: VersionCore,
        pre_release: Option<VersionPreRelease<'a>>,
        build: Option<VersionBuild<'a>>,
    ) -> Self {
        Self {
            core,
            pre_release,
            build,
        }
    }
}

#[test]
fn test_to_string() {
    let core = VersionCore::new(1, 2, 3);

    assert_eq!("1.2.3", Version::new(core.clone(), None, None).to_string());
    assert_eq!(
        "1.2.3-foo",
        Version::new(core.clone(), Some(VersionPreRelease("foo")), None).to_string()
    );
    assert_eq!(
        "1.2.3+foo",
        Version::new(core.clone(), None, Some(VersionBuild("foo"))).to_string()
    );
    assert_eq!(
        "1.2.3-foo.bar+baz",
        Version::new(
            core,
            Some(VersionPreRelease("foo.bar")),
            Some(VersionBuild("baz"))
        )
        .to_string()
    );
}

impl PartialEq for Version<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.core == other.core && self.pre_release == other.pre_release
    }
}

impl Eq for Version<'_> {}

impl PartialOrd for Version<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version<'_> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        use cmp::Ordering::*;

        if let ord @ (Less | Greater) = self.core.cmp(&other.core) {
            return ord;
        }

        match (&self.pre_release, &other.pre_release) {
            (None, None) => Equal,
            (None, Some(_)) => Greater,
            (Some(_), None) => Less,
            (Some(pre), Some(other_pre)) => pre.cmp(other_pre),
        }
    }
}

#[test]
fn test_eq() {
    let core = VersionCore::new(1, 0, 0);

    assert_eq!(
        Version::new(core.clone(), None, None),
        Version::new(core, None, Some(VersionBuild("foo")))
    );
}

#[test]
fn test_cmp() {
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

impl<'a> Version<'a> {
    /// Attempts to build a semantic version representation from the given slice `s`
    /// using the rules described on https://semver.org.
    ///
    /// Note that it deviates from them slightly by allowing the `v` prefix which is commonly used in practice.
    ///
    /// If there are any additional (e.g. whitespace) characters around the version, make sure to trim them beforehand.
    pub fn from(s: &'a str) -> Option<Self> {
        let (ver, r) = Self::parse(s)?;

        if r.is_empty() {
            Some(ver)
        } else {
            None
        }
    }

    pub(crate) fn parse(s: &'a str) -> Option<(Self, &'a str)> {
        let r = s.strip_prefix('v').unwrap_or(s);
        let (core, r) = VersionCore::parse(r)?;
        let (pre_release, r) = VersionPreRelease::parse(r)?;
        let (build, r) = VersionBuild::parse(r)?;

        Some((Self::new(core, pre_release, build), r))
    }
}

#[test]
fn test_from() {
    assert_eq!(None, Version::from("1.2.3 "));
    assert_eq!(
        Some(Version::new(VersionCore::new(1, 2, 3), None, None)),
        Version::from("1.2.3")
    );
}

#[test]
fn test_parse() {
    assert_eq!(
        Some((Version::new(VersionCore::new(1, 2, 3), None, None), " foo")),
        Version::parse("v1.2.3 foo")
    );
}
