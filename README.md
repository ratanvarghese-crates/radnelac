# Radnelac

This is a Rust crate for calendrical calculations: given a day represented in
one timekeeping system, this crate can create the representation for the same
day in another timekeeping system.

Additionally, the crate can convert dates to strings in some predefined formats.

For example, here is a conversion from the Gregorian calendar to the Julian:

```rust
use radnelac::calendar::*;
use radnelac::day_count::*;

let g = Gregorian::try_new(2025, GregorianMonth::July, 26).unwrap();
let j = g.convert::<Julian>();
assert_eq!(j, Julian::try_new(2025, JulianMonth::July, 13).unwrap());
```

Most of the calculations are based on *Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.

## Project Links

The primary copy of the source is the [Fossil repo at radnelac.org](https://fossil.radnelac.org/radnelac). If this code is hosted elsewhere, it is probably a mirror or fork.

Other relevant links:

+ [GitHub mirror](https://github.com/ratanvarghese-crates/radnelac)
+ [crates.io](https://crates.io/crates/radnelac)
+ [docs.rs](https://docs.rs/radnelac)

## Automated Test Results

Automated tests are run daily and the results are available on [radnelac.org](https://radnelac.org).

| Features    | Results                                              | Coverage |
|-------------|------------------------------------------------------|----------|
| default     | [Test results](https://www.radnelac.org/test-results/default/results.txt)    | [Coverage](https://www.radnelac.org/test-results/default/llvm-cov/html/) |
| no default  | [Test results](https://www.radnelac.org/test-results/no-default/results.txt) | [Coverage](https://www.radnelac.org/test-results/no-default/llvm-cov/html/) |

## License

Radnelac is provided under the [Mozilla Public License Version 2.0](https://www.mozilla.org/en-US/MPL/2.0/).

The Fossil repository for radnelac uses [highlight.js]() for syntax highlighting, and is provided under the [BSD 3-Clause License](https://github.com/highlightjs/highlight.js/blob/main/LICENSE). This is not used when building Radnelac or running code using Radnelac.