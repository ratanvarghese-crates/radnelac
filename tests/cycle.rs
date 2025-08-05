use proptest::proptest;
use radnelac::day_count::BoundedDayCount;
use radnelac::day_count::Fixed;
use radnelac::day_count::FromFixed;
use radnelac::day_count::FIXED_MAX;
use radnelac::day_cycle::Akan;
use radnelac::day_cycle::BoundedCycle;
use radnelac::day_cycle::OnOrBefore;
use radnelac::day_cycle::Weekday;

fn repeats<const N: u8, const M: u8, T: BoundedCycle<N, M> + FromFixed>(x: f64) {
    let f1 = Fixed::new(x);
    let a1 = T::from_fixed(f1);
    let a2 = T::from_fixed(Fixed::new(f1.get() + 1.0));
    let a3 = T::from_fixed(Fixed::new(f1.get() + (T::cycle_length() as f64)));
    assert_ne!(a1, a2);
    assert_eq!(a1, a3);
}

fn on_or_before<const N: u8, const M: u8, T: OnOrBefore<N, M> + FromFixed>(x1: f64, w: u8) {
    let w = T::from_u8(w).unwrap();
    let d1 = w.on_or_before(Fixed::new(x1));
    let d2 = w.on_or_before(d1);
    assert_eq!(d1, d2);
    let x2 = d2.get_day_i() as i32;
    for i in 1..(T::cycle_length() - 1) {
        let d3 = w.on_or_before(Fixed::cast_new(x2 - (i as i32)));
        assert_ne!(d1, d3);
    }
}

fn nearby<const N: u8, const M: u8, T: OnOrBefore<N, M> + FromFixed>(x1: f64, w: u8) {
    let w = T::from_i64(w as i64).unwrap();
    let f0 = Fixed::cast_new(x1 as i64);
    let f1 = w.on_or_before(f0);
    let f2 = w.on_or_after(f0);
    let f3 = w.nearest(f0);
    let f4 = w.before(f0);
    let f5 = w.after(f0);
    assert!(f1 <= f0);
    assert!(f2 >= f0);
    assert!(f4 < f0);
    assert!(f5 > f0);

    let diff = f0.get() - f3.get();
    let max_diff = ((T::cycle_length() / 2) + 1) as f64;
    assert!(-max_diff <= diff && diff <= max_diff);
}

proptest! {
    #[test]
    fn weekday_repeats(x in (-FIXED_MAX)..(FIXED_MAX - 7.0)) {
        repeats::<7, 0, Weekday>(x);
    }

    #[test]
    fn akan_repeats(x in (-FIXED_MAX)..(FIXED_MAX - 7.0)) {
        repeats::<42, 1, Akan>(x);
    }

    #[test]
    fn weekday_on_or_before(x1 in (-FIXED_MAX+14.0)..FIXED_MAX, w in 0..6) {
        on_or_before::<7, 0, Weekday>(x1, w as u8);
    }

    #[test]
    fn akan_on_or_before(x1 in (-FIXED_MAX+42.0)..FIXED_MAX, w in 1..42) {
        on_or_before::<42, 1, Akan>(x1, w as u8);
    }

    #[test]
    fn weekday_nearby(x1 in (-FIXED_MAX+14.0)..FIXED_MAX, w in 0..6) {
        nearby::<7, 0, Weekday>(x1, w as u8);
    }

    #[test]
    fn akan_nearby(x1 in (-FIXED_MAX+42.0)..FIXED_MAX, w in 1..42) {
        nearby::<42, 1, Akan>(x1, w as u8);
    }
}
