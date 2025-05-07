use crate::calendar::egyptian::*;

const ARMENIAN_EPOCH: f64 = 201443.0;

pub fn fixed_from_armenian(year: i32, month: u8, day: u8) -> f64 {
    ARMENIAN_EPOCH + fixed_from_egyptian(year, month, day) - EGYPTIAN_EPOCH
}

pub fn fixed_to_armenian(date: f64) -> (i32, u8, u8) {
    fixed_to_egyptian(date + EGYPTIAN_EPOCH - ARMENIAN_EPOCH)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn armenian_roundtrip() {
        let year0 = 747;
        let month0 = 6;
        let day0 = 29;
        let (year1, month1, day1) = fixed_to_armenian(fixed_from_armenian(year0, month0, day0));
        assert_eq!(year0, year1);
        assert_eq!(month0, month1);
        assert_eq!(day0, day1);
    }
}
