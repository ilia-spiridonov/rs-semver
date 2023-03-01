# rs-semver
Tools for dealing with semantic versions (see [semver.org](https://semver.org))

## Installation
Install with Cargo directly from this Git repo.

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
If you would like to also sort by build metadata lexicographically, consider `Version::cmp_with_build`.

Additionally, it's possible to directly compute the difference between any two Versions via `Version::find_difference`.

Finally, incrementing a Version is achieved by passing a `VersionDiff` (e.g. `VersionDiff::Major`) to `Version::to_incremented`. This method may be changed in the near future to perform an in-place mutation instead.

### Range
Structure `Range` represents a range of Versions, described using a syntax largely similar to that of `node-semver` (because `rs-semver` was written with the goal of supporting of an implementation of a Node-compatible package manager) but significantly stricter (many ambiguous/meaningless inputs will be rejected).

A `Range` is constructed by parsing a `&str`:
```rust
use rs_semver::Range;

// No allocations occur unless the Range contains multiple units (or 'sets') joined with AND (' ') or OR ('||')
let range = Range::from("^3.27.1").unwrap();

// Note: the output is the internal representation of the Range, which may not match the string input when serialized
// Also note: the upper bound Version has the '0' tag set to exclude all 4.0.0 Versions with pre-release tags
println!("range: {}", range); // => '>=3.27.1 <4.0.0-0'
```

To test whether a Version matches a Range use `Range::is_matched_by` with the chosen `MatchingAlg`:
* `MatchingAlg::Classic` strictly follows the spec when comparing Versions against bounds;
* `MatchingAlg::Node` follows `node-semver`'s approach: a Version with a pre-release tag is only compared to a bound Version if it also has a tag and their cores are completely equal; in this scheme a Version like `2.3.4-rc.5` does not match a Range like `>=1.2.3-rc.4`, even though this violates the spec.

More powerful matching (e.g. overlapping multiple `ranges`) can be easily achieved by composing iterators:
```rust
fn highest_matching_version<'a>(versions: &'a [Version], ranges: &[Range]) -> Option<&'a Version> {
    versions
        .iter()
        .filter(|ver| {
            ranges
                .iter()
                .all(|range| range.is_matched_by(MatchingAlg::Node, ver))
        })
        .max()
}
```
