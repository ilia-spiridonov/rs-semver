use super::{Version, VersionCore, VersionPreRelease};

#[derive(Clone, Copy)]
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

        let core = match (diff, self.pre_release.is_some()) {
            (Major | Minor | Patch, true) => self.core.clone(),
            (Major | PreMajor, _) => VersionCore::new(self.core.major + 1, 0, 0),
            (Minor | PreMinor, _) => VersionCore::new(self.core.major, self.core.minor + 1, 0),
            (Patch | PrePatch, _) | (PreRelease, false) => {
                VersionCore::new(self.core.major, self.core.minor, self.core.patch + 1)
            }
            (PreRelease, true) => self.core.clone(),
        };

        let pre_release = match (diff, &self.pre_release) {
            (Major | Minor | Patch, _) => None,
            (PreMajor | PreMinor | PrePatch, _) | (PreRelease, None) => {
                Some(VersionPreRelease::default())
            }
            (PreRelease, Some(pre)) => Some(pre.to_incremented()),
        };

        Self::new(core, pre_release, None)
    }

    /// Finds the largest difference between two versions.
    ///
    /// If their cores are equal, then `pre_release` tags are compared: if they're equal (or both missing), then `None` is returned.
    /// Otherwise,`PreRelease` is returned (even if only one tag is present).
    ///
    /// Otherwise, if both versions don't have pre-release tags, then the normal logic (`Major | Minor | Patch`) applies for the core.
    /// Or, if there's at least one tag, then the output is `PreMajor | PreMinor | PrePatch`.
    ///
    /// In any case, `build` is always ignored.
    pub fn find_difference(&self, other: &Self) -> Option<VersionDiff> {
        use VersionDiff::*;

        None
    }
}

#[test]
fn test_to_incremented() {
    use VersionDiff::*;

    let test = |v, d| Version::from(v).unwrap().to_incremented(d).to_string();

    assert_eq!("2.0.0", test("1.2.3", Major));
    assert_eq!("1.2.3", test("1.2.3-foo", Major));
    assert_eq!("2.0.0", test("1.2.3+foo", Major));
    assert_eq!("1.3.0", test("1.2.3", Minor));
    assert_eq!("1.2.3", test("1.2.3-foo", Minor));
    assert_eq!("1.2.4", test("1.2.3", Patch));
    assert_eq!("1.2.3", test("1.2.3-foo", Patch));
    assert_eq!("2.0.0-0", test("1.2.3", PreMajor));
    assert_eq!("2.0.0-0", test("1.2.3-foo", PreMajor));
    assert_eq!("1.3.0-0", test("1.2.3", PreMinor));
    assert_eq!("1.3.0-0", test("1.2.3-foo", PreMinor));
    assert_eq!("1.2.4-0", test("1.2.3", PrePatch));
    assert_eq!("1.2.4-0", test("1.2.3-foo", PrePatch));
    assert_eq!("1.2.4-0", test("1.2.3", PreRelease));
    assert_eq!("1.2.3-foo.0", test("1.2.3-foo", PreRelease));
    assert_eq!("1.2.3-0.foo.1.bar", test("1.2.3-0.foo.0.bar", PreRelease));
}

#[test]
fn test_find_difference() {
    use VersionDiff::*;

    let test = |v, w| {
        Version::from(v)
            .unwrap()
            .find_difference(&Version::from(w).unwrap())
    };

    assert!(matches!(test("1.2.3", "2.3.4"), Some(Major)));
    assert!(matches!(test("1.2.3", "2.3.4-foo"), Some(PreMajor)));
    assert!(matches!(test("1.2.3", "1.3.4"), Some(Minor)));
    assert!(matches!(test("1.2.3", "1.3.4-foo"), Some(PreMinor)));
    assert!(matches!(test("1.2.3", "1.2.4"), Some(Patch)));
    assert!(matches!(test("1.2.3", "1.2.4-foo"), Some(PrePatch)));
    assert!(matches!(test("1.2.3", "1.2.3-foo"), Some(PreRelease)));
    assert!(matches!(test("1.2.3-foo", "1.2.3-bar"), Some(PreRelease)));
    assert!(matches!(test("1.2.3", "1.2.3"), None));
    assert!(matches!(test("1.2.3", "1.2.3+foo"), None));
    assert!(matches!(test("1.2.3-foo", "1.2.3-foo"), None));
}