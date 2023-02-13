use super::{Version, VersionCore};

pub enum VersionIncrement {
    Major,
    Minor,
    Patch,
}

impl Version<'_> {
    /// Increments the version according to the rules described on https://semver.org and depending on `inc`.
    ///
    /// The incremented version's `pre_release` and `build` will always be set to `None`.
    pub fn to_incremented(&self, inc: VersionIncrement) -> Self {
        use VersionIncrement::*;

        let (major, minor, patch) = match inc {
            Major => (self.core.major + 1, 0, 0),
            Minor => (self.core.major, self.core.minor + 1, 0),
            Patch => (self.core.major, self.core.minor, self.core.patch + 1),
        };

        Self::new(VersionCore::new(major, minor, patch), None, None)
    }
}

#[test]
fn test_to_incremented() {
    use VersionIncrement::*;

    let ver = Version::from("1.2.3-foo+bar").unwrap();

    assert_eq!(Version::from("2.0.0").unwrap(), ver.to_incremented(Major));
    assert_eq!(Version::from("1.3.0").unwrap(), ver.to_incremented(Minor));
    assert_eq!(Version::from("1.2.4").unwrap(), ver.to_incremented(Patch));
}
