# rs-semver

Tools for dealing with semantic versions (see [semver.org](https://semver.org/))

## API
* `Version` (`Clone`, `Debug`, `Display`, `Eq`, `Hash`, `Ord`, `PartialEq`, `PartialOrd`)
    * `::cmp_with_build`
    * `::find_difference`
    * `::from`
    * `::to_incremented`
* `Range` (`Clone`, `Debug`, `Display`, `PartialEq`)
    * `::from`
    * `::is_matched_by`
