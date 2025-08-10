// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[cfg(feature = "display")]
mod display_logic {
    pub use proptest::proptest;
    pub use radnelac::day_count::BoundedDayCount;
    pub use radnelac::day_count::Fixed;
    pub use radnelac::day_count::FromFixed;
    pub use radnelac::day_count::FIXED_MAX;
    pub use radnelac::day_cycle::Akan;
    pub use radnelac::day_cycle::BoundedCycle;
    pub use radnelac::day_cycle::Weekday;

    pub fn repeats<const N: u8, const M: u8, T: BoundedCycle<N, M> + FromFixed + ToString>(x: f64) {
        let f1 = Fixed::new(x);
        let a1 = T::from_fixed(f1);
        let a2 = T::from_fixed(Fixed::new(f1.get() + 1.0));
        let a3 = T::from_fixed(Fixed::new(f1.get() + (T::cycle_length() as f64)));
        assert_ne!(a1.to_string(), a2.to_string());
        assert_eq!(a1.to_string(), a3.to_string());
    }
}

#[cfg(feature = "display")]
use display_logic::*;

#[cfg(feature = "display")]
proptest! {
    #[test]
    fn weekday_display_repeats(x in (-FIXED_MAX)..(FIXED_MAX - 7.0)) {
        repeats::<7, 0, Weekday>(x);
    }

    #[test]
    fn akan_display_repeats(x in (-FIXED_MAX)..(FIXED_MAX - 7.0)) {
        repeats::<42, 1, Akan>(x);
    }
}
