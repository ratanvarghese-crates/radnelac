// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::common::error::CalendarError;
use num_traits::AsPrimitive;
use num_traits::Bounded;
use num_traits::Euclid;
use num_traits::FromPrimitive;
use num_traits::NumAssign;
use num_traits::ToPrimitive;
use num_traits::Zero;
use std::cmp::PartialOrd;

// https://en.m.wikipedia.org/wiki/Double-precision_floating-point_format
// > Between 2^52=4,503,599,627,370,496 and 2^53=9,007,199,254,740,992 the
// > representable numbers are exactly the integers. For the next range,
// > from 2^53 to 2^54, everything is multiplied by 2, so the representable
// > numbers are the even ones, etc. Conversely, for the previous range from
// > 2^51 to 2^52, the spacing is 0.5, etc.
// >
// > The spacing as a fraction of the numbers in the range from 2^n to 2^n+1
// > is 2^n−52.

// We want to represent seconds as fractions of a day, and represent days
// since any calendar's epoch as whole numbers. We should avoid using floating
// point in the ranges where that would be inaccurate.
// 1/(24 * 60 * 60) = 0.000011574074074074073499346534
// 2 ** (36 - 52)   = 0.000015258789062500000000000000 (n = 36 is too large)
// 2 ** (35 - 52)   = 0.000007629394531250000000000000 (n = 35 is small, but risks off by 1 second)
// 2 ** (34 - 52)   = 0.000003814697265625000000000000 (n = 34 is probably small enough)
// 2 ** 34          = 17179869184
// 2 ** 35          = 34359738368
// 2 ** 36          = 68719476736
// Converted into years, it's still a lot of time:
// 2 ** 34 / 365.25 = 47035918.36824093

pub const EFFECTIVE_MAX: f64 = 17179869184.0;
pub const EFFECTIVE_MIN: f64 = -EFFECTIVE_MAX;
pub const EQ_SCALE: f64 = EFFECTIVE_MAX;
pub const EFFECTIVE_EPSILON: f64 = 0.000003814697265625;

