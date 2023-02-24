use super::{Version, VersionCore};

pub enum VersionDiff {
    Major,
    PreMajor,
    Minor,
    PreMinor,
    Patch,
    PrePatch,
    PreRelease,
}

impl Version {
    /// Increments the version core following the rules described on https://semver.org and depending on `diff`.
    ///
    /// If `diff` is `Major | Minor | Patch` and the version has a pre-release tag, then `pre_release` will
    /// be dropped but the core will remain unchanged.
    ///
    /// If `diff` is `PreMajor | PreMinor | PrePatch`, then `pre_release` will always be just set to `0`.
    ///
    /// If `diff` is `PreRelease` and the version doesn't have a pre-release tag, then `pre_release` will
    /// be set to `0`. In this case the core will also be patch-incremented.
    /// Otherwise, the rightmost numeric part of the tag will be incremented.
    /// If the tag has no numeric parts, then a new `0` part will be appended to it.
    ///
    /// The version's `build` will always be dropped.
    pub fn to_incremented(&self, diff: VersionDiff) -> Self {
        use VersionDiff::*;

        let (major, minor, patch) = match diff {
            Major => (self.core.major + 1, 0, 0),
            Minor => (self.core.major, self.core.minor + 1, 0),
            Patch => (self.core.major, self.core.minor, self.core.patch + 1),
        };

        Self::new(VersionCore::new(major, minor, patch), None, None)
    }
}

#[test]
fn test_to_incremented() {
    use VersionDiff::*;

    let ver = Version::from("1.2.3-foo+bar").unwrap();

    assert_eq!(Version::from("2.0.0").unwrap(), ver.to_incremented(Major));
    assert_eq!(Version::from("1.3.0").unwrap(), ver.to_incremented(Minor));
    assert_eq!(Version::from("1.2.4").unwrap(), ver.to_incremented(Patch));
}
