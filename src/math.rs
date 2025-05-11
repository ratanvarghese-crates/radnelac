use crate::error::CalendarError;
use std::num::NonZero;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Sub;

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
// 2 ** (35 - 52)   = 0.000007629394531250000000000000 (n = 35 is small enough)
// 2 ** 35          = 34359738368
// 2 ** 36          = 68719476736
// That is a lot of days, converted into years:
// 2 ** 36 / 365.25 = 188143673.47296372

pub const EFFECTIVE_MAX: f64 = 68719476736.0;
pub const EFFECTIVE_MIN: f64 = 0.00000762939453125;
pub const EQ_SCALE: f64 = EFFECTIVE_MAX;

pub fn approx_eq(x: f64, y: f64) -> bool {
    if x == y {
        true
    } else if (x > 0.0 && y < 0.0) || (x < 0.0 && y > 0.0) {
        false
    } else {
        (x - y).abs() < (x.abs() / EQ_SCALE) || (x - y).abs() < EFFECTIVE_MIN
    }
}

fn approx_eq_slice(x: &[f64], y: &[f64]) -> bool {
    if x.len() != y.len() {
        return false;
    }

    for i in 0..x.len() {
        if !approx_eq(x[i], y[i]) {
            return false;
        }
    }
    true
}

fn floor_unless_scraping_ceiling(x: f64) -> f64 {
    let cx = x.ceil();
    if approx_eq(x, cx) {
        cx
    } else {
        x.floor()
    }
}

pub fn round(x: f64) -> f64 {
    (x + 0.5).floor()
}

pub fn sign(y: f64) -> f64 {
    if y == 0.0 {
        0.0
    } else {
        y.signum()
    }
}

pub fn modulus(x: f64, y: f64) -> Result<f64, CalendarError> {
    if y == 0.0 {
        Err(CalendarError::DivisionByZero)
    } else if y.abs() > EFFECTIVE_MAX || x.abs() > EFFECTIVE_MAX {
        Err(CalendarError::OutOfBounds)
    } else if x == 0.0 {
        Ok(0.0)
    } else {
        Ok(x - (y * (x / y).floor()))
    }
}

pub fn modulus_i(x: i32, y: NonZero<i32>) -> i32 {
    modulus(x as f64, y.get() as f64).expect("Nonzero i32 is safe for modulus") as i32
}

pub fn gcd(x: f64, y: f64) -> Result<f64, CalendarError> {
    if y == 0.0 {
        Ok(x)
    } else {
        gcd(y, modulus(x, y)?)
    }
}

pub fn lcm(x: f64, y: f64) -> Result<f64, CalendarError> {
    Ok((x * y) / gcd(x, y)?)
}

pub fn interval_modulus(x: f64, a: f64, b: f64) -> Result<f64, CalendarError> {
    if a == b {
        Ok(x)
    } else {
        Ok(a + modulus(x - a, b - a)?)
    }
}

pub fn sum<T, U>(f: impl Fn(U) -> T, p: impl Fn(U) -> bool, k: U) -> T
where
    T: AddAssign + From<u8> + Copy,
    U: AddAssign + From<u8> + Copy,
{
    let mut result: T = T::from(0);
    let mut i = k;
    while p(i) {
        result += f(i);
        i += U::from(1);
    }
    result
}

pub fn product<T, U>(f: impl Fn(U) -> T, p: impl Fn(U) -> bool, k: U) -> T
where
    T: MulAssign + From<u8> + Copy,
    U: AddAssign + From<u8> + Copy,
{
    let mut result: T = T::from(1);
    let mut i = k;
    while p(i) {
        result *= f(i);
        i += U::from(1);
    }
    result
}

pub fn search_min<T>(p: impl Fn(T) -> bool, k: T) -> T
where
    T: AddAssign + From<u8> + Copy,
{
    let mut i = k;
    while !p(i) {
        i += T::from(1)
    }
    i
}

pub fn search_max<T>(p: fn(T) -> bool, k: T) -> T
where
    T: AddAssign + Sub<T, Output = T> + From<u8> + Copy,
{
    let mut i = k - T::from(1);
    while p(i) {
        i += T::from(1)
    }
    i
}

// TODO: binary search (listing 1.35)
// TODO: inverse f (listing 1.36)
// TODO: list_of_fixed_from_moments (listing 1.37)
// TODO: range (1.38)
// TODO: scan_range (1.39)
// TODO: positions_in_range (1.40)

