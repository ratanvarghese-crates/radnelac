use crate::calendar::coptic::Coptic;
use crate::calendar::julian::Julian;
use crate::common::bound::BoundedDayCount;
use crate::common::date::CommonDate;
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
        let f = Fixed::new(date.get() + Coptic::epoch().get() - Ethiopic::epoch().get());
        Ethiopic::try_from_common_date(Coptic::from_fixed(f).to_common_date())
            .expect("Same month/day validity")
    }
}

impl ToFixed for Ethiopic {
    fn to_fixed(self) -> Fixed {
        let e =
            Coptic::try_from_common_date(self.to_common_date()).expect("Same month/day validity");
        Fixed::new(Ethiopic::epoch().get() + e.to_fixed().get() - Coptic::epoch().get())
    }
}

impl ToFromCommonDate for Ethiopic {
    fn to_common_date(self) -> CommonDate {
        self.0
    }

    fn from_common_date_unchecked(date: CommonDate) -> Self {
        debug_assert!(Self::valid_month_day(date).is_ok());
        Self(date)
    }

    fn valid_month_day(date: CommonDate) -> Result<(), CalendarError> {
        let month_opt = EthiopicMonth::from_u8(date.month);
        if month_opt.is_none() {
            Err(CalendarError::InvalidMonth)
        } else if date.day < 1 {
            Err(CalendarError::InvalidDay)
        } else if date.day > month_opt.unwrap().length(Ethiopic::is_leap(date.year)) {
            Err(CalendarError::InvalidDay)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::bound::EffectiveBound;

    use crate::day_count::RataDie;
    use crate::day_count::FIXED_MAX;
    use crate::day_count::FIXED_MIN;
    const MAX_YEARS: i32 = (FIXED_MAX / 365.25) as i32;

    use proptest::proptest;

    #[test]
    fn bounds_actually_work() {
        assert!(
            Ethiopic::from_fixed(Fixed::effective_min()) < Ethiopic::from_fixed(Fixed::cast_new(0))
        );
        assert!(
            Ethiopic::from_fixed(Fixed::effective_max()) > Ethiopic::from_fixed(Fixed::cast_new(0))
        );
    }

    proptest! {
        #[test]
        fn roundtrip(t in FIXED_MIN..FIXED_MAX) {
            let t0 = RataDie::new(t).to_fixed().to_day();
            let r = Ethiopic::from_fixed(t0);
            let t1 = r.to_fixed();
            assert_eq!(t0, t1);
        }

        #[test]
        fn locked_to_coptic(year in -MAX_YEARS..MAX_YEARS, month in 1..12, day in 1..30) {
            let d = CommonDate{ year: year, month: month as u8, day: day as u8 };
            let a = Ethiopic::try_from_common_date(d).unwrap();
            let e = Coptic::try_from_common_date(d).unwrap();
            let fa = a.to_fixed();
            let fe = e.to_fixed();
            let diff_f = fa.get_day_i() - fe.get_day_i();
            let diff_e = Ethiopic::epoch().get_day_i() - Coptic::epoch().get_day_i();
            assert_eq!(diff_f, diff_e);
        }

        #[test]
        fn invalid_common(year in -MAX_YEARS..MAX_YEARS, month in 14..u8::MAX, day in 31..u8::MAX) {
            let d_list = [
                CommonDate{ year, month, day },
                CommonDate{ year, month: 1, day},
                CommonDate{ year, month, day: 1 },
                CommonDate{ year, month: 1, day: 0},
                CommonDate{ year, month: 0, day: 1 }
            ];
            for d in d_list {
                assert!(Ethiopic::try_from_common_date(d).is_err());
            }
        }
    }
}
