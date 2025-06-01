use crate::calendar::julian::Julian;
use crate::calendar::julian::JulianMonth;
use crate::common::bound::BoundedDayCount;
use crate::common::date::CommonDate;
use crate::common::date::ToFromCommonDate;
use crate::common::math::TermNum;
use crate::day_count::CalculatedBounds;
use crate::day_count::Fixed;
use crate::day_count::FromFixed;
use crate::day_count::ToFixed;
use std::cmp::Ordering;
use std::num::NonZero;

#[allow(unused_imports)] //FromPrimitive is needed for derive
use num_traits::FromPrimitive;

const YEAR_ROME_FOUNDED_JULIAN: i32 = -753;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, FromPrimitive)]
pub enum RomanMonthlyEvent {
    Kalends = 1,
    Nones,
    Ides,
}

pub type RomanMonth = JulianMonth;

impl RomanMonth {
    pub fn ides_of_month(self) -> u8 {
        match self {
            RomanMonth::July => 15,
            RomanMonth::March => 15,
            RomanMonth::May => 15,
            RomanMonth::October => 15,
            _ => 13,
        }
    }

    pub fn nones_of_month(self) -> u8 {
        self.ides_of_month() - 8
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Roman {
    year: i32,
    month: RomanMonth,
    event: RomanMonthlyEvent,
    count: u8,
    leap: bool,
}

impl Roman {
    pub fn year(self) -> i32 {
        self.year
    }

    pub fn month(self) -> RomanMonth {
        self.month
    }

    pub fn event(self) -> RomanMonthlyEvent {
        self.event
    }

    pub fn count(self) -> u8 {
        self.count
    }

    pub fn leap(self) -> bool {
        self.leap
    }

    pub fn julian_year_from_auc(year: NonZero<i32>) -> NonZero<i32> {
        let j_year = year.get();
        if j_year >= 1 && j_year <= -YEAR_ROME_FOUNDED_JULIAN {
            NonZero::new(j_year + YEAR_ROME_FOUNDED_JULIAN - 1).expect("Checked by if")
        } else {
            NonZero::new(j_year + YEAR_ROME_FOUNDED_JULIAN).expect("Checked by if")
        }
    }

    pub fn auc_year_from_julian(year: NonZero<i32>) -> NonZero<i32> {
        let a_year = year.get();
        if YEAR_ROME_FOUNDED_JULIAN <= a_year && a_year <= -1 {
            NonZero::new(a_year - YEAR_ROME_FOUNDED_JULIAN + 1).expect("Checked by if")
        } else {
            NonZero::new(a_year - YEAR_ROME_FOUNDED_JULIAN).expect("Checked by if")
        }
    }

    pub fn from_julian_unchecked(j_cdate: CommonDate, date: i64) -> (i32, u8, u8, u8, bool) {
        let month = (j_cdate.month as i64).adjusted_remainder(12) as u8;
        let year = j_cdate.year;
        let day = j_cdate.day;
        let month1 = (month as i64 + 1).adjusted_remainder(12) as u8;
        let year1 = if month1 != 1 {
            year
        } else if year != -1 {
            year + 1
        } else {
            1
        };
        let month_r = RomanMonth::from_u8(month).expect("Kept in range by adjusted_remainder");
        let month1_r = RomanMonth::from_u8(month1).expect("Kept in range by adjusted_remainder");
        let kalends1 = Roman {
            year: year1,
            month: month1_r,
            event: RomanMonthlyEvent::Kalends,
            count: 1,
            leap: false,
        }
        .to_fixed()
        .get_day_i();
        if day == 1 {
            (year, month, RomanMonthlyEvent::Kalends as u8, 1, false)
        } else if day <= month_r.nones_of_month() {
            (
                year,
                month,
                RomanMonthlyEvent::Nones as u8,
                month_r.nones_of_month() - day + 1,
                false,
            )
        } else if day <= month_r.ides_of_month() {
            (
                year,
                month,
                RomanMonthlyEvent::Ides as u8,
                month_r.ides_of_month() - day + 1,
                false,
            )
        } else if month_r != RomanMonth::February || !Julian::is_leap(year) {
            (
                year1,
                month1,
                RomanMonthlyEvent::Kalends as u8,
                ((kalends1 - date) + 1) as u8,
                false,
            )
        } else if day < 25 {
            (
                year,
                RomanMonth::March as u8,
                RomanMonthlyEvent::Kalends as u8,
                (30 - day) as u8,
                false,
            )
        } else {
            (
                year,
                RomanMonth::March as u8,
                RomanMonthlyEvent::Kalends as u8,
                (31 - day) as u8,
                day == 25,
            )
        }
    }
}

impl PartialOrd for Roman {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.year != other.year {
            self.year.partial_cmp(&other.year)
        } else if self.month != other.month {
            self.month.partial_cmp(&other.month)
        } else if self.event != other.event {
            self.event.partial_cmp(&other.event)
        } else {
            other.count.partial_cmp(&self.count) //Intentionally reversed, "count" decreases with time
        }
    }
}

impl CalculatedBounds for Roman {}

impl FromFixed for Roman {
    fn from_fixed(date: Fixed) -> Roman {
        let j_cdate = Julian::from_fixed(date).to_common_date();
        let r_fields = Roman::from_julian_unchecked(j_cdate, date.get_day_i());
        Roman {
            year: r_fields.0,
            month: RomanMonth::from_u8(r_fields.1).expect("TODO: verify"),
            event: RomanMonthlyEvent::from_u8(r_fields.2).expect("TODO: verify"),
            count: r_fields.3,
            leap: r_fields.4,
        }
    }
}

impl ToFixed for Roman {
    fn to_fixed(self) -> Fixed {
        let jld = match self.event {
            RomanMonthlyEvent::Kalends => 1,
            RomanMonthlyEvent::Nones => self.month.nones_of_month(),
            RomanMonthlyEvent::Ides => self.month.ides_of_month(),
        };
        let jlc = CommonDate::new(self.year, self.month as u8, jld);
        let j = Julian::try_from_common_date(jlc)
            .expect("Month/day in range")
            .to_fixed()
            .get_day_i();
        let c = self.count as i64;
        let do_lp = Julian::is_leap(self.year)
            && self.month == RomanMonth::March
            && self.event == RomanMonthlyEvent::Kalends
            && self.count <= 16
            && self.count >= 6;
        let lp0 = if do_lp { 0 } else { 1 };
        let lp1 = if self.leap { 1 } else { 0 };
        Fixed::cast_new(j - c + lp0 + lp1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::bound::EffectiveBound;
    use crate::common::date::ToFromCommonDate;
    use crate::day_count::FIXED_MAX;
    use crate::day_count::FIXED_MIN;
    use crate::day_count::RataDie;
    use proptest::prop_assume;
    use proptest::proptest;

    #[test]
    fn bounds_actually_work() {
        assert!(Roman::from_fixed(Fixed::effective_min()) < Roman::from_fixed(Fixed::cast_new(0)));
        assert!(Roman::from_fixed(Fixed::effective_max()) > Roman::from_fixed(Fixed::cast_new(0)));
    }

    #[test]
    fn ides_of_march() {
        let j = Julian::try_from_common_date(CommonDate::new(-44, 3, 15)).unwrap();
        let f = j.to_fixed();
        let r = Roman::from_fixed(f);
        assert_eq!(r.event, RomanMonthlyEvent::Ides);
        assert_eq!(r.month, RomanMonth::March);
        assert_eq!(r.count, 1);
    }

    proptest! {
        #[test]
        fn roundtrip(t in FIXED_MIN..FIXED_MAX) {
            let t0 = RataDie::new(t).to_fixed().to_day();
            let r = Roman::from_fixed(t0);
            let t1 = r.to_fixed();
            assert_eq!(t0, t1);
        }

        #[test]
        fn auc_roundtrip(t in i16::MIN..i16::MAX) {
            prop_assume!(t != 0);
            assert_eq!(t as i32, Roman::julian_year_from_auc(Roman::auc_year_from_julian(NonZero::new(t as i32).unwrap())).get());
        }

        #[test]
        fn consistent_order(t0 in FIXED_MIN..FIXED_MAX, t1 in FIXED_MIN..FIXED_MAX) {
            let f0 = Fixed::new(t0);
            let f1 = Fixed::new(t1);
            let d0 = Roman::from_fixed(f0);
            let d1 = Roman::from_fixed(f1);
            assert_eq!(f0 < f1, d0 < d1);
            assert_eq!(f0 <= f1, d0 <= d1);
            assert_eq!(f0 == f1, d0 == d1);
            assert_eq!(f0 >= f1, d0 >= d1);
            assert_eq!(f0 > f1, d0 > d1);
        }

        #[test]
        fn consistent_order_small(t0 in FIXED_MIN..FIXED_MAX, diff in i8::MIN..i8::MAX) {
            let f0 = Fixed::new(t0);
            let f1 = Fixed::new(t0 + (diff as f64));
            let d0 = Roman::from_fixed(f0);
            let d1 = Roman::from_fixed(f1);
            assert_eq!(f0 < f1, d0 < d1);
            assert_eq!(f0 <= f1, d0 <= d1);
            assert_eq!(f0 == f1, d0 == d1);
            assert_eq!(f0 >= f1, d0 >= d1);
            assert_eq!(f0 > f1, d0 > d1);
        }
    }
}
