# rs-semver
Tools for dealing with semantic versions (see [semver.org](https://semver.org/))

## Version
Import `Version` to manipulate Semantic Versions. Parse from a slice:
```rust
use rs_semver::Version;

// cargo run 1.2.3-foo.bar+baz
fn main() {
    let raw_ver = std::env::args().skip(1).next().unwrap();
    if let Some(ver) = Version::from(&raw_ver) {
        println!("parsed version: {}", ver);
    }
}
```

Or create manually:
```rust
use rs_semver::{Version, VersionBuild, VersionCore, VersionIncrement, VersionPreRelease};

fn main() {
    let ver = Version::new(
        VersionCore::new(1, 2, 3),
        Some(VersionPreRelease("foo")),
        Some(VersionBuild("bar")),
    );

    let ver = ver.to_incremented(VersionIncrement::Major);

    println!(
        "is {} a pre-release version? {}",
        ver,
        ver.pre_release.is_some()
    );
}
```

Also, `Version` implements both `Eq` and `Ord`, using the algorithms described in the specification:
```rust
assert!(Version::from("1.0.0-rc.0") < Version::from("1.0.0"));
```

## Range
Import `Range` to manipulate `node-semver`-like version ranges. Parse from a slice:
```rust
use rs_semver::Range;

// cargo run ^1.2.3
fn main() {
    let raw_range = std::env::args().skip(1).next().unwrap();
    if let Some(range) = Range::from(&raw_range) {
        println!("parsed range: {}", range);
    }
}
```

Or create manually:
```rust
use rs_semver::{Range, RangeComparator, RangeUnit, Version};

fn main() {
    let range = Range::Just(RangeUnit::new(
        (RangeComparator::Less, Version::from("1.2.3").unwrap()),
        None,
    ));

    let ver = Version::from("1.0.0").unwrap();

    println!("does {} match {}? {}", ver, range, range.is_satisfied_by(&ver));
}
```
