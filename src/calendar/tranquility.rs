use crate::calendar::gregorian::Gregorian;
use crate::clock::ClockTime;
use crate::clock::TimeOfDay;
use crate::common::bound::BoundedDayCount;
use crate::common::date::CommonDate;
use crate::common::date::OrdinalDate;
use crate::common::date::ToFromCommonDate;
use crate::common::error::CalendarError;
use crate::common::math::TermNum;
use crate::day_count::CalculatedBounds;
use crate::day_count::Epoch;
use crate::day_count::Fixed;
use crate::day_count::FromFixed;
use crate::day_count::ToFixed;
use crate::day_cycle::Weekday;
#[allow(unused_imports)] //FromPrimitive is needed for derive
use num_traits::FromPrimitive;
use std::cmp::Ordering;

const TRANQUILITY_EPOCH_GREGORIAN: CommonDate = CommonDate {
    year: 1969,
    month: 7,
    day: 20,
};

const TRANQUILITY_EPOCH_CLOCK: ClockTime = ClockTime {
    hours: 20,
    minutes: 18,
    seconds: 1.2,
};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, FromPrimitive)]
pub enum TranquilityMonth {
    Archimedes = 1,
    Brahe,
    Copernicus,
    Darwin,
    Einstein,
    Faraday,
    Galileo,
    Hippocrates,
    Imhotep,
    Jung,
    Kepler,
    Lavoisier,
    Mendel,
}

const AFTER_H27: i64 = (TranquilityMonth::Hippocrates as i64) * 28;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, FromPrimitive)]
pub enum TranquilityComplementaryDay {
    MoonLandingDay = 0,
    ArmstrongDay,
    AldrinDay,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TranquilityMoment {
    date: CommonDate,
    time: ClockTime,
}

impl TranquilityMoment {
    pub fn year(self) -> Option<i32> {
        if self.date.day == 0 {
            None
        } else {
            Some(self.date.year)
        }
    }

    pub fn month(self) -> Option<TranquilityMonth> {
        if self.date.month == 0 {
            None
        } else {
            TranquilityMonth::from_u8(self.date.month)
        }
    }

    pub fn complementary(self) -> Option<TranquilityComplementaryDay> {
        if self.date.month == 0 {
            TranquilityComplementaryDay::from_u8(self.date.day)
        } else {
            None
        }
    }

    pub fn is_after_tranquility(self) -> bool {
        if self.date.year == 0 {
            self.time > TRANQUILITY_EPOCH_CLOCK
        } else {
            self.date.year > 0
        }
    }

    pub fn day(self) -> u8 {
        self.date.day
    }

    pub fn hour(self) -> u8 {
        self.time.hours
    }

    pub fn minute(self) -> u8 {
        self.time.minutes
    }

    pub fn second(self) -> f32 {
        self.time.seconds
    }

    pub fn weekday(self) -> Option<Weekday> {
        if self.complementary().is_some() {
            None
        } else {
            Weekday::from_i64(((self.date.day as i64) + 4).modulus(7))
        }
    }

    pub fn is_leap(t_year: i32) -> bool {
        if t_year > 0 {
            Gregorian::is_leap(t_year + TRANQUILITY_EPOCH_GREGORIAN.year)
        } else if t_year < 0 {
            Gregorian::is_leap(t_year + TRANQUILITY_EPOCH_GREGORIAN.year + 1)
        } else {
            false
        }
    }

    pub fn complementary_count(p_year: i32) -> u8 {
        if Self::is_leap(p_year) {
            2
        } else if p_year == -1 {
            //Armstrong Day replaced by Moon Landing Day
            0
        } else {
            1
        }
    }

    pub fn prior_elapsed_days(year: i32) -> i64 {
        if year == 0 {
            TranquilityMoment::epoch().get_day_i() - 1
        } else {
            let y = if year < 0 { year + 1 } else { year };
            let prior_g = Gregorian::try_from_common_date(CommonDate {
                year: (y - 1) + TRANQUILITY_EPOCH_GREGORIAN.year,
                month: TRANQUILITY_EPOCH_GREGORIAN.month,
                day: TRANQUILITY_EPOCH_GREGORIAN.day,
            })
            .expect("Month and day known to be valid.");
            prior_g.to_fixed().get_day_i()
        }
    }