pub trait TermNum:
    NumAssign
    + PartialOrd
    + ToPrimitive
    + FromPrimitive
    + AsPrimitive<f64>
    + AsPrimitive<i64>
    + AsPrimitive<Self>
    + Euclid
    + Bounded
    + Copy
{
    fn approx_eq(self, other: Self) -> bool {
        self == other
    }

    fn approx_floor(self) -> Self {
        self
    }

    fn floor_round(self) -> Self {
        self
    }

    fn is_a_number(self) -> bool {
        true
    }

    fn modulus(self, other: Self) -> Self {
        let x = self;
        let y = other;
        Euclid::rem_euclid(&x, &y)
    }

    fn approx_eq_iter<T: IntoIterator<Item = Self>>(x: T, y: T) -> bool {
        !x.into_iter()
            .zip(y.into_iter())
            .any(|(zx, zy)| !zx.approx_eq(zy))
    }

    fn sign(self) -> Self {
        if self.is_zero() {
            Self::zero()
        } else {
            Self::one()
        }
    }

    fn gcd(self, other: Self) -> Self {
        //LISTING 1.22 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        let x = self;
        let y = other;
        if y.is_zero() {
            x
        } else {
            y.gcd(x.modulus(y))
        }
    }

    fn lcm(self, other: Self) -> Self {
        //LISTING 1.23 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        let x = self;
        let y = other;
        (x * y) / x.gcd(y)
    }

    fn interval_modulus(self, a: Self, b: Self) -> Self {
        //LISTING 1.24 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        let x = self;
        if a == b {
            x
        } else {
            a + (x - a).modulus(b - a)
        }
    }

    fn adjusted_remainder(self, b: Self) -> Self {
        //LISTING 1.28 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        let x = self;
        let m = x.modulus(b);
        if m == Self::zero() {
            b
        } else {
            m
        }
    }

    fn sum<U: TermNum>(f: impl Fn(U) -> Self, p: impl Fn(U) -> bool, k: U) -> Self {
        //LISTING 1.30 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        //Modified to use iteration instead of recursion
        let mut result = Self::zero();
        let mut i = k;
        while p(i) {
            result += f(i);
            i += U::one();
        }
        result
    }

    fn product<U: TermNum>(f: impl Fn(U) -> Self, p: impl Fn(U) -> bool, k: U) -> Self {
        //LISTING 1.31 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        //Modified to user iteration instead of recursion
        let mut result = Self::one();
        let mut i = k;
        while p(i) {
            result *= f(i);
            i += U::one();
        }
        result
    }

    fn validate_mixed_radix(a: &[Self], b: &[Self]) -> Result<(), CalendarError> {
        if a.len() != (b.len() + 1) {
            Err(CalendarError::MixedRadixWrongSize)
        } else if b.iter().any(|&bx| bx.is_zero()) {
            Err(CalendarError::MixedRadixZeroBase)
        } else {
            Ok(())
        }
    }

    fn from_mixed_radix(a: &[Self], b: &[Self], k: usize) -> Result<f64, CalendarError> {
        //LISTING 1.41 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        let n = b.len();
        let as_f64 = <Self as AsPrimitive<f64>>::as_;
        match TermNum::validate_mixed_radix(a, b) {
            Ok(()) => (),
            Err(error) => return Err(error),
        };

        let sum_mul: f64 = TermNum::sum(
            |i| a[i] * TermNum::product(|j| b[j], |j| j < k, i),
            |i| i <= k,
            0,
        )
        .as_();

        let sum_div: f64 = TermNum::sum(
            |i| as_f64(a[i]) / as_f64(TermNum::product(|j| b[j], |j| j < i, k)),
            |i| i <= n,
            k + 1,
        );

        Ok(sum_mul + sum_div)
    }

    fn to_mixed_radix(x: f64, b: &[Self], k: usize, a: &mut [Self]) -> Result<(), CalendarError> {
        //LISTING 1.42 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        let n = b.len();
        match TermNum::validate_mixed_radix(a, b) {
            Ok(()) => (),
            Err(error) => return Err(error),
        };

        for i in 0..(n + 1) {
            if i == 0 {
                let p0: f64 = TermNum::product(|j| b[j], |j| j < k, 0).as_();
                let q0 = Self::from_f64((x / p0).approx_floor() as f64);
                match q0 {
                    Some(q) => a[i] = q,
                    None => return Err(CalendarError::ImpossibleResult),
                }
            } else if i > 0 && i < k {
                let p1: f64 = TermNum::product(|j| b[j], |j| j < k, i).as_();
                let q1 = Self::from_f64((x / p1).approx_floor() as f64);
                match q1 {
                    Some(q) => a[i] = q.modulus(b[i - 1]),
                    None => return Err(CalendarError::ImpossibleResult),
                }
            } else if i >= k && i < n {
                let p2: f64 = TermNum::product(|j| b[j], |j| j < i, k).as_();
                let q2 = Self::from_f64((x * p2).approx_floor() as f64);
                match q2 {
                    Some(q) => a[i] = q.modulus(b[i - 1]),
                    None => return Err(CalendarError::ImpossibleResult),
                }
            } else {
                let p3: f64 = TermNum::product(|j| b[j], |j| j < n, k).as_();
                let q3 = x * p3;
                let m = q3.modulus(b[n - 1].as_());
                if m.approx_eq(b[n - 1].as_()) || m.approx_eq(0.0) {
                    a[i] = Self::zero();
                } else if m.fract().approx_eq(1.0) {
                    a[i] = match Self::from_f64(m.ceil()) {
                        Some(m3) => m3,
                        None => return Err(CalendarError::ImpossibleResult),
                    };
                } else {
                    a[i] = match Self::from_f64(m) {
                        Some(m3) => m3,
                        None => return Err(CalendarError::ImpossibleResult),
                    };
                }
            }
        }
        Ok(())
    }

    fn search_min(p: impl Fn(Self) -> bool, k: Self) -> Self {
        //LISTING 1.32 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        //Modified to use iteration instead of recursion
        let mut i = k;
        while !p(i) {
            i += Self::one()
        }
        i
    }

    fn search_max(p: impl Fn(Self) -> bool, k: Self) -> Self {
        //LISTING 1.33 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        //Modified to use iteration instead of recursion
        let mut i = k - Self::one();
        while p(i) {
            i += Self::one()
        }
        i
    }
}

impl TermNum for usize {}
impl TermNum for u64 {}
impl TermNum for u32 {}
impl TermNum for u16 {}
impl TermNum for u8 {}

