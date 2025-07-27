use crate::calendar::gregorian::Gregorian;
use crate::calendar::prelude::CommonDate;
use crate::calendar::prelude::HasLeapYears;
use crate::calendar::prelude::OrdinalDate;
use crate::calendar::prelude::Perennial;
use crate::calendar::prelude::Quarter;
use crate::calendar::prelude::ToFromCommonDate;
use crate::calendar::prelude::ToFromOrdinalDate;
use crate::calendar::HasIntercalaryDays;
use crate::clock::ClockTime;
use crate::clock::TimeOfDay;
use crate::common::error::CalendarError;
use crate::common::math::TermNum;
use crate::day_count::BoundedDayCount;
use crate::day_count::CalculatedBounds;
use crate::day_count::Epoch;
use crate::day_count::Fixed;
use crate::day_count::FromFixed;
use crate::day_count::ToFixed;
use crate::day_cycle::Weekday;
#[allow(unused_imports)] //FromPrimitive is needed for derive
use num_traits::FromPrimitive;
use std::cmp::Ordering;
use std::num::NonZero;

const TRANQUILITY_EPOCH_GREGORIAN: CommonDate = CommonDate {
    year: 1969,
    month: 7,
    day: 20,
};
const NON_MONTH: u8 = 0;

const TRANQUILITY_EPOCH_CLOCK: ClockTime = ClockTime {
    hours: 20,
    minutes: 18,
    seconds: 1.2,
};

/// Represents a month of the Tranquility Calendar
///
/// The Tranquility months are named after famous historical figures.
///
/// Note that the complementary days of the Tranquility calendar year have no
/// month and thus are not represented by TranquilityMonth. When representing an
/// arbitrary day in the Tranquility calendar, use an `Option<TranquilityMonth>` for the
/// the month field.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, FromPrimitive, ToPrimitive)]
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

/// Represents a complementary day of the Tranquility Calendar
///
/// These are a bit more complex than the complementary days of the Positivist
/// or Cotsworth calendars.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum TranquilityComplementaryDay {
    /// This is the day of the Apollo 11 moon landing, which is the epoch for
    /// the Tranquility calendar. It is not part of any week, month or year.
    MoonLandingDay = 0,
    /// This is the last day of every year except 1 Before Tranquility. It
    /// is not part of any week or month.
    ArmstrongDay,
    /// This is an extra day added during leap years. It occurs between Hippocrates
    /// 27 and 28. It is not part of any week or month.
    AldrinDay,
}

/// Represents a date *and time* in the Tranquility Calendar
///
/// The Tranquility calendar was proposed by Jeff Siggins in the July 1989 issue of
/// OMNI magazine. It is used by the Orion's Arm collaborative science fiction project.
/// Hardly anybody else uses the Tranquility calendar - it is so obscure that its Wikipedia
/// page was deleted for lack of notability.
///
/// Siggins' article starts with the following:
/// > Are you tired of that same old calendar, with 12 months of unequal length, and dates
/// > that always fall on different days? Do you forget lunch dates, closing days for real
/// > estate deals, or deadlines for IRA rollovers? When plotting graphs in your fruit fly
/// > experiment, do you ever get confused? Do you wonder why that same vile mood hits on the
/// > fifth of one month and the fifteenth of the next?
/// >
/// > If these problems are yours, you are probably ready for the next step in time accounting,
/// > the Tranquility calendar, designed for a perfection-seeking society, especially the men
/// > and women of science. Inspired by the Apollo 11 manned mission to the moon and developed
/// > for *Omni*, the Tranquility calendar will ease the complexity of scientific calculation,
/// > help astronomers fathom the movements of heavenly spheres, and facilitate high-stakes business.
/// > It will also aid everyday users who simply require a precise, easy-to-follow record of
/// > the events of their lives.
///
/// Unfortunately, despite these lofty goals, the Tranquility Calendar has many edge cases not
/// present in other timekeeping systems; this can cause difficulty when implementing software
/// applications. **There are almost certainly discrepancies between this library and others
/// attempting to implement the Tranquility calendar**.
///
/// ## Year 0
///
/// Year 0 is **not** supported for this calendar **except** when representing Moon Landing
/// Day as a `CommonDate`. Technically, Moon Landing Day is not part of any week, month, or
/// year.
///
/// ## Further reading
/// + [Orion's Arm "Encyclopaedia Galactica"](https://www.orionsarm.com/eg-article/48c6d4c3d54cf/)
/// + [Wikipedia Deletion Log](https://en.wikipedia.org/wiki/Wikipedia:Articles_for_deletion/Tranquility_Calendar)
/// + [Archived Wikipedia article](https://web.archive.org/web/20180818233025/https://en.wikipedia.org/wiki/Tranquility_calendar)
/// + Archived copies of Jeff Siggins' article for OMNI
///   + [archive.org copy of mithrandir.com](https://web.archive.org/web/20161025042320/http://www.mithrandir.com/Tranquility/tranquilityArticle.html)
///   + [archive.org copy of OMNI July 1989, pages 63, 64](https://archive.org/details/omni-archive/OMNI_1989_07/page/n63/mode/2up)
///   + [archive.org copy of OMNI July 1989, pages 65, 66](https://archive.org/details/omni-archive/OMNI_1989_07/page/n65/mode/2up)

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TranquilityMoment {
    date: CommonDate,
    time: ClockTime,
}