fn validate_mixed_radix<T: Into<f64> + Copy>(a: &[T], b: &[T]) -> Result<(), CalendarError> {
    if a.len() != (b.len() + 1) {
        Err(CalendarError::MixedRadixWrongSize)
    } else if b.iter().any(|&bx| bx.into() == 0.0) {
        Err(CalendarError::MixedRadixZeroBase)
    } else {
        Ok(())
    }
}

pub fn from_mixed_radix<T>(a: &[T], b: &[T], k: usize) -> Result<f64, CalendarError>
where
    T: Mul<Output = T> + Div<Output = T> + MulAssign + AddAssign + Into<f64> + From<u8> + Copy,
{
    let n = b.len();
    match validate_mixed_radix(a, b) {
        Ok(()) => (),
        Err(error) => return Err(error),
    };

    let sum_mul: f64 = sum(|i| a[i] * product(|j| b[j], |j| j < k, i), |i| i <= k, 0).into();

    let sum_div: f64 = sum(
        |i| (a[i].into() as f64) / ((product(|j| b[j], |j| j < i, k)).into() as f64),
        |i| i <= n,
        k + 1,
    )
    .into();

    Ok(sum_mul + sum_div)
}

pub fn to_mixed_radix(x: f64, b: &[f64], k: usize, a: &mut [f64]) -> Result<(), CalendarError> {
    let n = b.len();
    match validate_mixed_radix(a, b) {
        Ok(()) => (),
        Err(error) => return Err(error),
    };

    for i in 0..(n + 1) {
        if i == 0 {
            let p0 = product(|j| b[j], |j| j < k, 0);
            a[i] = floor_unless_scraping_ceiling(x / p0);
        } else if i > 0 && i < k {
            let p1 = product(|j| b[j], |j| j < k, i);
            a[i] = modulus(floor_unless_scraping_ceiling(x / p1), b[i - 1])?;
        } else if i >= k && i < n {
            let p2 = product(|j| b[j], |j| j < i, k);
            a[i] = modulus(floor_unless_scraping_ceiling(x * p2), b[i - 1])?;
        } else {
            let p3 = product(|j| b[j], |j| j < n, k);
            let m = modulus(x * p3, b[n - 1])?;
            if approx_eq(m, b[n - 1]) || approx_eq(m, 0.0) {
                a[i] = 0.0;
            } else {
                a[i] = m;
            }
        }
    }
    Ok(())
}

pub fn to_mixed_radix_i(x: f64, b: &[i32], k: usize, a: &mut [i32]) -> Result<(), CalendarError> {
    let n = b.len();
    match validate_mixed_radix(a, b) {
        Ok(()) => (),
        Err(error) => return Err(error),
    };

    for i in 0..(n + 1) {
        if i == 0 {
            let p0 = product(|j| b[j], |j| j < k, 0) as f64;
            a[i] = floor_unless_scraping_ceiling(x / p0) as i32;
        } else if i > 0 && i < k {
            let p1 = product(|j| b[j], |j| j < k, i) as f64;
            a[i] = modulus_i(
                floor_unless_scraping_ceiling(x / p1) as i32,
                NonZero::new(b[i - 1]).expect("Checked for 0 earlier"),
            );
        } else if i >= k && i < n {
            let p2 = product(|j| b[j], |j| j < i, k) as f64;
            a[i] = modulus_i(
                floor_unless_scraping_ceiling(x * p2) as i32,
                NonZero::new(b[i - 1]).expect("Checked for 0 earlier"),
            );
        } else {
            let p3 = product(|j| b[j], |j| j < n, k) as f64;
            a[i] = modulus_i(
                (x * p3).round() as i32, //A deviation from the formula
                NonZero::new(b[i - 1]).expect("Checked for 0 earlier"),
            );
        }
    }
    Ok(())
}

