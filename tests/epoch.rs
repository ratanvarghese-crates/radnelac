// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use radnelac::day_count::BoundedDayCount;
use radnelac::day_count::Epoch;
use radnelac::day_count::Fixed;
use radnelac::day_count::FromFixed;
use radnelac::day_count::ModifiedJulianDay;
use radnelac::day_count::UnixMoment;

use radnelac::day_count::JulianDay;

fn around_epoch<U: Epoch + FromFixed + BoundedDayCount<f64>>(diff: f64) {
    let before = Fixed::new(U::epoch().get() - 1.0);
    let exact = Fixed::new(U::epoch().get() + 0.0);
    let after = Fixed::new(U::epoch().get() + 1.0);
    assert_eq!(U::from_fixed(before).get(), -diff);
    assert_eq!(U::from_fixed(exact).get(), 0.0);
    assert_eq!(U::from_fixed(after).get(), diff);
}

fn around_epoch_i<U: Epoch + FromFixed + BoundedDayCount<i64>>(diff: i64) {
    let before = Fixed::new(U::epoch().get() - 1.0);
    let exact = Fixed::new(U::epoch().get() + 0.0);
    let after = Fixed::new(U::epoch().get() + 1.0);
    assert_eq!(U::from_fixed(before).get(), -diff);
    assert_eq!(U::from_fixed(exact).get(), 0);
    assert_eq!(U::from_fixed(after).get(), diff);
}

#[test]
fn jd_around_epoch() {
    around_epoch::<JulianDay>(1.0);
}

#[test]
fn mjd_around_epoch() {
    around_epoch::<ModifiedJulianDay>(1.0);
}

#[test]
fn unix_around_epoch() {
    const UNIX_DAY: i64 = 24 * 60 * 60;
    around_epoch_i::<UnixMoment>(UNIX_DAY);
}