impl TranquilityMoment {
    pub fn is_after_tranquility(self) -> bool {
        if self.date.year == 0 {
            self.time > TRANQUILITY_EPOCH_CLOCK
        } else {
            self.date.year > 0
        }
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
}

impl ToFromOrdinalDate for TranquilityMoment {
    fn valid_ordinal(ord: OrdinalDate) -> Result<(), CalendarError> {
        let correction = if TranquilityMoment::is_leap(ord.year) {
            1
        } else {
            0
        };
        if ord.day_of_year > 0 && ord.day_of_year <= (365 + correction) {
            Ok(())
        } else {
            Err(CalendarError::InvalidDayOfYear)
        }
    }

    fn ordinal_from_fixed(fixed_date: Fixed) -> OrdinalDate {
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

    fn to_ordinal(self) -> OrdinalDate {
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

    fn from_ordinal_unchecked(ord: OrdinalDate) -> Self {
        let date_tq = match (
            ord.day_of_year as i64,
            ord.year,
            TranquilityMoment::is_leap(ord.year),
        ) {
            (_, 0, _) => CommonDate::new(0, NON_MONTH, 0),
            (365, _, false) => CommonDate::new(ord.year, NON_MONTH, 1),
            (366, _, true) => CommonDate::new(ord.year, NON_MONTH, 1),
            (AFTER_H27, _, true) => CommonDate::new(ord.year, NON_MONTH, 2),
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
            time: ClockTime {
                hours: 0,
                minutes: 0,
                seconds: 0.0,
            },
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

impl HasIntercalaryDays<TranquilityComplementaryDay> for TranquilityMoment {
    fn complementary(self) -> Option<TranquilityComplementaryDay> {
        if self.date.month == NON_MONTH {
            TranquilityComplementaryDay::from_u8(self.date.day)
        } else {
            None
        }
    }

    fn complementary_count(p_year: i32) -> u8 {
        if Self::is_leap(p_year) {
            2
        } else if p_year == -1 {
            //Armstrong Day replaced by Moon Landing Day
            0
        } else {
            1
        }
    }
}

impl Perennial<TranquilityMonth, Weekday> for TranquilityMoment {
    fn weekday(self) -> Option<Weekday> {
        if self.complementary().is_some() {
            None
        } else {
            Weekday::from_i64(((self.date.day as i64) + 4).modulus(7))
        }
    }

    fn days_per_week() -> u8 {
        7
    }

    fn weeks_per_month() -> u8 {
        4
    }
}

impl HasLeapYears for TranquilityMoment {
    fn is_leap(t_year: i32) -> bool {
        if t_year > 0 {
            Gregorian::is_leap(t_year + TRANQUILITY_EPOCH_GREGORIAN.year)
        } else if t_year < 0 {
            Gregorian::is_leap(t_year + TRANQUILITY_EPOCH_GREGORIAN.year + 1)
        } else {
            false
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
        TranquilityMoment {
            date: TranquilityMoment::from_ordinal_unchecked(ord).date,
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

impl ToFromCommonDate<TranquilityMonth> for TranquilityMoment {
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
        } else if date.month == NON_MONTH {
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

    fn year_end_date(year: i32) -> CommonDate {
        CommonDate::new(
            year,
            NON_MONTH,
            TranquilityComplementaryDay::ArmstrongDay as u8,
        )
    }
}

impl Quarter for TranquilityMoment {
    fn quarter(self) -> NonZero<u8> {
        let m = self.to_common_date().month;
        if m == NON_MONTH {
            let d = self.to_common_date().day;
            if d == 2 {
                NonZero::new(3 as u8).expect("2 != 0")
            } else {
                NonZero::new(4 as u8).expect("4 != 0")
            }
        } else if m == 13 {
            NonZero::new(4 as u8).expect("4 != 0")
        } else {
            NonZero::new(((m - 1) / 3) + 1).expect("(m - 1)/3 > -1")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day_count::RataDie;

    use crate::day_count::FIXED_MAX;
    use crate::day_count::FIXED_MIN;

    use proptest::proptest;

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
    fn article_examples() {
        let d_list = [
            // From Siggins' article
            // 21, 1, 3
            // Beginning of the Perseid meteor showers (1989)
            // ???
            // Louise Brown, first test-tube baby, is born (1978)
            // 1978-07-25 https://en.wikipedia.org/wiki/Louise_Brown
            (CommonDate::new(1978, 7, 25), CommonDate::new(10, 1, 5)),
            // NASA established and funded by Congress (1958)
            // 1958-07-29 https://en.wikipedia.org/wiki/NASA
            (CommonDate::new(1958, 7, 29), CommonDate::new(-11, 1, 9)),
            // Explorer VI transmits first picture of Earth from space (1959)
            // 1959-08-14 https://en.wikipedia.org/wiki/Explorer_6
            // (CommonDate::new(1959, 8, 14), CommonDate::new(-10, 1, 18)), //Incorrect???
            // USSR explodes its first hydrogen bomb (1953)
            // 1953-08-12 https://en.wikipedia.org/wiki/RDS-6s
            (CommonDate::new(1953, 8, 12), CommonDate::new(-16, 1, 23)),
            // Lunar eclipse (1989)
            // 1989-08-17 https://en.wikipedia.org/wiki/August_1989_lunar_eclipse
            (CommonDate::new(1989, 8, 17), CommonDate::new(21, 1, 28)),
            // Launch of Voyager 2, space-craft for planetary exploration (1977)
            // 1977-08-20 https://en.wikipedia.org/wiki/Voyager_2
            (CommonDate::new(1977, 8, 20), CommonDate::new(9, 2, 3)),
            // -1890, 2, 7
            // Pliny the Elder dies in eruption of Vesuvius at Pompeii (79)
            // ???
            // Partial solar eclipse (1989)
            // 1989-08-31 wiki:https://en.wikipedia.org/wiki/Solar_eclipse_of_August_31,_1989
            (CommonDate::new(1989, 8, 31), CommonDate::new(21, 2, 14)),
            // Viking 2 lands near polar cap of Mars, sends photos of Earth (1976)
            // 1976-09-03 wiki:https://en.wikipedia.org/wiki/Viking_2
            (CommonDate::new(1976, 9, 3), CommonDate::new(8, 2, 17)),
            // Charles Darwin, in a letter to American botanist Asa Gray, propounds the law of evolution of species by means of natural selection (1857)
            // 1857-09-05 wiki:https://en.wikipedia.org/wiki/Charles_Darwin
            (CommonDate::new(1857, 9, 5), CommonDate::new(-112, 2, 19)),
            // First privately owned rocket launched by Space Services, Inc., of the USA (1982)
            // 1982-09-09 wiki:https://en.wikipedia.org/wiki/Conestoga_(rocket)
            (CommonDate::new(1982, 9, 9), CommonDate::new(14, 2, 23)),
            // 16, 2, 28
            // First solo balloon flight across the Atlantic Ocean embarks from Maine, arrives 86 hours later (1984)
            // ???
            // Autumnal equinox. The serpent of light appears on the pyramid at Chichen Itza Mexico (1989)
            // 1989-09-23 https://www.timeanddate.com/calendar/seasons.html?year=1950&n=1440
            (CommonDate::new(1989, 9, 23), CommonDate::new(21, 3, 9)),
            // "Blue sun," a 200-mile-wide blanket of smoke, caused by forest fires in Alberta and British Columbia, Canada (1950)
            // 1950-09-24 https://en.wikipedia.org/wiki/Chinchaga_fire (black sunday)
            // (CommonDate::new(1950, 9, 24), CommonDate::new(-19, 3, 12)), //Incorrect?
            // Shuttle Discovery launched, first manned US craft since the Challenger (1988)
            // 1988-09-29 https://en.wikipedia.org/wiki/STS-26
            (CommonDate::new(1988, 9, 29), CommonDate::new(20, 3, 15)),
            // 21, 3, 16
            // First day of year 5750 on the Judaic calendar (1989)
            // ???
            // Sputnik 1, first successful man-made satellite, is launched by the USSR (1957)
            // 1957-10-04 https://en.wikipedia.org/wiki/Sputnik_1
            (CommonDate::new(1957, 10, 4), CommonDate::new(-12, 3, 20)),
            // -302, 3, 25
            // Antonie van Leeuwenhoek announces discovery of microorganisms (1667)
            // ???
            // Christopher Columbus lands in the Bahamas (1492)
            // 1492-10-12 https://en.wikipedia.org/wiki/Voyages_of_Christopher_Columbus
            // (CommonDate::new(1492, 10, 2), CommonDate::new(-477, 3, 28)), //Incorrect?
            // USAF Chaptain Charles Yeager becomes first human to fly faster than the speed of sound
            // 1947-10-14 https://en.wikipedia.org/wiki/Bell_X-1
            // (CommonDate::new(1947, 10, 4), CommonDate::new(-22, 4, 2)), //Incorrect?
            // -123, 4, 4
            // First public operation using ether as an anesthetic is performed at Massachusetts General Hospital
            // ???
            // Thomas Edison invents incandescent electric lamp (1879)
            // 1879-10-22 https://en.wikipedia.org/wiki/Incandescent_light_bulb
            // (CommonDate::new(1879, 10, 22), CommonDate::new(-90, 4, 9)), //Incorrect?
            // The first recorded parachute descent, from a balloon (1797) (1st Brumaire, Year VI of the Republican calendar)
            // 1797-10-22 https://en.wikipedia.org/wiki/Andr%C3%A9-Jacques_Garnerin
            (CommonDate::new(1797, 10, 22), CommonDate::new(-172, 4, 10)),
            // -10, 4, 15
            // The Soviet Union releases the first pictures of the far side of the moon, taken by Lunik 3 (1959)
            // ??? 1959-10-07 wiki:https://en.wikipedia.org/wiki/Luna_3
            // Laika, a Russian dog, becomes the first "higher" animal in space (1957)
            // 1957-11-03 https://en.wikipedia.org/wiki/Sputnik_2
            (CommonDate::new(1957, 11, 3), CommonDate::new(-12, 4, 22)),
            // Archaeologist Howard Carter discovers tomb of King Tut at Luxor, Egypt (1922)
            // 1992-11-04 https://en.wikipedia.org/wiki/Discovery_of_the_tomb_of_Tutankhamun
            // (CommonDate::new(1992, 11, 4), CommonDate::new(-47, 4, 23)), // Incorrect?
            // 25, 4, 28
            // The next transit of the planet Mercury across the face of the sun (1993)
            // ??? 1993-11-06 https://en.m.wikipedia.org/wiki/Transit_of_Mercury
            // The first coast-to-coast direct-dial telephone service begins, Englewood, New Jersey (1951)
            // 1951-11-10 https://en.wikipedia.org/wiki/Englewood,_New_Jersey
            (CommonDate::new(1951, 11, 10), CommonDate::new(-18, 5, 1)),
            // Voyager 1 nears Saturn; photos reveal three new moons (1980)
            // 1980-11-12 https://en.wikipedia.org/wiki/Voyager_1
            (CommonDate::new(1980, 11, 12), CommonDate::new(12, 5, 3)),
            // -3, 5, 8
            // The densest meteor shower ever recorded (1966)
            // ???
            // -63, 5, 13
            // SOS adopted as the international distress call (1906)
            // ??? 1906-11-03 wiki:https://en.wikipedia.org/wiki/SOS
            // Charles Darwin's The Origin of Species is published (1859)
            // 1859-11-24 https://en.wikipedia.org/wiki/On_the_Origin_of_Species
            (CommonDate::new(1859, 11, 24), CommonDate::new(-110, 5, 15)),
            // -23, 5, 28
            // Percy Spencer patents the microwave oven (1946)
            // ??? 1945-10-08 wiki:https://en.wikipedia.org/wiki/Percy_Spencer
            // Anethesia used for first time to perform a dental extraction (1844)
            //1844-12-10 https://en.wikipedia.org/wiki/History_of_general_anesthesia
            // (CommonDate::new(1844, 12, 10), CommonDate::new(-125, 6, 4)), // Incorrect??
            // -22, 6, 5
            // John Bardeen, Walter Brattain, and William Shockley invent the transistor (1947)
            // ??? 1947-(11-17 to 12-23) wiki:https://en.wikipedia.org/wiki/Transistor
            // The first airplane flight by Orville and Wilbur Wright, Kitty Hawk, North Carolina (1903)
            //1903-12-17 https://en.wikipedia.org/wiki/Wright_Flyer
            (CommonDate::new(1903, 12, 17), CommonDate::new(-66, 6, 10)),
            // Winter solstice (1989)
            //1989-12-21 https://aa.usno.navy.mil/calculated/seasons?year=1989&tz=0.00&tz_sign=-1&tz_label=false&dst=false&submit=Get+Data
            (CommonDate::new(1989, 12, 21), CommonDate::new(21, 6, 14)),
            // 6, 6, 17
            // The discovery of Lucy, fossil remains of an early female hominid, in Ethiopia (1974)
            // ???
            // New Year's Day on both the Gregorian Calendar (1990) and Japanese calendar (2651)
            // 1990-01-01 by definition
            (CommonDate::new(1990, 1, 1), CommonDate::new(21, 6, 25)),
            // 21, 6, 28
            // The earth is at its farthest distance from the sun (aphelion) (1990)
            // ??? 1990-01-04 is the perihelion wiki:https://aa.usno.navy.mil/calculated/seasons?year=1990&tz=0.00&tz_sign=-1&tz_label=false&dst=false&submit=Get+Data
            // Galileo discovers the moons of Jupiter (1610)
            // 1610-01-07 https://en.wikipedia.org/wiki/Galileo_Galilei
            (CommonDate::new(1610, 1, 7), CommonDate::new(-360, 7, 3)),
            // 16, 7, 9
            // Ornithologists count 1350 great white cranes at Poyand Lake in China, the most ever recorded (1985)
            // ???
            // Earthquake changes course of the Mississippi River (1812)
            // 1812-01-23 https://en.wikipedia.org/wiki/1811%E2%80%931812_New_Madrid_earthquakes
            (CommonDate::new(1812, 1, 23), CommonDate::new(-158, 7, 19)),
            // Annular eclipse of the sun (1990)
            // 1990-01-26 wiki:https://en.wikipedia.org/wiki/List_of_solar_eclipses_in_the_20th_century
            (CommonDate::new(1990, 1, 26), CommonDate::new(21, 7, 22)),
            // Apollo 1 fire kills US astronauts Gus Grissom, Ed White, and Roger Chaffee (1967)
            //1967-01-27 https://en.wikipedia.org/wiki/Apollo_1
            (CommonDate::new(1967, 1, 27), CommonDate::new(-3, 7, 23)),
            // The space shuttle Challenger explodes, killing seven American astronauts (1986)
            //1986-01-28 https://en.wikipedia.org/wiki/Space_Shuttle_Challenger_disaster
            (CommonDate::new(1986, 1, 28), CommonDate::new(17, 7, 24)),
            // Explorer 1, the first US satellite, is launched (1958)
            //1958-02-01 https://en.wikipedia.org/wiki/Explorer_1
            // (CommonDate::new(1958, 2, 1), CommonDate::new(-12, 7, 27)), //Incorrect??
            // Soviet Luna 9 makes first successful soft landing on the moon (1966)
            //1966-02-03 https://en.wikipedia.org/wiki/Luna_9
            (CommonDate::new(1966, 2, 3), CommonDate::new(-4, 8, 2)),
            // Two US astronauts become first humans to fly untethered in space (1984)
            //1984-02-07 https://en.wikipedia.org/wiki/STS-41-B
            // (CommonDate::new(1984, 2, 7), CommonDate::new(15, 8, 2)), //Incorrect??
            // Total lunar eclipse (1990)
            //1990-02-09 https://en.wikipedia.org/wiki/List_of_lunar_eclipses_in_the_20th_century
            // (CommonDate::new(1990, 2, 9), CommonDate::new(21, 8, 7)), //Incorrect??
            // -388, 8, 12
            // Pope Gregory corrects the Julian calendar (1582)
            // ??? 1582-10-15 wiki:https://en.wikipedia.org/wiki/Gregorian_calendar

            // Italian philosopher Giordano Bruno burned at the stake for his heliocentric views (1600)
            //1600-02-17 https://en.wikipedia.org/wiki/Giordano_Bruno
            (CommonDate::new(1600, 2, 17), CommonDate::new(-370, 8, 16)),
            // The planet Pluto is discovered by Clyde Tombaught (1930)
            //1930-02-18 https://en.m.wikipedia.org/wiki/Pluto
            (CommonDate::new(1930, 2, 18), CommonDate::new(-40, 8, 17)),
            // John Glenn aboard the Friendship 7, becomes the first American to orbit Earth (1962)
            //1962-02-20 https://en.wikipedia.org/wiki/Mercury-Atlas_6
            (CommonDate::new(1962, 2, 20), CommonDate::new(-8, 8, 19)),
            // Sir James Chadwick of Great Britain announces the discovery of the neutron (1932)
            //1932-02-27 https://web.mit.edu/22.54/resources/Chadwick.pdf
            (CommonDate::new(1932, 2, 27), CommonDate::new(-38, 8, 26)),
            // The launch of Pioneer 10, first known Earth object to leave solar system (1972)
            //1972-03-03 https://en.wikipedia.org/wiki/Pioneer_10
            // (CommonDate::new(1972, 3, 3), CommonDate::new(3, 9, 1)),
            // -189, 9, 5
            // Sir William Herschel discovers Uranus (1781)
            // ?? 1781-03-13 wiki:https://en.wikipedia.org/wiki/Uranus
            // -2013, 9, 14
            // The Ides of March, the day that Julius Caesar died (-44)
            // ?? Julian -44-03-15 wiki:https://en.wikipedia.org/wiki/Julius_Caesar
            // -44, 9, 15
            // Robert Goddard launches the first successful liquid-fuel rocket (1926)
            // ??

            // The US Congress authorizes conversion to standard time zones and daylight saving time (1918)
            // 1918-03-29 https://en.wikipedia.org/wiki/Standard_Time_Act
            // (CommonDate::new(1918, 3, 29), CommonDate::new(-12, 9, 18)), //Incorrect?
            // Vernal equinox. Serpent of light appears on the pyramid at Chichen Itza, Mexico (1990)
            // 1990-03-20 https://aa.usno.navy.mil/calculated/seasons?year=1990&tz=0.00&tz_sign=-1&tz_label=false&dst=false&submit=Get+Data
            (CommonDate::new(1990, 3, 20), CommonDate::new(21, 9, 19)),
            // Accident at Three Mile Island Nuclear Generating Station in Pennsylvania (1979)
            // 1979-03-28 https://en.wikipedia.org/wiki/Three_Mile_Island_accident
            (CommonDate::new(1979, 3, 28), CommonDate::new(10, 9, 27)),
            // Mariner 10 spacecraft approaches Mercury and sends 647 photos back to Earth (1974)
            // 1974-03-29 https://en.wikipedia.org/wiki/Mariner_10
            (CommonDate::new(1974, 3, 29), CommonDate::new(5, 9, 28)),
            // Samuel Morey patents the internal-combustion engine (1826)
            // 1826-04-01 https://www.ancientpages.com/2017/04/01/samuel-morey-patent/
            (CommonDate::new(1826, 4, 1), CommonDate::new(-144, 10, 3)),
            // First commerical communications satellite launched (US) (1965)
            // 1965-04-06 https://en.wikipedia.org/wiki/Communications_satellite
            // (CommonDate::new(1965, 4, 6), CommonDate::new(-5, 10, 6)), //Incorrect?
            // -61, 10, 8
            // Robert E. Peary claims discovery of the North Pole (1909)
            // ?? 1909-04-06?? -07?? wiki:https://en.wikipedia.org/wiki/Robert_Peary
            // Cosmonaut Yuri Gagarin of the USSR orbits Earth, becoming the first human in space (1961)
            // 1961-04-12 https://en.wikipedia.org/wiki/Vostok_1
            (CommonDate::new(1961, 4, 12), CommonDate::new(-9, 10, 14)),
            // 3, 10, 18
            // Two giant pandas, gifts from People's Republic of China, arrive at the National Zoo in Washington DC (1972)
            // ??
            // -284, 10, 19
            // Sir Isaac Newton presents Philosophiae naturalis principia mathematica to the Royal Society (1686)
            // ??
            // Francis Crick and James Watson report their discovery of the DNA double helix (1953)
            //1953-04-25 wiki:https://www.nature.com/articles/171737a0
            (CommonDate::new(1953, 04, 25), CommonDate::new(-17, 10, 27)),
            // Alan Shepard becomes the first American in space (1961)
            //1961-05-05 wiki:https://en.wikipedia.org/wiki/Mercury-Redstone_3
            (CommonDate::new(1961, 05, 05), CommonDate::new(-9, 11, 9)),
            // -174, 11, 18
            // Dr Edward Jenner conducts his first experiment with cow-pox vaccination (1796)
            // ??
            // Charles Lindbergh lands in Paris, becoming the first person to fly an airplane solo, nonstop across the Atlantic Ocean (1927)
            //1927-05-21 wiki:https://en.wikipedia.org/wiki/Spirit_of_St._Louis
            (CommonDate::new(1927, 05, 21), CommonDate::new(-43, 11, 25)),
            // The Concorde supersonic transport makes its first transatlantic flight to USA (1976)
            //1976-05-24 wiki:https://en.wikipedia.org/wiki/Concorde_operational_history
            (CommonDate::new(1976, 05, 24), CommonDate::new(7, 11, 28)),
            // -2554, 12, 4
            // Most famous ancient solar eclipse occurs during a battle between Lydians and Medes (585 BC)
            // ?? Julian -585-05-28 wiki:https://en.wikipedia.org/wiki/Eclipse_of_Thales
            // US launches the Mariner 9, first spacecraft to orbit another planet (1971)
            // 1971-05-30 https://en.wikipedia.org/wiki/Mariner_9
            (CommonDate::new(1971, 5, 30), CommonDate::new(2, 12, 6)),
            // Guglielmo Marconi is granted patent for the radio in Great Britain (1896)
            // 1896-06-02 https://en.wikipedia.org/wiki/Guglielmo_Marconi
            (CommonDate::new(1896, 6, 2), CommonDate::new(-74, 12, 9)),
            // Byron Allen pedals Gossamer Albatross aircraft across the English Channel (1979)
            // 1979-06-12 https://en.wikipedia.org/wiki/MacCready_Gossamer_Albatross
            (CommonDate::new(1979, 6, 12), CommonDate::new(10, 12, 19)),
            // 14, 12, 20
            // Pioneer 10 exits solar system (1983)
            // ???
            // Ben Franklin flies kite during a lightning storm and discovers electricity (1752)
            //1752-06-15 https://en.wikipedia.org/wiki/Benjamin_Franklin
            (CommonDate::new(1752, 6, 15), CommonDate::new(-218, 12, 22)),
            // Sally Ride becomes first US woman in space (1983)
            //1983-06-18 https://en.wikipedia.org/wiki/STS-7
            (CommonDate::new(1983, 6, 18), CommonDate::new(14, 12, 25)),
            // Summer solstice, longest day of year, Northern Hemisphere (1990)
            //1990-06-21 https://aa.usno.navy.mil/calculated/seasons?year=1990&tz=0.00&tz_sign=-1&tz_label=false&dst=false&submit=Get+Data
            (CommonDate::new(1990, 6, 21), CommonDate::new(21, 12, 28)),
            // First reported UFO sighting using the term flying saucers (1947)
            //1947-06-24 https://en.wikipedia.org/wiki/Kenneth_Arnold_UFO_sighting
            (CommonDate::new(1947, 6, 24), CommonDate::new(-23, 13, 3)),
            // -13, 13, 4
            // CBS broadcasts first commercial color TV program (1957)
            // ???
            // Mysterious explosion devastates a huge forest in Tunguska, Siberia (1908)
            // 1908-06-30 https://en.wikipedia.org/wiki/Tunguska_event
            (CommonDate::new(1908, 6, 30), CommonDate::new(-62, 13, 9)),
            // -83, 13, 15
            // Louis Pasteur inoculates a boy with antirabies serum (1887)
            // ???
            // Skylab falls to Earth (1979)
            // 1979-07-11 https://en.wikipedia.org/wiki/Skylab
            (CommonDate::new(1979, 7, 11), CommonDate::new(10, 13, 20)),
            // -8, 13, 22
            // First transatlantic conversation using communications satellite (1962)
            // ??? wiki:https://en.wikipedia.org/wiki/Telstar
            // First atomic bomb is detonated, Trinity Site, New Mexico (1945)
            // 1945-07-16 https://en.wikipedia.org/wiki/Trinity_(nuclear_test)
            (CommonDate::new(1945, 7, 16), CommonDate::new(-25, 13, 25)),
        ];
        for pair in d_list {
            let dg = Gregorian::try_from_common_date(pair.0).unwrap().to_fixed();
            let dq = TranquilityMoment::try_from_common_date(pair.1)
                .unwrap()
                .to_fixed();
            assert_eq!(dq.get_day_i(), dg.get_day_i(), "{:?}", pair);
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
            assert_eq!(dq.year(), pair.1);
        }
    }

    proptest! {
        #[test]
        fn gregorian_lookup(t in FIXED_MIN..FIXED_MAX) {
            // https://web.archive.org/web/20180818233025/https://en.wikipedia.org/wiki/Tranquility_calendar
            let f = RataDie::new(t).to_fixed().to_day();
            let g = Gregorian::from_fixed(f);
            let gc = g.to_common_date();
            let q = TranquilityMoment::from_fixed(f);
            if q.try_month().is_some() {
                let qm = q.try_month().unwrap();
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
            if q.try_month().is_some() {
                let qm = q.try_month().unwrap();
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
