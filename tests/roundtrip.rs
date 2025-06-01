use proptest::proptest;
use radnelac::calendar::Armenian;
use radnelac::common::bound::BoundedDayCount;
use radnelac::day_count::Fixed;
use radnelac::day_count::FromFixed;
use radnelac::day_count::ToFixed;
use radnelac::day_count::FIXED_MAX;
use radnelac::day_count::FIXED_MIN;
use std::fmt::Debug;

fn roundtrip<T: FromFixed + ToFixed + PartialEq + Debug>(t: f64) {
    let f0 = Fixed::new(t).to_day();
    let d0 = T::from_fixed(f0);
    let f1 = d0.to_fixed();
    let d1 = T::from_fixed(f1);
    assert_eq!(d0, d1, "t = {:?}; f0 = {:?}; f1 = {:?}", t, f0, f1);
    assert_eq!(f0, f1, "t = {:?}; d0 = {:?}; d1 = {:?}", t, d0, d1);
}

proptest! {
    #[test]
    fn armenian(t in FIXED_MIN..FIXED_MAX) {
        roundtrip::<Armenian>(t);
    }
}
