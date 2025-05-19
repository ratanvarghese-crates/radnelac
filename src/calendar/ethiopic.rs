use crate::calendar::coptic::Coptic;
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

const ETHIOPIC_EPOCH_JULIAN: CommonDate = CommonDate {
    year: 8,
    month: 8,
    day: 29,
};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, FromPrimitive)]
pub enum EthiopicMonth {
    Maskaram = 1,
    Teqemt,
    Hedar,
    Takhsas,
    Ter,
    Yakatit,
    Magabit,
    Miyazya,
    Genbot,
    Sane,
    Hamle,
    Nahase,
    Paguemen,
}

impl EthiopicMonth {
    pub fn length(self, leap: bool) -> u8 {
        match self {
            EthiopicMonth::Paguemen => {
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
pub struct Ethiopic(CommonDate);

impl Ethiopic {
    pub fn year(self) -> i32 {
        self.0.year
    }

    pub fn month(self) -> EthiopicMonth {
        EthiopicMonth::from_u8(self.0.month).expect("Will not allow setting invalid value.")
    }

    pub fn day(self) -> u8 {
        self.0.day
    }

    pub fn is_leap(year: i32) -> bool {
        year.modulus(4) == 3
    }
}

impl CalculatedBounds for Ethiopic {}

impl Epoch for Ethiopic {
    fn epoch() -> Fixed {
        Julian::try_from_common_date(ETHIOPIC_EPOCH_JULIAN)
            .expect("Epoch known to be in range.")
            .to_fixed()
    }
}

impl FromFixed for Ethiopic {
    fn from_fixed(date: Fixed) -> Ethiopic {
        // Deliberately diverging from Calendrical Calculations to avoid crossing bounds checks
        let e =
            Coptic::from_fixed_generic_unchecked(date.get_day_i(), Ethiopic::epoch().get_day_i());
        Ethiopic(e)
    }
}

impl ToFixed for Ethiopic {
    fn to_fixed(self) -> Fixed {
        // Deliberately diverging from Calendrical Calculations to avoid crossing bounds checks
        let e = Coptic::to_fixed_generic_unchecked(self.0, Ethiopic::epoch().get_day_i());
        Fixed::cast_new(e).expect("TODO: verify")
    }
}

impl ToCommonDate for Ethiopic {
    fn to_common_date(self) -> CommonDate {
        self.0
    }
}

impl TryFromCommonDate for Ethiopic {
    fn try_from_common_date(date: CommonDate) -> Result<Self, CalendarError> {
        let month_opt = EthiopicMonth::from_u8(date.month);
        if month_opt.is_none() {
            Err(CalendarError::InvalidMonth)
        } else if date.day < 1 {
            Err(CalendarError::InvalidDay)
        } else if date.day > month_opt.unwrap().length(Ethiopic::is_leap(date.year)) {
            Err(CalendarError::InvalidDay)
        } else {
            let e = Ethiopic(date);
            if e < Ethiopic::effective_min() || e > Ethiopic::effective_max() {
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

    use crate::common::math::EFFECTIVE_MAX;
    use crate::common::math::EFFECTIVE_MIN;
    use crate::day_count::rd::RataDie;

    use proptest::proptest;

    proptest! {
        #[test]
        fn roundtrip(t in EFFECTIVE_MIN..EFFECTIVE_MAX) {
            let t0 = RataDie::checked_new(t).unwrap().to_fixed().to_day();
            let r = Ethiopic::from_fixed(t0);
            let t1 = r.to_fixed();
            assert_eq!(t0, t1);
        }
    }
}
