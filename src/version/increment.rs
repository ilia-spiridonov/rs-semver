use super::{Version, VersionPreRelease};

pub enum VersionIncrement {
    Major,
    Minor,
    Patch,
}

impl Version<'_> {
    /// Increments the version in-place according to the rules described on https://semver.org and depending on `kind`.
    ///
    /// If `drop_pre_release` is `false`, then the version's `pre_release` will be set to `"0"` if it's not `None`.
    ///
    /// The version's `build` is always reset to `None`.
    pub fn increment(&mut self, kind: VersionIncrement, drop_pre_release: bool) {
        use VersionIncrement::*;

        match kind {
            Major => {
                self.core.major += 1;
                self.core.minor = 0;
                self.core.patch = 0;
            }
            Minor => {
                self.core.minor += 1;
                self.core.patch = 0;
            }
            Patch => {
                self.core.patch += 1;
            }
        };

        if self.pre_release.is_some() {
            self.pre_release = if drop_pre_release {
                None
            } else {
                Some(VersionPreRelease("0"))
            };
        }

        self.build = None;
    }
}

#[test]
fn test_increment() {
    use VersionIncrement::*;

    let ver = Version::from("1.2.3").unwrap();

    {
        let mut ver = ver.clone();
        ver.increment(Major, true);
        assert_eq!(Version::from("2.0.0").unwrap(), ver);
    }
    {
        let mut ver = ver.clone();
        ver.increment(Minor, true);
        assert_eq!(Version::from("1.3.0").unwrap(), ver);
    }
    {
        let mut ver = ver.clone();
        ver.increment(Patch, true);
        assert_eq!(Version::from("1.2.4").unwrap(), ver);
    }

    let ver = Version::from("4.5.6-rc.0").unwrap();

    {
        let mut ver = ver.clone();
        ver.increment(Patch, true);
        assert_eq!(Version::from("4.5.7").unwrap(), ver);
    }
    {
        let mut ver = ver.clone();
        ver.increment(Patch, false);
        assert_eq!(Version::from("4.5.7-0").unwrap(), ver);
    }
}