    pub fn ordinal_from_fixed(fixed_date: Fixed) -> OrdinalDate {
        //Common
        //Gregorian:   (jan1).....(feb28)(mar1).....(jul20)(jul21).....(dec31)
        //Greg  ord:   (1)...........(59)(60).........(201)(202).........(365)
        //Tranquility: (far25)....(hip27)(hip28)......(arm)(arc1)......(far24)
        //Tran  ord:   (165)........(223)(224)........(365)(1)...........(164)
        //Leap
        //Gregorian:   (jan1).....(feb29)(mar1).....(jul20)(jul21).....(dec31)
        //Greg  ord:   (1)...........(60)(61).........(202)(203).........(366)
        //Tranquility: (far25)......(ald)(hip28)......(arm)(arc1)......(far24)
        //Tran  ord:   (165)........(224)(225)........(366)(1)...........(164)
        const ORDINAL_SHIFT: i64 = ((TranquilityMonth::Faraday as i64) * 28) - 4;
        let g_ord = Gregorian::ordinal_from_fixed(fixed_date);
        let g_doy_shift = (g_ord.day_of_year as i64) + ORDINAL_SHIFT;
        let g_len = if Gregorian::is_leap(g_ord.year) {
            366
        } else {
            365
        };
        let tq_doy = g_doy_shift.adjusted_remainder(g_len);
        let y_approx_0 = g_ord.year - TRANQUILITY_EPOCH_GREGORIAN.year;
        let correct_0 = if tq_doy <= ORDINAL_SHIFT { 1 } else { 0 };
        let y_approx_1 = y_approx_0 + correct_0;
        let year = if y_approx_1 < 1 {
            y_approx_1 - 1
        } else {
            y_approx_1
        };
        if year == -1 && tq_doy == 365 {
            OrdinalDate {
                year: 0,
                day_of_year: 1,
            }
        } else {
            OrdinalDate {
                year: year,
                day_of_year: tq_doy as u16,
            }
        }
    }

    pub fn to_ordinal(self) -> OrdinalDate {
        let comp_count = Self::complementary_count(self.date.year) as i64;
        let ordinal_day = match self.complementary() {
            Some(TranquilityComplementaryDay::MoonLandingDay) => 1,
            Some(TranquilityComplementaryDay::ArmstrongDay) => 364 + comp_count,
            Some(TranquilityComplementaryDay::AldrinDay) => AFTER_H27,
            None => {
                let month = self.date.month as i64;
                let day = self.date.day as i64;
                let approx = ((month - 1) * 28) + day;
                let correction = if approx < AFTER_H27 || comp_count < 2 {
                    0
                } else {
                    1
                };
                approx + correction
            }
        };
        OrdinalDate {
            year: self.date.year,
            day_of_year: ordinal_day as u16,
        }
    }
}

impl PartialOrd for TranquilityMoment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.date == other.date {
            self.time.partial_cmp(&other.time)
        } else {
            let self_ord = self.to_ordinal();
            let other_ord = other.to_ordinal();
            self_ord.partial_cmp(&other_ord)
        }
    }
}

impl CalculatedBounds for TranquilityMoment {}

impl Epoch for TranquilityMoment {
    fn epoch() -> Fixed {
        let date = Gregorian::try_from_common_date(TRANQUILITY_EPOCH_GREGORIAN)
            .expect("Epoch known to be valid")
            .to_fixed();
        let time = TimeOfDay::new_from_clock(TRANQUILITY_EPOCH_CLOCK);
        Fixed::new(date.get() + time.get())
    }
}

impl FromFixed for TranquilityMoment {
    fn from_fixed(date: Fixed) -> TranquilityMoment {
        let ord = TranquilityMoment::ordinal_from_fixed(date);
        let date_tq = match (
            ord.day_of_year as i64,
            ord.year,
            TranquilityMoment::is_leap(ord.year),
        ) {
            (_, 0, _) => CommonDate::new(0, 0, 0),
            (365, _, false) => CommonDate::new(ord.year, 0, 1),
            (366, _, true) => CommonDate::new(ord.year, 0, 1),
            (AFTER_H27, _, true) => CommonDate::new(ord.year, 0, 2),
            (doy, y, is_leap) => {
                let correction = if doy < AFTER_H27 || !is_leap { 0 } else { 1 };
                let month = ((((doy - correction) - 1) as i64).div_euclid(28) + 1) as u8;
                let day = ((doy - correction) as i64).adjusted_remainder(28) as u8;
                debug_assert!(month > 0 && month < 14, "doy: {}, y: {}", doy, y);
                CommonDate::new(y, month, day)
            }
        };
        TranquilityMoment {
            date: date_tq,
            time: ClockTime::new(TimeOfDay::from_fixed(date)),
        }
    }
}

