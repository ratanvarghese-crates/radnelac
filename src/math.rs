use std::ops::AddAssign;
use std::ops::MulAssign;
use std::ops::Sub;

const EFFECTIVE_MAX: f64 = 9007199254740992.0;
const EFFECTIVE_MIN: f64 = f32::EPSILON as f64;

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

pub fn modulus(x: f64, y: f64) -> f64 {
    if y.abs() < EFFECTIVE_MIN {
        panic!("modulus(x, y) where y is almost 0")
    }
    if y.abs() > EFFECTIVE_MAX {
        panic!("y too large")
    }
    if x.abs() > EFFECTIVE_MAX {
        panic!("x too large")
    }
    if x == 0.0 {
        0.0
    } else {
        x - (y * (x / y).floor())
    }
}

pub fn gcd(x: f64, y: f64) -> f64 {
    if y == 0.0 {
        x
    } else {
        gcd(y, modulus(x, y))
    }
}

pub fn lcm(x: f64, y: f64) -> f64 {
    (x * y) / gcd(x, y)
}

pub fn interval_modulus(x: f64, a: f64, b: f64) -> f64 {
    if a == b {
        x
    } else {
        a + modulus(x - a, b - a)
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

pub fn from_mixed_radix(a: &[f64], b: &[f64], k: usize) -> f64 {
    let n = b.len();

    if a.len() != (n + 1) {
        panic!("Forbidden mixed radix number size");
    }

    let sum_mul = sum(|i| a[i] * product(|j| b[j], |j| j < k, i), |i| i <= k, 0);

    let sum_div = sum(
        |i| a[i] / product(|j| b[j], |j| j < i, k),
        |i| i <= n,
        k + 1,
    );

    sum_mul + sum_div
}

pub fn to_mixed_radix(x: f64, b: &[f64], k: usize) -> Vec<f64> {
    let mut a: Vec<f64> = Vec::new();
    let n = b.len();
    let x = x;

    for i in 0..(n + 1) {
        if i == 0 {
            let p0 = product(|j| b[j], |j| j < k, 0);
            a.push((x / p0).floor());
        } else if i > 0 && i < k {
            let p1 = product(|j| b[j], |j| j < k, i);
            a.push(modulus((x / p1).floor(), b[i - 1]));
        } else if i >= k && i < n {
            let p2 = product(|j| b[j], |j| j < i, k);
            a.push(modulus((x * p2).floor(), b[i - 1]));
        } else {
            let p3 = product(|j| b[j], |j| j < n, k);
            a.push(modulus(x * p3, b[n - 1]));
        }
    }

    a
}

//TODO: angles, minutes, degrees

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prop_assume;
    use proptest::proptest;

    #[test]
    fn modulus_basics() {
        assert_eq!(modulus(9.0, 5.0), 4.0);
        assert_eq!(modulus(-9.0, 5.0), 1.0);
        assert_eq!(modulus(9.0, -5.0), -1.0);
        assert_eq!(modulus(-9.0, -5.0), -4.0);
        //TODO: revisit chapter 1.7 for modulus properties
    }

    #[test]
    #[should_panic(expected = "modulus(x, y) where y is almost 0")]
    fn modulus_zero() {
        modulus(123.0, 0.0);
    }

    #[test]
    fn gcd_wikipedia_examples() {
        //See https://en.wikipedia.org/wiki/Greatest_common_divisor
        assert_eq!(gcd(8.0, 12.0), 4.0);
        assert_eq!(gcd(54.0, 24.0), 6.0);
        assert_eq!(gcd(9.0, 28.0), 1.0); //Coprime
        assert_eq!(gcd(24.0, 60.0), 12.0);
        assert_eq!(gcd(42.0, 56.0), 14.0);
    }

    #[test]
    fn lcm_wikipedia_examples() {
        //https://en.wikipedia.org/wiki/Least_common_multiple
        assert_eq!(lcm(5.0, 4.0), 20.0);
        assert_eq!(lcm(6.0, 4.0), 12.0);
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

    #[test]
    fn mixed_radix_time() {
        let a = [1.0, 21.0, 14.0];
        let b = [60.0, 60.0];
        let seconds = from_mixed_radix(&a, &b, 2);
        let minutes = from_mixed_radix(&a, &b, 1);
        let hours = from_mixed_radix(&a, &b, 0);
        assert_eq!(seconds, (1.0 * 3600.0) + (21.0 * 60.0) + 14.0);
        assert_eq!(minutes, (1.0 * 60.0) + 21.0 + (14.0 / 60.0));
        assert_eq!(hours, 1.0 + (21.0 / 60.0) + (14.0 / 3600.0));

        let a_seconds = to_mixed_radix(seconds, &b, 2);
        let a_minutes = to_mixed_radix(minutes, &b, 1);
        let a_hours = to_mixed_radix(hours, &b, 0);

        println!("{seconds}, {minutes}, {hours}");

        assert_eq!(a_seconds, a);
        assert_eq!(a_minutes, a);
        assert_eq!(a_hours, a);
    }

    proptest! {
        #[test]
        fn modulus_positivity(x in -EFFECTIVE_MAX..EFFECTIVE_MAX, y in EFFECTIVE_MIN..EFFECTIVE_MAX) {
            assert!(modulus(x as f64, y as f64) >= 0.0);
        }

        #[test]
        fn modulus_negative_x(x in EFFECTIVE_MIN..EFFECTIVE_MAX, y in EFFECTIVE_MIN..EFFECTIVE_MAX) {
            prop_assume!(y.abs() > EFFECTIVE_MIN);
            prop_assume!(modulus(x as f64,y as f64) != 0.0);
            let a0 = modulus(-x as f64, y as f64);
            let a1 = y as f64 - modulus(x as f64, y as f64);
            let diff = a0 - a1;
            assert!(diff == 0.0 || diff.abs() < a0/1e12);
        }

        #[test]
        fn modulus_mult(
            x in -94906265.0..94906265.0,
            y in -94906265.0..94906265.0,
            z in -94906265.0..94906265.0) {
            //Using sqrt(EFFECTIVE_MAX) as limit
            prop_assume!((y as f64).abs() > EFFECTIVE_MIN);
            prop_assume!((z as f64).abs() > EFFECTIVE_MIN);
            let a = modulus(x, y);
            let az = modulus(x*z, y*z);
            let diff = (a * z) - az;
            assert!(diff == 0.0 || (diff as f64).abs() < az.abs()/1e12);
        }

        #[test]
        fn modulus_mult_minus_1(x in -94906265.0..94906265.0, y in -94906265.0..94906265.0) {
            prop_assume!((y as f64).abs() > EFFECTIVE_MIN);
            let a0 = modulus(-(x as f64), -(y as f64));
            let a1 = -modulus(x, y);
            assert_eq!(a0, a1);
        }

        #[test]
        fn modulus_int_multiple_of_y(x in -94906265..94906265, y in -94906265..94906265) {
            let x = x as i32;
            let y = y as i32;
            prop_assume!(y != 0);
            let a = x - modulus(x as f64, y as f64) as i32;
            assert_eq!(a % y, 0);
        }

        #[test]
        fn modulus_bounds(x in -EFFECTIVE_MAX..EFFECTIVE_MAX, y in -EFFECTIVE_MAX..EFFECTIVE_MAX) {
            prop_assume!(y.abs() > EFFECTIVE_MIN);
            let a = modulus(x, y) * sign(y);
            assert!(0.0 <= a && a < y.abs());
        }
    }
}
