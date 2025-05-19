use crate::calendar::julian::Julian;
use crate::common::bound::BoundedDayCount;
use crate::common::bound::EffectiveBound;
use crate::common::date::CommonDate;
use crate::common::date::ToCommonDate;
use crate::common::date::TryFromCommonDate;
use crate::common::error::CalendarError;
use crate::common::math::TermNum;
use crate::day_count::fixed::CalculatedBounds;
use crate::day_count::fixed::Epoch;
use crate::day_count::fixed::Fixed;
use crate::day_count::fixed::FromFixed;
use crate::day_count::fixed::ToFixed;
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

impl Coptic {
    pub fn year(self) -> i32 {
        self.0.year
    }

    pub fn month(self) -> CopticMonth {
        CopticMonth::from_u8(self.0.month).expect("Will not allow setting invalid value.")
    }

    pub fn day(self) -> u8 {
        self.0.day
    }

    pub fn is_leap(year: i32) -> bool {
        year.modulus(4) == 3
    }

    pub fn from_fixed_generic_unchecked(date: i64, epoch: i64) -> CommonDate {
        let year = (4 * (date - epoch) + 1463).div_euclid(1461) as i32;
        let year_start = Coptic::to_fixed_generic_unchecked(CommonDate::new(year, 1, 1), epoch);
        let month = ((date - year_start).div_euclid(30) + 1) as u8;
        let month_start =
            Coptic::to_fixed_generic_unchecked(CommonDate::new(year, month, 1), epoch);

        let day = (date - month_start + 1) as u8;
        CommonDate::new(year, month, day)
    }

    pub fn to_fixed_generic_unchecked(date: CommonDate, epoch: i64) -> i64 {
        let year = date.year as i64;
        let month = date.month as i64;
        let day = date.day as i64;

        epoch - 1 + (365 * (year - 1)) + year.div_euclid(4) + (30 * (month - 1)) + day
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
    fn from_fixed(date: Fixed) -> Coptic {
        Coptic(Coptic::from_fixed_generic_unchecked(
            date.get_day_i(),
            Coptic::epoch().get_day_i(),
        ))
    }
}

impl ToFixed for Coptic {
    fn to_fixed(self) -> Fixed {
        Fixed::cast_new(Coptic::to_fixed_generic_unchecked(
            self.0,
            Coptic::epoch().get_day_i(),
        ))
        .expect("TODO: verify")
    }
}

impl ToCommonDate for Coptic {
    fn to_common_date(self) -> CommonDate {
        self.0
    }
}

impl TryFromCommonDate for Coptic {
    fn try_from_common_date(date: CommonDate) -> Result<Self, CalendarError> {
        let month_opt = CopticMonth::from_u8(date.month);
        if month_opt.is_none() {
            Err(CalendarError::InvalidMonth)
        } else if date.day < 1 {
            Err(CalendarError::InvalidDay)
        } else if date.day > month_opt.unwrap().length(Coptic::is_leap(date.year)) {
            Err(CalendarError::InvalidDay)
        } else {
            let e = Coptic(date);
            if e < Coptic::effective_min() || e > Coptic::effective_max() {
                Err(CalendarError::OutOfBounds)
            } else {
                Ok(e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calendar::julian::JulianMonth;

    use crate::common::math::EFFECTIVE_MAX;
    use crate::common::math::EFFECTIVE_MIN;
    use crate::day_count::rd::RataDie;

    use proptest::proptest;

    proptest! {
        #[test]
        fn roundtrip(t in EFFECTIVE_MIN..EFFECTIVE_MAX) {
            let t0 = RataDie::checked_new(t).unwrap().to_fixed().to_day();
            let r = Coptic::from_fixed(t0);
            let t1 = r.to_fixed();
            assert_eq!(t0, t1);
        }

        #[test]
        fn christmas(y in i16::MIN..i16::MAX) {
            let c = Coptic::try_from_common_date(CommonDate::new(y as i32, CopticMonth::Koiak as u8, 29))?;
            let j = Julian::from_fixed(c.to_fixed());
            assert_eq!(j.month(), JulianMonth::December);
            assert!(j.day() == 25 || j.day() == 26);
        }
    }
}