impl TermNum for i64 {
    fn sign(self) -> Self {
        if self.is_zero() {
            Self::zero()
        } else {
            self.signum()
        }
    }

    fn modulus(self, other: Self) -> Self {
        debug_assert!(other != Self::zero());
        if other > Self::zero() {
            let x = self;
            let y = other;
            Euclid::rem_euclid(&x, &y)
        } else {
            let x = -self;
            let y = -other;
            -Euclid::rem_euclid(&x, &y)
        }
    }
}

macro_rules! CastFn {
    ($n: ident, $t:ident) => {
        fn $n(self) -> Self {
            (self as $t).$n() as Self
        }
    };
    ($n: ident, $t:ident, $u: ident) => {
        fn $n(self, other: Self) -> $u {
            (self as $t).$n(other as $t) as $u
        }
    };
}

impl TermNum for i32 {
    CastFn!(sign, i64);
    CastFn!(modulus, i64, Self);
}

impl TermNum for i16 {
    CastFn!(sign, i64);
    CastFn!(modulus, i64, Self);
}

impl TermNum for i8 {
    CastFn!(sign, i64);
    CastFn!(modulus, i64, Self);
}

impl TermNum for f64 {
    fn sign(self) -> Self {
        //LISTING 1.16 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        //Modified to use `signum()`
        if self.is_zero() {
            Self::zero()
        } else {
            self.signum()
        }
    }

    fn approx_eq(self, other: Self) -> bool {
        let x = self;
        let y = other;
        if x == y {
            true
        } else if x.signum() != y.signum() && x != 0.0 && y != 0.0 {
            false
        } else {
            (x - y).abs() < (x.abs() / EQ_SCALE) || (x - y).abs() < EFFECTIVE_EPSILON
        }
    }

    fn approx_floor(self) -> Self {
        let x = self;
        let cx = x.ceil();
        if x.approx_eq(cx) {
            cx
        } else {
            x.floor()
        }
    }

    fn floor_round(self) -> Self {
        //LISTING 1.15 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        (self + 0.5).floor()
    }

    fn modulus(self, other: Self) -> Self {
        //LISTING 1.17 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        let x = self;
        let y = other;
        debug_assert!(y != 0.0); //Cannot replace with NonZero - it's not supported for f64
        if x == 0.0 {
            0.0
        } else {
            x - (y * (x / y).floor())
        }
    }

    fn is_a_number(self) -> bool {
        !self.is_nan()
    }
}

impl TermNum for f32 {
    CastFn!(sign, f64);
    CastFn!(modulus, f64, Self);
    CastFn!(approx_eq, f64, bool);
    CastFn!(approx_floor, f64);
    CastFn!(floor_round, f64);

    fn is_a_number(self) -> bool {
        !self.is_nan()
    }
}

