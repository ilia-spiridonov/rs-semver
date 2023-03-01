# rs-semver

Tools for dealing with semantic versions (see [semver.org](https://semver.org))

## API
* `Range` (`Clone`, `Debug`, `Display`, `PartialEq`)
    * `::from`
    * `::is_matched_by`

## Usage
### Version
Structure `Version` represents a valid Semantic Version as described in the specification (not quite, as `rs-semver` allows the commonly added `v` prefix while the spec explicitly rejects it).

The only permitted way to construct it is to pass a `&str` to `Version::from`:
```rust
use rs_semver::Version;

// Version owns everything, so "rc.5" will be copied to the heap; most versions don't have tags however
let ver = Version::from("4.109.2-rc.5").unwrap();

println!("major version is {}", ver.core.major);
println!("{} is a pre-release version: {}", ver, ver.pre_release.is_some());
```

A `Version` consists of three fields (`core`, `pre_release`, and `build`), with the latter two being optional:
* `core` represents the three non-negative integers also known as `major`, `minor`, and `patch`, which together form the version number;
* `pre_release` represents an optional pre-release tag; its presence affects comparisons and `Range` matching (see below);
* `build` represents optional build metadata; mostly just a label and doesn't affect anything.

Versions are comparable (`Eq`, `Ord`) and hash-able (`Hash`) as per the spec.
Note that this implies that `build` is always ignored.
If you would like to also sort by build metadata lexicographically consider `Version::cmp_with_build`.

Additionally, it's possible to directly compute the difference between any two Versions via `Version::find_difference`.

Finally, incrementing a Version is achieved by passing a `VersionDiff` (e.g. `VersionDiff::Major`) to `Version::to_incremented`.