//TODO: angles, minutes, degrees

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prop_assume;
    use proptest::proptest;

    #[test]
    fn modulus_basics() {
        assert_eq!(modulus(9.0, 5.0).unwrap(), 4.0);
        assert_eq!(modulus(-9.0, 5.0).unwrap(), 1.0);
        assert_eq!(modulus(9.0, -5.0).unwrap(), -1.0);
        assert_eq!(modulus(-9.0, -5.0).unwrap(), -4.0);
    }

    #[test]
    #[should_panic]
    fn modulus_zero() {
        modulus(123.0, 0.0).unwrap();
    }

    #[test]
    fn gcd_wikipedia_examples() {
        //See https://en.wikipedia.org/wiki/Greatest_common_divisor
        assert_eq!(gcd(8.0, 12.0).unwrap(), 4.0);
        assert_eq!(gcd(54.0, 24.0).unwrap(), 6.0);
        assert_eq!(gcd(9.0, 28.0).unwrap(), 1.0); //Coprime
        assert_eq!(gcd(24.0, 60.0).unwrap(), 12.0);
        assert_eq!(gcd(42.0, 56.0).unwrap(), 14.0);
    }

    #[test]
    fn lcm_wikipedia_examples() {
        //https://en.wikipedia.org/wiki/Least_common_multiple
        assert_eq!(lcm(5.0, 4.0).unwrap(), 20.0);
        assert_eq!(lcm(6.0, 4.0).unwrap(), 12.0);
    }

    #[test]
    fn sum_of_2x() {
        let y = sum(|x| x * 2.0, |i| i < 3.0, 1.0);
        assert_eq!(y, 3.0 * 2.0);
    }

    #[test]
    fn product_of_2x() {
        let y = product(|x| x * 2.0, |i| i < 3.0, 1.0);
        assert_eq!(y, 2.0 * 4.0);
    }

    #[test]
    fn search_min_sign() {
        let y = search_min(|i| sign(i) == 1.0, -10.0);
        assert_eq!(y, 1.0);
        let z = search_min(|i| sign(i) == 1.0, 500.0);
        assert_eq!(z, 500.0);
    }

    #[test]
    fn search_max_sign() {
        let y = search_max(|i| sign(i) == -1.0, -10.0);
        assert_eq!(y, 0.0);
        let z = search_max(|i| sign(i) == -1.0, 500.0);
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
            let seconds = from_mixed_radix(&a, &b, 2).unwrap();
            let minutes = from_mixed_radix(&a, &b, 1).unwrap();
            let hours = from_mixed_radix(&a, &b, 0).unwrap();
            let expected_seconds = (ahr * 3600.0) + (amn* 60.0) + asc;
            let expected_minutes = (ahr * 60.0) + amn + (asc / 60.0);
            let expected_hours = ahr + (amn / 60.0) + (asc / 3600.0);
            assert!(approx_eq(seconds, expected_seconds));
            assert!(approx_eq(minutes, expected_minutes));
            assert!(approx_eq(hours, expected_hours));

            let mut a_seconds = [0.0, 0.0, 0.0];
            let mut a_minutes = [0.0, 0.0, 0.0];
            let mut a_hours = [0.0, 0.0, 0.0];
            to_mixed_radix(seconds, &b, 2, &mut a_seconds).unwrap();
            to_mixed_radix(minutes, &b, 1, &mut a_minutes).unwrap();
            to_mixed_radix(hours, &b, 0, &mut a_hours).unwrap();

            println!("a: {a:?}, a_hours: {a_hours:?}, hours: {hours}");

            assert!(approx_eq_slice(&a_seconds, &a));
            assert!(approx_eq_slice(&a_minutes, &a));
            assert!(approx_eq_slice(&a_hours, &a));
        }

        #[test]
        fn mixed_radix_time_i(ahr in 0..24,amn in 0..59,asc in 0..59) {
            let ahr = ahr as i32;
            let amn = amn as i32;
            let asc = asc as i32;
            let a = [ahr, amn, asc];
            let b = [60, 60];
            let seconds = from_mixed_radix(&a, &b, 2).unwrap();
            let minutes = from_mixed_radix(&a, &b, 1).unwrap();
            let hours = from_mixed_radix(&a, &b, 0).unwrap();

            let ahr = ahr as f64;
            let amn = amn as f64;
            let asc = asc as f64;
            let expected_seconds = (ahr * 3600.0) + (amn* 60.0) + asc;
            let expected_minutes = (ahr * 60.0) + amn + (asc / 60.0);
            let expected_hours = ahr + (amn / 60.0) + (asc / 3600.0);
            assert!(approx_eq(seconds, expected_seconds));
            assert!(approx_eq(minutes, expected_minutes));
            assert!(approx_eq(hours, expected_hours));

            let mut a_seconds = [0, 0, 0];
            let mut a_minutes = [0, 0, 0];
            let mut a_hours = [0, 0, 0];
            to_mixed_radix_i(seconds, &b, 2, &mut a_seconds).unwrap();
            to_mixed_radix_i(minutes, &b, 1, &mut a_minutes).unwrap();
            to_mixed_radix_i(hours, &b, 0, &mut a_hours).unwrap();

            println!("a: {a:?}, a_hours: {a_hours:?}, hours: {hours}");

            assert_eq!(&a_seconds, &a);
            assert_eq!(&a_minutes, &a);
            assert_eq!(&a_hours, &a);
        }


        #[test]
        fn modulus_positivity(x in -EFFECTIVE_MAX..EFFECTIVE_MAX, y in 0.0..EFFECTIVE_MAX) {
            assert!(modulus(x as f64, y as f64).unwrap() >= 0.0);
        }

        #[test]
        fn modulus_i_positivity(x: i32, y in 1..i32::MAX) {
            assert!(modulus_i(x, NonZero::new(y).unwrap()) >= 0);
        }


        #[test]
        fn modulus_negative_x(x in 0.0..EFFECTIVE_MAX, y in 0.0..EFFECTIVE_MAX) {
            prop_assume!(y != 0.0);
            prop_assume!(modulus(x as f64,y as f64).unwrap() != 0.0);
            let a0 = modulus(-x as f64, y as f64).unwrap();
            let a1 = y as f64 - modulus(x as f64, y as f64).unwrap();
            assert!(approx_eq(a0, a1));
        }

        #[test]
        fn modulus_i_negative_x(x in 0..i32::MAX, y in 1..i32::MAX) {
            let y = NonZero::new(y).unwrap();
            prop_assume!(modulus_i(x,y) != 0);
            let a0 = modulus_i(-x, y);
            let a1 = y.get() - modulus_i(x, y);
            assert_eq!(a0, a1);
        }


        #[test]
        fn modulus_mult(
            x in -262144.0..262144.0,
            y in -262144.0..262144.0,
            z in -262144.0..262144.0) {
            //Using sqrt(EFFECTIVE_MAX) as limit
            let x = x as f64;
            let y = y as f64;
            let z = z as f64;
            prop_assume!(y != 0.0);
            prop_assume!(z != 0.0);
            let a: f64 = modulus(x, y).unwrap();
            let az: f64 = modulus(x*z, y*z).unwrap();
            println!("x={}; y={}; z={}; a={}; a*z= {}; az= {};", x, y, z, a, a*z, az);
            assert!(approx_eq(a * z, az));
        }

        #[test]
        fn modulus_i_mult(
            x in i16::MIN..i16::MAX,
            y in i16::MIN..i16::MAX,
            z in i16::MIN..i16::MAX) {
            let x = x as i32;
            let y = y as i32;
            let z = z as i32;
            prop_assume!(y != 0);
            prop_assume!(z != 0);
            let nzyz = NonZero::new(y*z).unwrap();
            let nzy = NonZero::new(y).unwrap();
            let a = modulus_i(x, nzy);
            let az = modulus_i(x*z, nzyz);
            println!("x={}; y={}; z={}; a={}; a*z= {}; az= {};", x, y, z, a, a*z, az);
            assert_eq!(a * z, az);
        }

        #[test]
        fn modulus_mult_minus_1(x in 0.0..EFFECTIVE_MAX, y in 0.0..EFFECTIVE_MAX) {
            prop_assume!(y != 0.0);
            let a0 = modulus(-(x as f64), -(y as f64)).unwrap();
            let a1 = -modulus(x, y).unwrap();
            assert_eq!(a0, a1);
        }

        #[test]
        fn modulus_i_mult_minus_1(x in 0..i32::MAX, y in 1..i32::MAX) {
            let a0 = modulus_i(-x, NonZero::new(-y).unwrap());
            let a1 = -modulus_i(x, NonZero::new(y).unwrap());
            assert_eq!(a0, a1);
        }

        #[test]
        fn modulus_i_multiple_of_y(x: i32, y: i32) {
            prop_assume!(y != 0);
            let a = (x as i64) - (modulus_i(x, NonZero::new(y).unwrap()) as i64);
            assert_eq!(a % (y as i64), 0);
        }

        #[test]
        fn modulus_bounds(x in -EFFECTIVE_MAX..EFFECTIVE_MAX, y in -EFFECTIVE_MAX..EFFECTIVE_MAX) {
            prop_assume!(y != 0.0);
            let a = modulus(x, y).unwrap() * sign(y);
            assert!(0.0 <= a && a < y.abs());
        }
        #[test]
        fn modulus_i_bounds(x: i32, y: i32) {
            prop_assume!(y != 0);
            let a = modulus_i(x, NonZero::new(y).unwrap()) * (sign(y as f64) as i32);
            assert!(0 <= a && a < y.abs());
        }
    }
}
