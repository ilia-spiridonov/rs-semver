use std::fmt;

use crate::version_core::VersionCore;

pub struct Version<'a> {
    pub core: VersionCore,
    pub pre_release: Option<&'a str>,
    pub build: Option<&'a str>,
}

impl fmt::Display for Version<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.core)?;

        if let Some(pre_release) = self.pre_release {
            write!(f, "-{}", pre_release)?;
        }

        if let Some(build) = self.build {
            write!(f, "+{}", build)?;
        }

        Ok(())
    }
}

impl<'a> Version<'a> {
    fn new(core: VersionCore, pre_release: Option<&'a str>, build: Option<&'a str>) -> Self {
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
        Version::new(core.clone(), Some("foo"), None).to_string()
    );
    assert_eq!(
        "1.2.3+foo",
        Version::new(core.clone(), None, Some("foo")).to_string()
    );
    assert_eq!(
        "1.2.3-foo.bar+baz",
        Version::new(core, Some("foo.bar"), Some("baz")).to_string()
    );
}

impl Version<'_> {
    // TODO
}