impl ToFixed for TranquilityMoment {
    fn to_fixed(self) -> Fixed {
        let t = TimeOfDay::new_from_clock(self.time);
        let ord = self.to_ordinal();
        let offset_prior = TranquilityMoment::prior_elapsed_days(ord.year);
        Fixed::new((offset_prior as f64) + (ord.day_of_year as f64) + t.get())
    }
}

impl ToFromCommonDate for TranquilityMoment {
    fn to_common_date(self) -> CommonDate {
        self.date
    }

    fn from_common_date_unchecked(date: CommonDate) -> Self {
        debug_assert!(Self::valid_month_day(date).is_ok());
        Self {
            date,
            time: ClockTime::default(),
        }
    }

    fn valid_month_day(date: CommonDate) -> Result<(), CalendarError> {
        if date.month > 13 {
            Err(CalendarError::InvalidMonth)
        } else if date.month == 0 {
            if date.day == 0 && date.year == 0 {
                Ok(())
            } else if date.day == 1 && date.year != 0 && date.year != -1 {
                Ok(())
            } else if date.day == 2 && date.year != 0 && Self::is_leap(date.year) {
                Ok(())
            } else {
                Err(CalendarError::InvalidDay)
            }
        } else if date.day < 1 || date.day > 28 {
            Err(CalendarError::InvalidDay)
        } else if date.year == 0 {
            //Only for Moon Landing Day, as above
            Err(CalendarError::InvalidYear)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day_count::RataDie;

    use crate::common::bound::EffectiveBound;
    use crate::day_count::FIXED_MAX;
    use crate::day_count::FIXED_MIN;

    use proptest::prop_assume;
    use proptest::proptest;
    const MAX_YEARS: i32 = (FIXED_MAX / 365.25) as i32;

    #[test]
    fn bounds_actually_work() {
        assert!(
            TranquilityMoment::from_fixed(Fixed::effective_min())
                < TranquilityMoment::from_fixed(Fixed::cast_new(0))
        );
        assert!(
            TranquilityMoment::from_fixed(Fixed::effective_max())
                > TranquilityMoment::from_fixed(Fixed::cast_new(0))
        );
    }

    #[test]
    fn moon_landing_edge_cases() {
        let f0 = TranquilityMoment::epoch();
        let q = TranquilityMoment::from_fixed(f0);
        assert_eq!(q.to_common_date(), CommonDate::new(0, 0, 0));
        let f1 = q.to_fixed();
        assert_eq!(f0, f1);
    }

    #[test]
    fn obvious_conversions_from_gregorian() {
        let d_list = [
            // Moon Landing Day
            (CommonDate::new(1969, 7, 20), CommonDate::new(0, 0, 0)),
            // Near Moon Landing Day
            (CommonDate::new(1969, 6, 30), CommonDate::new(-1, 13, 9)),
            (CommonDate::new(1969, 7, 1), CommonDate::new(-1, 13, 10)),
            (CommonDate::new(1969, 7, 9), CommonDate::new(-1, 13, 18)),
            (CommonDate::new(1969, 7, 19), CommonDate::new(-1, 13, 28)),
            (CommonDate::new(1969, 7, 21), CommonDate::new(1, 1, 1)),
            (CommonDate::new(1969, 7, 31), CommonDate::new(1, 1, 11)),
            (CommonDate::new(1969, 8, 1), CommonDate::new(1, 1, 12)),
            // Armstrong Day
            (CommonDate::new(1965, 7, 20), CommonDate::new(-5, 0, 1)),
            (CommonDate::new(1968, 7, 20), CommonDate::new(-2, 0, 1)),
            (CommonDate::new(1970, 7, 20), CommonDate::new(1, 0, 1)),
            (CommonDate::new(1989, 7, 20), CommonDate::new(20, 0, 1)), //in Siggins' article
            (CommonDate::new(1995, 7, 20), CommonDate::new(26, 0, 1)),
            (CommonDate::new(1995, 7, 21), CommonDate::new(27, 1, 1)),
            (CommonDate::new(2000, 7, 20), CommonDate::new(31, 0, 1)),
            (CommonDate::new(2020, 7, 20), CommonDate::new(51, 0, 1)),
            (CommonDate::new(2025, 7, 20), CommonDate::new(56, 0, 1)),
            // Aldrin Day
            (CommonDate::new(1968, 2, 29), CommonDate::new(-2, 0, 2)),
            (CommonDate::new(1972, 2, 29), CommonDate::new(3, 0, 2)),
            (CommonDate::new(2000, 2, 29), CommonDate::new(31, 0, 2)),
        ];
        for pair in d_list {
            let dg = Gregorian::try_from_common_date(pair.0).unwrap().to_fixed();
            let dq = TranquilityMoment::try_from_common_date(pair.1)
                .unwrap()
                .to_fixed();
            assert_eq!(dq.get_day_i(), dg.get_day_i());
        }
    }

    #[test]
    fn orions_arm() {
        let d_list = [
            // Orion's Arm Encyclopaedia Galactica - Atomic Age
            // https://www.orionsarm.com/eg-topic/460b135b0c6d8
            // Orion's Arm Calendar Converter
            // https://www.orionsarm.com/xcms.php?r=oa-calendar-converter
            // Wikipedia
            // https://en.wikipedia.org/wiki/Subrahmanyan_Chandrasekhar
            // https://en.wikipedia.org/wiki/Albert_Einstein
            // https://en.wikipedia.org/wiki/Yuri_Gagarin
            // In cases where articles conflict with the calendar converter,
            // the calendar converter is used. These cases are marked with *
            (CommonDate::new(1910, 10, 19), -59), //Chandrasekhar
            (CommonDate::new(1995, 8, 21), 27),   //Chandrasekhar*
            (CommonDate::new(1879, 3, 14), -91),  //Einstein*
            (CommonDate::new(1955, 4, 18), -15),  //Einstein*
            (CommonDate::new(1934, 3, 9), -36),   //Gagarin*
            (CommonDate::new(1968, 3, 27), -2),   //Gagarin*
            (CommonDate::new(1912, 6, 23), -58),  //Turing*
            (CommonDate::new(1954, 6, 7), -16),   //Turing*
        ];
        for pair in d_list {
            let f = Gregorian::try_from_common_date(pair.0).unwrap().to_fixed();
            let dq = TranquilityMoment::from_fixed(f);
            assert_eq!(dq.year().unwrap(), pair.1);
        }
    }

    proptest! {
        #[test]
        fn complementary_xor_weekday(t in FIXED_MIN..FIXED_MAX) {
            let t0 = RataDie::new(t).to_fixed().to_day();
            let r0 = TranquilityMoment::from_fixed(t0);
            let w0 = r0.weekday();
            let s0 = r0.complementary();
            assert_ne!(w0.is_some(), s0.is_some());
        }

        #[test]
        fn invalid_common(year in -MAX_YEARS..MAX_YEARS, month in 14..u8::MAX, day in 32..u8::MAX) {
            let d_list = [
                CommonDate{ year, month, day },
                CommonDate{ year, month: 1, day},
                CommonDate{ year, month, day: 1 },
                CommonDate{ year, month: 1, day: 0},
            ];
            for d in d_list {
                assert!(TranquilityMoment::try_from_common_date(d).is_err());
            }
        }

        #[test]
        fn gregorian_lookup(t in FIXED_MIN..FIXED_MAX) {
            // https://web.archive.org/web/20180818233025/https://en.wikipedia.org/wiki/Tranquility_calendar
            let f = RataDie::new(t).to_fixed().to_day();
            let g = Gregorian::from_fixed(f);
            let gc = g.to_common_date();
            let q = TranquilityMoment::from_fixed(f);
            if q.month().is_some() {
                let qm = q.month().unwrap();
                let entry = match qm {
                    TranquilityMonth::Archimedes => ((7, 21), (8, 17)),
                    TranquilityMonth::Brahe => ((8, 18), (9, 14)),
                    TranquilityMonth::Copernicus => ((9, 15), (10, 12)),
                    TranquilityMonth::Darwin => ((10, 13), (11, 9)),
                    TranquilityMonth::Einstein => ((11, 10), (12, 7)),
                    TranquilityMonth::Faraday => ((12, 8), (1, 4)),
                    TranquilityMonth::Galileo => ((1, 5), (2, 1)),
                    TranquilityMonth::Hippocrates => ((2, 2), (3, 1)),
                    TranquilityMonth::Imhotep => ((3, 2), (3, 29)),
                    TranquilityMonth::Jung => ((3, 30), (4, 26)),
                    TranquilityMonth::Kepler => ((4, 27), (5, 24)),
                    TranquilityMonth::Lavoisier => ((5, 25), (6, 21)),
                    TranquilityMonth::Mendel => ((6, 22), (7, 19)),
                };
                let mut y_min = gc.year;
                let mut y_max = gc.year;
                if qm == TranquilityMonth::Faraday {
                    let in_new_year = gc.month == entry.1.0;
                    y_min = if in_new_year { gc.year - 1 } else { gc.year };
                    y_max = y_min + 1;
                }
                let gc_min = CommonDate::new(y_min, entry.0.0, entry.0.1);
                let gc_max = CommonDate::new(y_max, entry.1.0, entry.1.1);
                assert!(gc >= gc_min, "gc: {:?}, gc_min: {:?}, q: {:?}", gc, gc_min, q);
                assert!(gc <= gc_max, "gc: {:?}, gc_max: {:?}, q: {:?}", gc, gc_max, q);
            } else {
                let qc = q.complementary().unwrap();
                let entry = match qc {
                    TranquilityComplementaryDay::MoonLandingDay => (7, 20),
                    TranquilityComplementaryDay::ArmstrongDay => (7, 20),
                    TranquilityComplementaryDay::AldrinDay => (2, 29)
                };
                assert_eq!(gc.month as i64, entry.0);
                assert_eq!(gc.day as i64, entry.1);
            }
        }

        #[test]
        fn gregorian_lookup_small(t in i8::MIN..i8::MAX) {
            // https://web.archive.org/web/20180818233025/https://en.wikipedia.org/wiki/Tranquility_calendar
            let e = TranquilityMoment::epoch().get_day_i();
            let f = RataDie::cast_new(e + (t as i64)).to_fixed().to_day();
            let g = Gregorian::from_fixed(f);
            let gc = g.to_common_date();
            let q = TranquilityMoment::from_fixed(f);
            if q.month().is_some() {
                let qm = q.month().unwrap();
                let entry = match qm {
                    TranquilityMonth::Archimedes => ((7, 21), (8, 17)),
                    TranquilityMonth::Brahe => ((8, 18), (9, 14)),
                    TranquilityMonth::Copernicus => ((9, 15), (10, 12)),
                    TranquilityMonth::Darwin => ((10, 13), (11, 9)),
                    TranquilityMonth::Einstein => ((11, 10), (12, 7)),
                    TranquilityMonth::Faraday => ((12, 8), (1, 4)),
                    TranquilityMonth::Galileo => ((1, 5), (2, 1)),
                    TranquilityMonth::Hippocrates => ((2, 2), (3, 1)),
                    TranquilityMonth::Imhotep => ((3, 2), (3, 29)),
                    TranquilityMonth::Jung => ((3, 30), (4, 26)),
                    TranquilityMonth::Kepler => ((4, 27), (5, 24)),
                    TranquilityMonth::Lavoisier => ((5, 25), (6, 21)),
                    TranquilityMonth::Mendel => ((6, 22), (7, 19)),
                };
                let mut y_min = gc.year;
                let mut y_max = gc.year;
                if qm == TranquilityMonth::Faraday {
                    let in_new_year = gc.month == entry.1.0;
                    y_min = if in_new_year { gc.year - 1 } else { gc.year };
                    y_max = y_min + 1;
                }
                let gc_min = CommonDate::new(y_min, entry.0.0, entry.0.1);
                let gc_max = CommonDate::new(y_max, entry.1.0, entry.1.1);
                assert!(gc >= gc_min, "gc: {:?}, gc_min: {:?}, q: {:?}", gc, gc_min, q);
                assert!(gc <= gc_max, "gc: {:?}, gc_max: {:?}, q: {:?}", gc, gc_max, q);
            } else {
                let qc = q.complementary().unwrap();
                let entry = match qc {
                    TranquilityComplementaryDay::MoonLandingDay => (7, 20),
                    TranquilityComplementaryDay::ArmstrongDay => (7, 20),
                    TranquilityComplementaryDay::AldrinDay => (2, 29)
                };
                assert_eq!(gc.month as i64, entry.0);
                assert_eq!(gc.day as i64, entry.1);
            }
        }
    }
}
