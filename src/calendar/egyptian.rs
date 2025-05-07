use crate::epoch::moment_from_jd;
use crate::math::modulus;

pub const EGYPTIAN_EPOCH: f64 = moment_from_jd(1448638.0) - 0.5; //Nabonassar era

pub fn fixed_from_egyptian(year: i32, month: u8, day: u8) -> f64 {
    let year = year as f64;
    let month = month as f64;
    let day = day as f64;
    EGYPTIAN_EPOCH + (365.0 * (year - 1.0)) + (30.0 * (month - 1.0)) + day - 1.0
}

pub fn fixed_to_egyptian(date: f64) -> (i32, u8, u8) {
    let days = date - EGYPTIAN_EPOCH;
    let year = (days / 365.0).floor() + 1.0;
    let month = (modulus(days, 365.0) / 30.0).floor() + 1.0;
    let day = days - (365.0 * (year - 1.0)) - (30.0 * (month - 1.0)) + 1.0;
    (year as i32, month as u8, day as u8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn egyptian_roundtrip() {
        let year0 = 747;
        let month0 = 6;
        let day0 = 29;
        let (year1, month1, day1) = fixed_to_egyptian(fixed_from_egyptian(year0, month0, day0));
        assert_eq!(year0, year1);
        assert_eq!(month0, month1);
        assert_eq!(day0, day1);
    }
}
