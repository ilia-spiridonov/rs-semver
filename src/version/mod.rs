use std::{cmp, fmt, hash};

pub use self::core::VersionCore;
pub use build::VersionBuild;
pub use difference::VersionDiff;
pub use pattern::VersionPattern;
pub use pre_release::VersionPreRelease;

mod build;
mod common;
mod core;
mod difference;
mod pattern;
mod pre_release;

#[derive(Clone, Debug)]
pub struct Version {
    pub core: VersionCore,
    pub pre_release: Option<VersionPreRelease>,
    pub build: Option<VersionBuild>,
}

impl fmt::Display for Version {
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

impl Version {
    pub fn new(
        core: VersionCore,
        pre_release: Option<VersionPreRelease>,
        build: Option<VersionBuild>,
    ) -> Self {
        Self {
            core,
            pre_release,
            build,
        }
    }

    pub fn with_core(major: u32, minor: u32, patch: u32) -> Self {
        Self::new(VersionCore::new(major, minor, patch), None, None)
    }
}

#[test]
fn test_to_string() {
    let core = VersionCore::new(1, 2, 3);

    assert_eq!("1.2.3", Version::new(core.clone(), None, None).to_string());
    assert_eq!(
        "1.2.3-foo",
        Version::new(
            core.clone(),
            Some(VersionPreRelease("foo".to_string())),
            None
        )
        .to_string()
    );
    assert_eq!(
        "1.2.3+foo",
        Version::new(core.clone(), None, Some(VersionBuild("foo".to_string()))).to_string()
    );
    assert_eq!(
        "1.2.3-foo.bar+baz",
        Version::new(
            core,
            Some(VersionPreRelease("foo.bar".to_string())),
            Some(VersionBuild("baz".to_string()))
        )
        .to_string()
    );
}

impl hash::Hash for Version {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.core.hash(state);
        self.pre_release.hash(state);
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.core == other.core && self.pre_release == other.pre_release
    }
}

impl Eq for Version {}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
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
    let parse = |s| Version::from(s).expect(s);

    assert!(parse("1.2.3") == parse("1.2.3+foo"));
    assert!(parse("1.2.3+foo") == parse("1.2.3+bar"));
}

#[test]
fn test_cmp() {
    let parse = |s| Version::from(s).expect(s);

    assert!(parse("0.1.2") < parse("1.0.0"));
    assert!(parse("1.1.2") < parse("1.2.3"));
    assert!(parse("1.1.2") < parse("1.1.3"));
    assert!(parse("1.0.0-alpha") < parse("1.0.0-alpha.1"));
    assert!(parse("1.0.0-alpha.1") < parse("1.0.0-alpha.beta"));
    assert!(parse("1.0.0-alpha.beta") < parse("1.0.0-beta"));
    assert!(parse("1.0.0-beta") < parse("1.0.0-beta.2"));
    assert!(parse("1.0.0-beta.2") < parse("1.0.0-beta.11"));
    assert!(parse("1.0.0-beta.11") < parse("1.0.0-rc.1"));
    assert!(parse("1.0.0-rc.1") < parse("1.0.0"));
}

impl Version {
    pub fn cmp_with_build(&self, other: &Self) -> cmp::Ordering {
        match self.cmp(other) {
            cmp::Ordering::Equal => self.build.cmp(&other.build),
            ord => ord,
        }
    }
}

#[test]
fn test_cmp_with_build() {
    use cmp::Ordering::*;

    let parse = |s| Version::from(s).expect(s);

    assert_eq!(Less, parse("1.2.3-0").cmp_with_build(&parse("1.2.3")));
    assert_eq!(Greater, parse("1.2.3-1").cmp_with_build(&parse("1.2.3-0")));
    assert_eq!(Less, parse("1.2.3").cmp_with_build(&parse("1.2.3+foo")));
    assert_eq!(Less, parse("1.2.3+f").cmp_with_build(&parse("1.2.3+foo")));
    assert_eq!(
        Equal,
        parse("1.2.3+foo").cmp_with_build(&parse("1.2.3+foo"))
    );
    assert_eq!(Greater, parse("1.2.3+foo").cmp_with_build(&parse("1.2.3")));
}

impl Version {
    /// Attempts to build a semantic version representation from the given slice `s`
    /// using the grammar described on https://semver.org.
    ///
    /// Note that it deviates from it slightly by allowing the `v` prefix which is commonly used in practice.
    ///
    /// If there are any additional (e.g. whitespace) characters around the version, make sure to trim them beforehand.
    pub fn from(s: &str) -> Option<Self> {
        let (ver, r) = Self::parse(s)?;

        if r.is_empty() {
            Some(ver)
        } else {
            None
        }
    }

    pub(crate) fn parse(s: &str) -> Option<(Self, &str)> {
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
    let parse = |s| Version::parse(s).expect(s).0.to_string();

    assert_eq!("1.2.3", parse("v1.2.3"));
    assert_eq!("1.2.3-foo.bar.0", parse("1.2.3-foo.bar.0"));
    assert_eq!("1.2.3+foo.01", parse("1.2.3+foo.01"));
    assert_eq!("1.2.3-foo+bar", parse("1.2.3-foo+bar"));
    assert_eq!(None, Version::parse("v"));
    assert_eq!(None, Version::parse("-foo"));
    assert_eq!(None, Version::parse("+foo"));
    assert_eq!(None, Version::parse("+foo-bar"));
}
