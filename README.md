# radnelac

This is a crate for calendrical calculations: given a day represented in
one timekeeping system, this crate can create the representation for the same
day in another timekeeping system.

Additionally, the crate can convert dates to strings in some predefined formats.

For example, here is a conversion from the Gregorian calendar to the Julian:

```
use radnelac::calendar::*;
use radnelac::day_count::*;

let g = Gregorian::try_new(2025, GregorianMonth::July, 26).unwrap();
let j = g.convert::<Julian>();
assert_eq!(j, Julian::try_new(2025, JulianMonth::July, 13).unwrap());
```

## Links

The primary copy of the source is the [Fossil repo at radnelac.org](https://fossil.radnelac.org/radnelac). If this code is hosted elsewhere, it is probably a mirror or fork.

Other relevant links:

+ [GitHub mirror](https://github.com/ratanvarghese-crates/radnelac)
+ [crates.io](https://crates.io/crates/radnelac)
+ [docs.rs](https://docs.rs/radnelac)

## Automated Test Results

Automated tests are run regularly-ish (?) and the results are available on [radnelac.org](radnelac.org).

| Features    | Results                                              | Coverage |
|-------------|------------------------------------------------------|----------|
| default     | [Test results](https://www.radnelac.org/test-results/default/results.txt)    | [Coverage](https://www.radnelac.org/test-results/default/llvm-cov/html/) |
| no default  | [Test results](https://www.radnelac.org/test-results/no-default/results.txt) | [Coverage](https://www.radnelac.org/test-results/no-default/llvm-cov/html/) |