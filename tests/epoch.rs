use radnelac::common::bound::BoundedDayCount;
use radnelac::common::math::TermNum64;
use radnelac::day_count::Epoch;
use radnelac::day_count::Fixed;
use radnelac::day_count::FromFixed;
use radnelac::day_count::ModifiedJulianDay;
use radnelac::day_count::UnixMoment;

use radnelac::day_count::JulianDay;

fn around_epoch<T: TermNum64 + std::fmt::Debug, U: Epoch + FromFixed + BoundedDayCount<T>>(
    diff: T,
) {
    let before = Fixed::new(U::epoch().get() - 1.0);
    let exact = Fixed::new(U::epoch().get() + 0.0);
    let after = Fixed::new(U::epoch().get() + 1.0);
    assert_eq!(U::from_fixed(before).get(), -diff);
    assert_eq!(U::from_fixed(exact).get(), T::zero());
    assert_eq!(U::from_fixed(after).get(), diff);
}

#[test]
fn jd_around_epoch() {
    around_epoch::<f64, JulianDay>(1.0);
}

#[test]
fn mjd_around_epoch() {
    around_epoch::<f64, ModifiedJulianDay>(1.0);
}

#[test]
fn unix_around_epoch() {
    const UNIX_DAY: i64 = 24 * 60 * 60;
    around_epoch::<i64, UnixMoment>(UNIX_DAY);
}
