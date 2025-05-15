use crate::calendar::common::CommonDate;
use crate::calendar::julian::Julian;
use crate::calendar::julian::JulianMonth;
use crate::epoch::fixed::FixedDate;
use crate::error::CalendarError;
use crate::math::TermNum;
use std::num::NonZero;

const YEAR_ROME_FOUNDED: i32 = -753;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum RomanMonthlyEvent {
    Kalends = 1,
    Nones,
    Ides,
}

pub type RomanMonth = JulianMonth;

impl RomanMonth {
    fn ides_of_month(self) -> u8 {
        match self {
            RomanMonth::July => 15,
            RomanMonth::March => 15,
            RomanMonth::May => 15,
            RomanMonth::October => 15,
            _ => 13,
        }
    }

    fn nones_of_month(self) -> u8 {
        self.ides_of_month() - 8
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Roman {
    year: i32,
    month: RomanMonth,
    event: RomanMonthlyEvent,
    count: i8,
    leap: bool,
}

impl Roman {
    fn julian_year_from_auc(year: NonZero<i32>) -> NonZero<i32> {
        let j_year = year.get();
        if j_year >= 1 && j_year <= -YEAR_ROME_FOUNDED {
            NonZero::new(j_year + YEAR_ROME_FOUNDED - 1).unwrap()
        } else {
            NonZero::new(j_year + YEAR_ROME_FOUNDED).unwrap()
        }
    }

    fn auc_year_from_julian(year: NonZero<i32>) -> NonZero<i32> {
        let a_year = year.get();
        if YEAR_ROME_FOUNDED <= a_year && a_year <= -1 {
            NonZero::new(a_year - YEAR_ROME_FOUNDED + 1).unwrap()
        } else {
            NonZero::new(a_year - YEAR_ROME_FOUNDED).unwrap()
        }
    }
}

impl From<Roman> for FixedDate {
    fn from(date: Roman) -> FixedDate {
        let jld = match date.event {
            RomanMonthlyEvent::Kalends => 1,
            RomanMonthlyEvent::Nones => date.month.nones_of_month(),
            RomanMonthlyEvent::Ides => date.month.ides_of_month(),
        };
        let jlc = CommonDate::try_new(date.year, date.month as u8, jld)
            .expect("TODO: CommonDate enforces year limits?");
        let jldd = Julian::try_from(jlc).expect("TODO: CommonDate enforces year limits?");
        let j = i64::from(FixedDate::from(jldd));
        let c = date.count as i64;
        let do_lp = Julian::is_leap(date.year)
            && date.month == RomanMonth::March
            && date.event == RomanMonthlyEvent::Kalends
            && date.count <= 16
            && date.count >= 6;
        let lp0 = if do_lp { 0 } else { 1 };
        let lp1 = if date.leap { 1 } else { 0 };
        FixedDate::try_from(j - c + lp0 + lp1).expect("TODO: CommonDate enforces year limits?")
    }
}

impl TryFrom<FixedDate> for Roman {
    type Error = CalendarError;
    fn try_from(date: FixedDate) -> Result<Roman, Self::Error> {
        let j_date = Julian::try_from(date)?;
        let j_cdate = CommonDate::from(j_date);
        let month = u8::from(j_cdate.get_month());
        let year = j_cdate.get_year();
        let day = j_cdate.get_day();
        let month1 = (j_cdate.get_month() as i64 + 1).adjusted_remainder(12);
        let year1 = if month1 != 1 {
            year
        } else if year != -1 {
            year + 1
        } else {
            1
        };
        let kalends1 = FixedDate::from(Roman {
            year: year1,
            month: RomanMonth::try_from(month1 as u8)?,
            event: RomanMonthlyEvent::Kalends,
            count: 1,
            leap: false,
        });
        if day == 1 {
            Ok(Roman {
                year: year,
                month: RomanMonth::try_from(month as u8)?,
                event: RomanMonthlyEvent::Kalends,
                count: 1,
                leap: false,
            })
        } else if day <= RomanMonth::try_from(month as u8)?.nones_of_month() {
            Ok(Roman {
                year: year,
                month: RomanMonth::try_from(month as u8)?,
                event: RomanMonthlyEvent::Nones,
                count: RomanMonth::try_from(month as u8)?.nones_of_month() as i8 - day as i8 + 1,
                leap: false,
            })
        } else if day <= RomanMonth::try_from(month as u8)?.ides_of_month() {
            Ok(Roman {
                year: year,
                month: RomanMonth::try_from(month as u8)?,
                event: RomanMonthlyEvent::Ides,
                count: RomanMonth::try_from(month as u8)?.ides_of_month() as i8 - day as i8 + 1,
                leap: false,
            })
        } else if RomanMonth::try_from(month as u8)? != RomanMonth::February
            || !Julian::is_leap(year)
        {
            if kalends1 < date {
                println!(
                    "what: {} why: {} {} how: {} {}",
                    ((kalends1 - date) + 1),
                    month1,
                    year1,
                    month,
                    year
                );
            }
            debug_assert!(kalends1 >= date);
            Ok(Roman {
                year: year1,
                month: RomanMonth::try_from(month1 as u8)?,
                event: RomanMonthlyEvent::Kalends,
                count: ((kalends1 - date) + 1) as i8,
                leap: false,
            })
        } else if day < 25 {
            Ok(Roman {
                year: year,
                month: RomanMonth::March,
                event: RomanMonthlyEvent::Kalends,
                count: (30 - day) as i8,
                leap: false,
            })
        } else {
            Ok(Roman {
                year: year,
                month: RomanMonth::March,
                event: RomanMonthlyEvent::Kalends,
                count: (31 - day) as i8,
                leap: day == 25,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::epoch::fixed::FixedMoment;
    use crate::epoch::rd::RataDie;
    use proptest::prop_assume;

    use crate::epoch::rd::MAX_RD;
    use crate::epoch::rd::MIN_RD;
    use proptest::proptest;

    proptest! {
        #[test]
        fn roundtrip(t in MIN_RD..MAX_RD) {
            let t0 = FixedDate::from(FixedMoment::from(RataDie::try_from(t).unwrap()));
            let r = Roman::try_from(t0).unwrap();
            let t1 = FixedDate::from(r);
            assert_eq!(t0, t1);
        }

        #[test]
        fn auc_roundtrip(t in i16::MIN..i16::MAX) {
            prop_assume!(t != 0);
            assert_eq!(t as i32, Roman::julian_year_from_auc(Roman::auc_year_from_julian(NonZero::new(t as i32).unwrap())).get());
        }
    }
}
