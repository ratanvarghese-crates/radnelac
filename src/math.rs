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
    if y == 0.0 {
        panic!("Forbidden to call modulus(x, 0)")
    }
    x - (y * (x / y).floor())
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

pub fn sum(f: impl Fn(f64) -> f64, p: impl Fn(f64) -> bool, k: f64) -> f64 {
    let mut result: f64 = 0.0;
    let mut i = k;
    while p(i) {
        result += f(i);
        i += 1.0;
    }
    result
}

pub fn product(f: impl Fn(f64) -> f64, p: impl Fn(f64) -> bool, k: f64) -> f64 {
    let mut result: f64 = 1.0;
    let mut i = k;
    while p(i) {
        result *= f(i);
        i += 1.0;
    }
    result
}

pub fn search_min(p: impl Fn(f64) -> bool, k: f64) -> f64 {
    let mut i = k;
    while !p(i) {
        i += 1.0
    }
    i
}

pub fn search_max(p: fn(f64) -> bool, k: f64) -> f64 {
    let mut i = k - 1.0;
    while p(i) {
        i += 1.0
    }
    i
}

// TODO: binary search (listing 1.35)
// TODO: inverse f (listing 1.36)
// TODO: list_of_fixed_from_moments (listing 1.37)
// TODO: range (1.38)
// TODO: scan_range (1.39)
// TODO: positions_in_range (1.40)

pub fn from_mixed_radix(a: &[f64], b: &[f64], k: f64) -> f64 {
    let n = b.len() as f64;

    if (a.len() as f64) != (n + 1.0) {
        panic!("Forbidden mixed radix number size");
    }

    let sum_mul = sum(
        |i| a[i as usize] * product(|j| b[j as usize], |j| j < k, i),
        |i| i <= k,
        0.0,
    );

    let sum_div = sum(
        |i| a[i as usize] / product(|j| b[j as usize], |j| j < i, k),
        |i| i <= n,
        k + 1.0,
    );

    return sum_mul + sum_div;
}

pub fn to_mixed_radix(x: f64, b: &[f64], k: f64) -> Vec<f64> {
    let mut a: Vec<f64> = Vec::new();
    let n = b.len();

    for i in 0..(n + 1) {
        if i == 0 {
            let p0 = product(|j| b[j as usize], |j| j < k, 0.0);
            a.push((x / p0).floor());
        } else if i > 0 && i < (k as usize) {
            let p1 = product(|j| b[j as usize], |j| j < k, i as f64);
            a.push(modulus((x / p1).floor(), b[i - 1]));
        } else if i >= (k as usize) && i < n {
            let p2 = product(|j| b[j as usize], |j| j < i as f64, k as f64);
            a.push(modulus((x * p2).floor(), b[i - 1]));
        } else {
            let p3 = product(|j| b[j as usize], |j| j < n as f64, k as f64);
            a.push(modulus(x * p3, b[n - 1]));
        }
    }

    return a;
}

//TODO: angles, minutes, degrees

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modulus_basics() {
        assert_eq!(modulus(9.0, 5.0), 4.0);
        assert_eq!(modulus(-9.0, 5.0), 1.0);
        assert_eq!(modulus(9.0, -5.0), -1.0);
        assert_eq!(modulus(-9.0, -5.0), -4.0);
        //TODO: revisit chapter 1.7 for modulus properties
    }

    #[test]
    #[should_panic(expected = "Forbidden to call modulus(x, 0)")]
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
        let seconds = from_mixed_radix(&a, &b, 2.0);
        let minutes = from_mixed_radix(&a, &b, 1.0);
        let hours = from_mixed_radix(&a, &b, 0.0);
        assert_eq!(seconds, (1.0 * 3600.0) + (21.0 * 60.0) + 14.0);
        assert_eq!(minutes, (1.0 * 60.0) + 21.0 + (14.0 / 60.0));
        assert_eq!(hours, 1.0 + (21.0 / 60.0) + (14.0 / 3600.0));

        let a_seconds = to_mixed_radix(seconds, &b, 2.0);
        let a_minutes = to_mixed_radix(minutes, &b, 1.0);
        let a_hours = to_mixed_radix(hours, &b, 0.0);

        println!("{seconds}, {minutes}, {hours}");

        assert_eq!(a_seconds, a);
        assert_eq!(a_minutes, a);
        assert_eq!(a_hours, a);
    }
}