// TODO: binary search (listing 1.35)
// TODO: inverse f (listing 1.36)
// TODO: list_of_fixed_from_moments (listing 1.37)
// TODO: range (1.38)
// TODO: scan_range (1.39)
// TODO: positions_in_range (1.40)
// TODO: angles, minutes, degrees

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prop_assume;
    use proptest::proptest;

    #[test]
    fn modulus_basics() {
        assert_eq!((9.0).modulus(5.0), 4.0);
        assert_eq!((-9.0).modulus(5.0), 1.0);
        assert_eq!((9.0).modulus(-5.0), -1.0);
        assert_eq!((-9.0).modulus(-5.0), -4.0);
    }

    #[test]
    fn adjusted_remainder() {
        assert_eq!((13).adjusted_remainder(12), 1);
    }

    #[test]
    #[should_panic]
    fn modulus_zero() {
        (123.0).modulus(0.0);
    }

    #[test]
    fn gcd_wikipedia_examples() {
        //See https://en.wikipedia.org/wiki/Greatest_common_divisor
        assert_eq!(8.0.gcd(12.0), 4.0);
        assert_eq!(54.0.gcd(24.0), 6.0);
        assert_eq!(9.0.gcd(28.0), 1.0); //Coprime
        assert_eq!(24.0.gcd(60.0), 12.0);
        assert_eq!(42.0.gcd(56.0), 14.0);
    }

    #[test]
    fn lcm_wikipedia_examples() {
        //https://en.wikipedia.org/wiki/Least_common_multiple
        assert_eq!((5.0).lcm(4.0), 20.0);
        assert_eq!((6.0).lcm(4.0), 12.0);
    }

    #[test]
    fn sum_of_2x() {
        let y = TermNum::sum(|x| x * 2.0, |i| i < 3.0, 1.0);
        assert_eq!(y, 3.0 * 2.0);
    }

    #[test]
    fn product_of_2x() {
        let y = TermNum::product(|x| x * 2.0, |i| i < 3.0, 1.0);
        assert_eq!(y, 2.0 * 4.0);
    }

    #[test]
    fn search_min_sign() {
        let y = f64::search_min(|i| i.sign() == 1.0, -10.0);
        assert_eq!(y, 1.0);
        let z = f64::search_min(|i| i.sign() == 1.0, 500.0);
        assert_eq!(z, 500.0);
    }

    #[test]
    fn search_max_sign() {
        let y = f64::search_max(|i| i.sign() == -1.0, -10.0);
        assert_eq!(y, 0.0);
        let z = f64::search_max(|i| i.sign() == -1.0, 500.0);
        assert_eq!(z, 499.0);
    }

    proptest! {
        #[test]
        fn mixed_radix_time(ahr in 0..24,amn in 0..59,asc in 0..59) {
            let ahr = ahr as f64;
            let amn = amn as f64;
            let asc = asc as f64;
            let a = [ahr, amn, asc];
            let b = [60.0, 60.0];
            let seconds = TermNum::from_mixed_radix(&a, &b, 2).unwrap();
            let minutes = TermNum::from_mixed_radix(&a, &b, 1).unwrap();
            let hours = TermNum::from_mixed_radix(&a, &b, 0).unwrap();
            let expected_seconds = (ahr * 3600.0) + (amn* 60.0) + asc;
            let expected_minutes = (ahr * 60.0) + amn + (asc / 60.0);
            let expected_hours = ahr + (amn / 60.0) + (asc / 3600.0);
            assert!(seconds.approx_eq(expected_seconds));
            assert!(minutes.approx_eq(expected_minutes));
            assert!(hours.approx_eq(expected_hours));

            let mut a_seconds = [0.0, 0.0, 0.0];
            let mut a_minutes = [0.0, 0.0, 0.0];
            let mut a_hours = [0.0, 0.0, 0.0];
            TermNum::to_mixed_radix(seconds, &b, 2, &mut a_seconds).unwrap();
            TermNum::to_mixed_radix(minutes, &b, 1, &mut a_minutes).unwrap();
            TermNum::to_mixed_radix(hours, &b, 0, &mut a_hours).unwrap();

            println!("a: {a:?}, a_hours: {a_hours:?}, hours: {hours}");

            assert!(TermNum::approx_eq_iter(a_seconds, a));
            assert!(TermNum::approx_eq_iter(a_minutes, a));
            assert!(TermNum::approx_eq_iter(a_hours, a));
        }

        #[test]
        fn mixed_radix_time_i(ahr in 0..24,amn in 0..59,asc in 0..59) {
            let ahr = ahr as i32;
            let amn = amn as i32;
            let asc = asc as i32;
            let a = [ahr, amn, asc];
            let b = [60, 60];
            let seconds = TermNum::from_mixed_radix(&a, &b, 2).unwrap();
            let minutes = TermNum::from_mixed_radix(&a, &b, 1).unwrap();
            let hours = TermNum::from_mixed_radix(&a, &b, 0).unwrap();

            let ahr = ahr as f64;
            let amn = amn as f64;
            let asc = asc as f64;
            let expected_seconds = (ahr * 3600.0) + (amn* 60.0) + asc;
            let expected_minutes = (ahr * 60.0) + amn + (asc / 60.0);
            let expected_hours = ahr + (amn / 60.0) + (asc / 3600.0);
            assert!(seconds.approx_eq(expected_seconds));
            assert!(minutes.approx_eq(expected_minutes));
            assert!(hours.approx_eq(expected_hours));

            let mut a_seconds = [0, 0, 0];
            let mut a_minutes = [0, 0, 0];
            let mut a_hours = [0, 0, 0];
            TermNum::to_mixed_radix(seconds, &b, 2, &mut a_seconds).unwrap();
            TermNum::to_mixed_radix(minutes, &b, 1, &mut a_minutes).unwrap();
            TermNum::to_mixed_radix(hours, &b, 0, &mut a_hours).unwrap();

            println!("a: {a:?}, a_hours: {a_hours:?}, hours: {hours}");

            assert_eq!(&a_seconds, &a);
            assert_eq!(&a_minutes, &a);
            assert_eq!(&a_hours, &a);
        }


        #[test]
        fn modulus_positivity(x in EFFECTIVE_MIN..EFFECTIVE_MAX, y in 0.0..EFFECTIVE_MAX) {
            assert!((x as f64).modulus(y as f64) >= 0.0);
        }

        #[test]
        fn modulus_i_positivity(x: i32, y in 1..i32::MAX) {
            assert!(x.modulus(y) >= 0);
        }


        #[test]
        fn modulus_negative_x(x in 0.0..EFFECTIVE_MAX, y in 0.0..EFFECTIVE_MAX) {
            prop_assume!(y != 0.0);
            prop_assume!(x.modulus(y) != 0.0);
            let a0 = (-x).modulus(y);
            let a1 = y - x.modulus(y);
            assert!(a0.approx_eq(a1));
        }

        #[test]
        fn modulus_i_negative_x(x in 0..i32::MAX, y in 1..i32::MAX) {
            prop_assume!(x.modulus(y) != 0);
            let a0 = (-x).modulus(y);
            let a1 = y - x.modulus(y);
            assert_eq!(a0, a1);
        }

        #[test]
        fn modulus_mult(
            x in -131072.0..131072.0,
            y in -131072.0..131072.0,
            z in -131072.0..131072.0) {
            //LISTING 1.19 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
            //Using sqrt(EFFECTIVE_MAX) as limit
            let x = x as f64;
            let y = y as f64;
            let z = z as f64;
            prop_assume!(y != 0.0);
            prop_assume!(z != 0.0);
            let a: f64 = x.modulus(y);
            let az: f64 = (x*z).modulus(y*z);
            println!("x={}; y={}; z={}; a={}; a*z= {}; az= {};", x, y, z, a, a*z, az);
            assert!((a * z).approx_eq(az));
        }

        #[test]
        fn modulus_i_mult(
            x in i16::MIN..i16::MAX,
            y in i16::MIN..i16::MAX,
            z in i16::MIN..i16::MAX) {
            //LISTING 1.19 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
            let x = x as i32;
            let y = y as i32;
            let z = z as i32;
            prop_assume!(y != 0);
            prop_assume!(z != 0);
            let a = x.modulus(y);
            let az = (x*z).modulus(y*z);
            println!("x={}; y={}; z={}; a={}; a*z= {}; az= {};", x, y, z, a, a*z, az);
            assert_eq!(a * z, az);
        }

        #[test]
        fn modulus_mult_minus_1(x in 0.0..EFFECTIVE_MAX, y in 0.0..EFFECTIVE_MAX) {
            //LISTING 1.19 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
            prop_assume!(y != 0.0);
            let a0 = (-x).modulus(-y);
            let a1 = -(x.modulus(y));
            assert_eq!(a0, a1);
        }

        #[test]
        fn modulus_i_mult_minus_1(x in 0..i32::MAX, y in 1..i32::MAX) {
            //LISTING 1.19 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
            let a0 = (-x).modulus(-y);
            let a1 = -(x.modulus(y));
            assert_eq!(a0, a1);
        }

        #[test]
        fn modulus_i_multiple_of_y(x: i32, y: i32) {
            //LISTING 1.20 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
            prop_assume!(y != 0);
            let a = (x as i64) - (x.modulus(y) as i64);
            assert_eq!(a % (y as i64), 0);
        }

        #[test]
        fn modulus_bounds(x in EFFECTIVE_MIN..EFFECTIVE_MAX, y in EFFECTIVE_MIN..EFFECTIVE_MAX) {
            //LISTING 1.21 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
            prop_assume!(y != 0.0);
            let a = x.modulus(y) * y.sign();
            assert!(0.0 <= a && a < y.abs());
        }
        #[test]
        fn modulus_i_bounds(x: i32, y: i32) {
            //LISTING 1.21 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
            prop_assume!(y != 0);
            let a = x.modulus(y) * (y.sign());
            assert!(0 <= a && a < y.abs());
        }
    }
}
