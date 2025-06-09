use crate::calendar::julian::Julian;
use crate::common::bound::BoundedDayCount;
use crate::common::date::CommonDate;
use crate::common::date::CommonDay;
use crate::common::date::CommonYear;
use crate::common::date::GuaranteedMonth;
use crate::common::date::HasLeapYears;
use crate::common::date::ToFromCommonDate;
use crate::common::error::CalendarError;
use crate::common::math::TermNum;
use crate::day_count::CalculatedBounds;
use crate::day_count::Epoch;
use crate::day_count::Fixed;
use crate::day_count::FromFixed;
use crate::day_count::ToFixed;
#[allow(unused_imports)] //FromPrimitive is needed for derive
use num_traits::FromPrimitive;

const COPTIC_EPOCH_JULIAN: CommonDate = CommonDate {
    year: 284,
    month: 8,
    day: 29,
};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, FromPrimitive)]
pub enum CopticMonth {
    Thoout = 1,
    Paope,
    Athor,
    Koiak,
    Tobe,
    Meshir,
    Paremotep,
    Parmoute,
    Pashons,
    Paone,
    Epep,
    Mesore,
    Epagomene,
}

impl CopticMonth {
    pub fn length(self, leap: bool) -> u8 {
        match self {
            CopticMonth::Epagomene => {
                if leap {
                    6
                } else {
                    5
                }
            }
            _ => 30,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Coptic(CommonDate);

impl HasLeapYears for Coptic {
    fn is_leap(year: i32) -> bool {
        year.modulus(4) == 3
    }
}

impl CalculatedBounds for Coptic {}

impl Epoch for Coptic {
    fn epoch() -> Fixed {
        Julian::try_from_common_date(COPTIC_EPOCH_JULIAN)
            .expect("Epoch known to be in range.")
            .to_fixed()
    }
}

impl FromFixed for Coptic {
    fn from_fixed(fixed_date: Fixed) -> Coptic {
        let date = fixed_date.get_day_i();
        let epoch = Coptic::epoch().get_day_i();
        let year = (4 * (date - epoch) + 1463).div_euclid(1461) as i32;
        let year_start = Coptic::to_fixed(Coptic(CommonDate::new(year, 1, 1)));
        let month = ((date - year_start.get_day_i()).div_euclid(30) + 1) as u8;
        let month_start = Coptic::to_fixed(Coptic(CommonDate::new(year, month, 1)));

        let day = (date - month_start.get_day_i() + 1) as u8;
        Coptic(CommonDate::new(year, month, day))
    }
}

impl ToFixed for Coptic {
    fn to_fixed(self) -> Fixed {
        let year = self.0.year as i64;
        let month = self.0.month as i64;
        let day = self.0.day as i64;
        let epoch = Coptic::epoch().get_day_i();
        Fixed::cast_new(
            epoch - 1 + (365 * (year - 1)) + year.div_euclid(4) + (30 * (month - 1)) + day,
        )
    }
}

impl ToFromCommonDate for Coptic {
    fn to_common_date(self) -> CommonDate {
        self.0
    }

    fn from_common_date_unchecked(date: CommonDate) -> Self {
        debug_assert!(Self::valid_month_day(date).is_ok());
        Self(date)
    }

    fn valid_month_day(date: CommonDate) -> Result<(), CalendarError> {
        let month_opt = CopticMonth::from_u8(date.month);
        if month_opt.is_none() {
            Err(CalendarError::InvalidMonth)
        } else if date.day < 1 {
            Err(CalendarError::InvalidDay)
        } else if date.day > month_opt.unwrap().length(Coptic::is_leap(date.year)) {
            Err(CalendarError::InvalidDay)
        } else {
            Ok(())
        }
    }
}

impl CommonYear for Coptic {}
impl GuaranteedMonth<CopticMonth> for Coptic {}
impl CommonDay for Coptic {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calendar::julian::JulianMonth;

    use proptest::proptest;

    proptest! {
        #[test]
        fn christmas(y in i16::MIN..i16::MAX) {
            let c = Coptic::try_from_common_date(CommonDate::new(y as i32, CopticMonth::Koiak as u8, 29))?;
            let j = Julian::from_fixed(c.to_fixed());
            assert_eq!(j.month(), JulianMonth::December);
            assert!(j.day() == 25 || j.day() == 26);
        }
    }
}
