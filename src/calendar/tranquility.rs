// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::calendar::gregorian::Gregorian;
use crate::calendar::prelude::CommonDate;
use crate::calendar::prelude::HasLeapYears;
use crate::calendar::prelude::OrdinalDate;
use crate::calendar::prelude::Perennial;
use crate::calendar::prelude::Quarter;
use crate::calendar::prelude::ToFromCommonDate;
use crate::calendar::prelude::ToFromOrdinalDate;
use crate::calendar::CalendarMoment;
use crate::calendar::HasEpagemonae;
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

/// Represents a date in the Tranquility Calendar
///
/// ## Introduction
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
/// present in other timekeeping systems, and some aspects of the calendar are poorly documented;
/// this can cause difficulty when implementing software applications.
///
/// ### Year 0
///
/// Year 0 is **not** supported for this implementation of the Tranquility calendar, except as
/// a dummy value for the epoch. See the "Epoch" and "The Curious Case of 1 Before Tranquility"
/// sections for details.
///
/// ## Basic Structure
///
/// The epoch of the Tranquility calendar, known as Moon Landing Day, is an exception to
/// every other rule of the calendar. This also requires some special cases for the year
/// immediately before the epoch, 1 BT. See the "Epoch" section for details. The remainder
/// of this section is about days other than Moon Landing Day.
///
/// ### Months
///
/// From Siggins' article proposing the calendar:
/// > The Tranquility calendar uses a format of 13 months, each with 28 days (four seven-day
/// > weeks), for a total of 364 days. One extra day is added on at the end of the year to make
/// > 365. For leap years, a second extra day is added.
///
/// The months of the Tranquility calendar are named after notable scientists, and are also
/// in alphabetical order. Note that the "second extra day" for leap years is **not** at the
/// end of the year.
///
/// The first day of every Tranquility year is 1 Archimedes, as stated by Siggins:
/// >  The first day of each year is the first day of Archimedes (July 21 on the Gregorian calendar).
///
/// ### Armstrong Day and Aldrin Day
///
/// The two extra days are described as follows by Siggins:
/// > The last day of each Tranquility year is called Armstrong Day. Named for the first human
/// > to step on the moon, Armstrong Day celebrates the anniversary of Moon Landing Day. Thus
/// > July 20, 1970, is the Gregorian equivalent of Armstrong Day, 1 A.T., and July 20, 1989,
/// > is Armstrong Day, 20 A.T.
/// >
/// > Finally, the Tranquility calendar equivalent of leap day is called Aldrin Day. Named for
/// > Armstrong's fellow moon walker, Aldrin Day occurs every four years, with some exceptions.
/// > The exceptions are the leap days in every 400 years that are dropped to keep the calendar
/// > astronomically precise. Aldrin Day falls between the twenty-seventh and twenty-eighth of
/// > Hippocrates (February 29).
///
/// Siggins explains the Gregorian leap year rule earlier in the article, so we can assume that
/// "the leap days in every 400 years that are dropped" refer to the leap days dropped in
/// Gregorian years divisible by 100 and not by 400.
///
/// The odd placement of Aldrin Day is not explicitly justified in the text. However, implicit
/// in the passage above is the idea that *every* Armstrong Day corresponds to July 20 in the
/// proleptic Gregorian calendar, and *every* Aldrin Day corresponds to February 29. The year
/// lengths are also the same, meaning that there should be a one-to-one relationship between
/// Gregorian dates and Tranquility dates stable across all years (ignoring Moon Landing Day,
/// which replaces one specific Armstrong Day).
///
/// This would also imply that when calculating leap years, we need to apply the Gregorian
/// rule after applying an offset of 1969 or 1970, depending on the date relative to the epoch.
///
/// ## Epoch
///
/// The Tranquility calendar epoch is specific down to the tenth of a second. The day on which
/// the epoch occurs is called Moon Landing Day, and is described as follows by Siggins:
/// > The Tranquility calendar, on the other hand, is based on a recent, well-documented event,
/// > the landing by two American astronauts, Neil Armstrong and Edwin "Buzz" Aldrin, on the
/// > moon. Upon touchdown came those almost mystical words, "Houston ... Tranquility Base here.
/// > The Eagle has landed." Omni's Tranquility calendar starts at the very instant the word
/// > Tranquility was uttered. As one of the most astronomically analyzed moments in scientific
/// > history, our base point can be used to chart the exact position of the earth in
/// > relationship to the moon and other celestial bodies.
/// >
/// > Now for the details of the calendar itself. The day on which the moment of Tranquility
/// > occurred is called Moon Landing Day. It is the central day of the Tranquility calendar
/// > and it stands alone. Not part of any month or year, it has 20 hours, 18 minutes, and 1.2
/// > seconds Before Tranquility (B.T.), and 3 hours, 41 minutes, and 58.8 seconds After
/// > Tranquility (A.T). The Gregorian equivalent of Moon Landing Day is July 20, A.D. 1969.
///
/// We can assume that any date after Moon Landing Day is "After Tranquility" and any day before
/// Moon Landing Day is "Before Tranquility".
///
/// ### The Curious Case of 1 Before Tranquility
///
/// Siggins devoted very little of his article to dates Before Tranquility, and the year
/// immediately preceding Moon Landing Day is particularly confusing.
///
/// Consider the following statements in Siggins' article:
///
/// 1. "The Gregorian equivalent of Moon Landing Day is July 20, A.D. 1969"
/// 2. "The first day of each year is the first day of Archimedes (July 21 on the Gregorian calendar)."
/// 3. "The last day of each Tranquility year is called Armstrong Day."
/// 4. "Armstrong Day celebrates the anniversary of Moon Landing Day."
///
///    a. "July 20, 1970, is the Gregorian equivalent of Armstrong Day, 1 A.T."
///
///    b. "July 20, 1989, is Armstrong Day, 20 A.T."
///
/// First of all, it is not stated whether the year immediately preceding Moon Landing Day is
/// 1 BT or 0 BT - however the full version of Siggins' article in OMNI (with all the
/// images) contains many example dates which imply that 0 BT does not exist. For example,
/// the first airplane flight by Orville and Wilbur Wright at Kitty Hawk is listed as occurring
/// on Faraday 10, 66 BT and 1903 CE. [The exact date of the flight was December 17, 1903](https://airandspace.si.edu/collection-objects/1903-wright-flyer/nasm_A19610048000).
/// The 66th anniversary of the Wright brother's flight would be December 17, 1969 CE which
/// is after Moon Landing Day (as per statement 1 above) and before Armstrong Day, 1 AT (as
/// per statement 4a above) which is the last day of 1 AT (as per statement 3). However,
/// naively calculating 66 BT + 66 = 0 BT would suggest that the year should be 0 BT.
///
/// Based on the math above, we can assume some year BT was skipped, and the most obvious
/// choice is 0 BT. Skipping some other year such as 1 BT, 2 BT or 50 BT would be
/// unintuitive, whereas skipping year 0 is common practice when working with historical dates
/// using the Julian calendar.
///
/// Having established that the year immediately preceding Moon Landing Day is 1 BT, we must
/// figure out what the last day of 1 BT is named.
///
/// All 4 statements above cannot be true for 1 BT, as Armstrong Day, 1 BT would
/// either conflict with Moon Landing Day, not end the year, or not occur on the anniversary
/// of Moon Landing Day. Any implementation of the Tranquility calendar must implement some
/// special case to deal with this situation.
///
/// **This crate assumes that Armstrong Day is skipped in the year 1 BT** The day that
/// would normally be Armstrong Day, 1 BT is Moon Landing Day. The day before Moon Landing
/// Day is Mendel 28, 1 BT - which *would* be the day before Armstrong Day, 1 BT *if*
/// Armstrong Day existed in 1 BT.
///
/// This also seems to be the approach taken by the Orion's Arm Calendar Converter (note that
/// other pages in the Orion's Arm project may differ from the converter).
///
/// ## Representation and Examples
///
/// ### Months
///
/// The months are represented in this crate as [`TranquilityMonth`].
///
/// ```
/// use radnelac::calendar::*;
/// use radnelac::day_count::*;
///
/// let c_1_1 = CommonDate::new(57, 1, 1);
/// let tq_1_1 = Tranquility::try_from_common_date(c_1_1).unwrap();
/// assert_eq!(tq_1_1.try_month().unwrap(), TranquilityMonth::Archimedes);
/// let c_13_28 = CommonDate::new(57, 13, 28);
/// let tq_13_28 = Tranquility::try_from_common_date(c_13_28).unwrap();
/// assert_eq!(tq_13_28.try_month().unwrap(), TranquilityMonth::Mendel);
/// ```
///
/// ### Weekdays
///
/// The days of the Tranquility week are not always the same as the days of the common week.
///
/// ```
/// use radnelac::calendar::*;
/// use radnelac::day_count::*;
/// use radnelac::day_cycle::*;
///
/// let c = CommonDate::new(-1, 13, 28);
/// let p = Tranquility::try_from_common_date(c).unwrap();
/// assert_eq!(p.weekday().unwrap(), Weekday::Thursday); //Positivist week
/// assert_eq!(p.convert::<Weekday>(), Weekday::Saturday); //Common week
/// ```
///
/// ### Armstrong Day and Aldrin Day
///
/// The epagomenal days of a Tranquility year are represented as [`TranquilityComplementaryDay`].
/// When converting to and from a [`CommonDate`](crate::calendar::CommonDate), the epagomenal
/// days are treated as month 0.
///
/// ```
/// use radnelac::calendar::*;
/// use radnelac::day_count::*;
///
/// let c_arm = CommonDate::new(31, 0, 1);
/// let tq_arm = Tranquility::try_from_common_date(c_arm).unwrap();
/// assert!(tq_arm.try_month().is_none());
/// assert_eq!(tq_arm.epagomenae().unwrap(), TranquilityComplementaryDay::ArmstrongDay);
/// assert!(tq_arm.weekday().is_none());
///
/// let c_ald = CommonDate::new(31, 0, 2);
/// let tq_ald = Tranquility::try_from_common_date(c_ald).unwrap();
/// assert!(tq_ald.try_month().is_none());
/// assert_eq!(tq_ald.epagomenae().unwrap(), TranquilityComplementaryDay::AldrinDay);
/// assert!(tq_ald.weekday().is_none());
/// ```
///
/// Armstrong Day is at the end of a year (except the year 1 BT).
///
/// ```
/// use radnelac::calendar::*;
/// use radnelac::day_count::*;
///
/// let tq_end_31 = Tranquility::try_year_end(31).unwrap();
/// assert_eq!(tq_end_31.epagomenae().unwrap(), TranquilityComplementaryDay::ArmstrongDay);
/// let tq_end_1 = Tranquility::try_year_end(-1).unwrap();
/// assert!(tq_end_1.epagomenae().is_none());
/// ```
///
/// Aldrin Day is between 27 and 28 Hippocrates in a leap year. Note that leap years occur
/// at the same time as proleptic Gregorian leap years and Aldrin Day is always February 28
/// in the proleptic Gregorian calendar.
///
/// ```
/// use radnelac::calendar::*;
/// use radnelac::day_count::*;
///
/// let g_list = [
///     Gregorian::try_new(2024, GregorianMonth::February, 28).unwrap(),
///     Gregorian::try_new(2024, GregorianMonth::February, 29).unwrap(),
///     Gregorian::try_new(2024, GregorianMonth::March, 1).unwrap(),
/// ];
///
/// let tq_list = [
///     g_list[0].convert::<Tranquility>(),
///     g_list[1].convert::<Tranquility>(),
///     g_list[2].convert::<Tranquility>(),
/// ];
///
/// // Before Aldrin Day
/// assert_eq!(tq_list[0].year(), 55);
/// assert_eq!(tq_list[0].day(), 27);
/// assert_eq!(tq_list[0].try_month().unwrap(), TranquilityMonth::Hippocrates);
/// // Aldrin Day
/// assert_eq!(tq_list[1].year(), 55);
/// assert_eq!(tq_list[1].epagomenae().unwrap(), TranquilityComplementaryDay::AldrinDay);
/// assert!(tq_list[1].try_month().is_none());
/// // After Aldrin Day
/// assert_eq!(tq_list[2].year(), 55);
/// assert_eq!(tq_list[2].day(), 28);
/// assert_eq!(tq_list[2].try_month().unwrap(), TranquilityMonth::Hippocrates);
/// ```
///
/// ### Moon Landing Day
///
/// Moon Landing Day technically does not have any year, month or weekday. However it is
/// sometimes represented using the dummy year and month value `0`. The corresponding Gregorian
/// date is July 20, 1969.
///
/// ```
/// use radnelac::calendar::*;
/// use radnelac::day_count::*;
///
/// let g = Gregorian::try_new(1969, GregorianMonth::July, 20).unwrap();
/// let tq = g.convert::<Tranquility>();
/// assert_eq!(tq.year(), 0);
/// assert_eq!(tq.epagomenae().unwrap(), TranquilityComplementaryDay::MoonLandingDay);
/// assert!(tq.try_month().is_none());
/// assert!(tq.weekday().is_none());
/// assert_eq!(tq.to_common_date(), CommonDate::new(0, 0, 0));
/// ```
///
/// Moon Landing Day is between Mendel 28, 1 BT and Archimedes 1, 1 AT.
///
/// ```
/// use radnelac::calendar::*;
/// use radnelac::day_count::*;
///
/// let g_list = [
///     Gregorian::try_new(1969, GregorianMonth::July, 19).unwrap(),
///     Gregorian::try_new(1969, GregorianMonth::July, 20).unwrap(),
///     Gregorian::try_new(1969, GregorianMonth::July, 21).unwrap(),
/// ];
///
/// let tq_list = [
///     g_list[0].convert::<Tranquility>(),
///     g_list[1].convert::<Tranquility>(),
///     g_list[2].convert::<Tranquility>(),
/// ];
///
/// // Before Moon Landing Day
/// assert_eq!(tq_list[0].year(), -1);
/// assert_eq!(tq_list[0].day(), 28);
/// assert_eq!(tq_list[0].try_month().unwrap(), TranquilityMonth::Mendel);
/// // Moon Landing Day
/// assert_eq!(tq_list[1].epagomenae().unwrap(), TranquilityComplementaryDay::MoonLandingDay);
/// // After Moon Landing Day
/// assert_eq!(tq_list[2].year(), 1);
/// assert_eq!(tq_list[2].day(), 1);
/// assert_eq!(tq_list[2].try_month().unwrap(), TranquilityMonth::Archimedes);
/// ```
///
/// The epoch of the Tranquility calendar is specific to the tenth of a second. To check if
/// a specific point in time is before or after the epoch, callers should use [`TranquilityMoment`].
///
/// ## Inconsistencies with Other Implementations
///
/// The assumptions regarding 0 BT and 1 BT could differ from other implementations.
///
/// ## Further reading
/// + Orion's Arm
///   + ["Encyclopaedia Galactica"](https://www.orionsarm.com/eg-article/48c6d4c3d54cf/)
///   + [Calendar Converter](https://www.orionsarm.com/xcms.php?r=oa-calendar-converter)
/// + [Wikipedia Deletion Log](https://en.wikipedia.org/wiki/Wikipedia:Articles_for_deletion/Tranquility_Calendar)
/// + [Archived Wikipedia article](https://web.archive.org/web/20180818233025/https://en.wikipedia.org/wiki/Tranquility_calendar)
/// + Archived copies of Jeff Siggins' article for OMNI
///   + [archive.org copy of mithrandir.com](https://web.archive.org/web/20161025042320/http://www.mithrandir.com/Tranquility/tranquilityArticle.html)
///   + [archive.org copy of OMNI July 1989, pages 63, 64](https://archive.org/details/omni-archive/OMNI_1989_07/page/n63/mode/2up)
///   + [archive.org copy of OMNI July 1989, pages 65, 66](https://archive.org/details/omni-archive/OMNI_1989_07/page/n65/mode/2up)
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Tranquility(CommonDate);

impl Tranquility {
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

impl ToFromOrdinalDate for Tranquility {
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
        let comp_count = Self::epagomenae_count(self.0.year) as i64;
        let ordinal_day = match self.epagomenae() {
            Some(TranquilityComplementaryDay::MoonLandingDay) => 1,
            Some(TranquilityComplementaryDay::ArmstrongDay) => 364 + comp_count,
            Some(TranquilityComplementaryDay::AldrinDay) => AFTER_H27,
            None => {
                let month = self.0.month as i64;
                let day = self.0.day as i64;
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
            year: self.0.year,
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
        Tranquility(date_tq)
    }
}

impl PartialOrd for Tranquility {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else {
            let self_ord = self.to_ordinal();
            let other_ord = other.to_ordinal();
            self_ord.partial_cmp(&other_ord)
        }
    }
}

impl HasEpagemonae<TranquilityComplementaryDay> for Tranquility {
    fn epagomenae(self) -> Option<TranquilityComplementaryDay> {
        if self.0.month == NON_MONTH {
            TranquilityComplementaryDay::from_u8(self.0.day)
        } else {
            None
        }
    }

    fn epagomenae_count(p_year: i32) -> u8 {
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

impl Perennial<TranquilityMonth, Weekday> for Tranquility {
    fn weekday(self) -> Option<Weekday> {
        if self.epagomenae().is_some() {
            None
        } else {
            Weekday::from_i64(((self.0.day as i64) + 4).modulus(7))
        }
    }

    fn days_per_week() -> u8 {
        7
    }

    fn weeks_per_month() -> u8 {
        4
    }
}

impl HasLeapYears for Tranquility {
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

impl CalculatedBounds for Tranquility {}

impl Epoch for Tranquility {
    fn epoch() -> Fixed {
        let date = Gregorian::try_from_common_date(TRANQUILITY_EPOCH_GREGORIAN)
            .expect("Epoch known to be valid")
            .to_fixed();
        let time = TimeOfDay::try_from_clock(TRANQUILITY_EPOCH_CLOCK).expect("Known valid");
        Fixed::new(date.get() + time.get())
    }
}

impl FromFixed for Tranquility {
    fn from_fixed(date: Fixed) -> Tranquility {
        let ord = Tranquility::ordinal_from_fixed(date);
        Tranquility::from_ordinal_unchecked(ord)
    }
}

impl ToFixed for Tranquility {
    fn to_fixed(self) -> Fixed {
        let ord = self.to_ordinal();
        let offset_prior = Tranquility::prior_elapsed_days(ord.year);
        Fixed::new((offset_prior as f64) + (ord.day_of_year as f64))
    }
}

impl ToFromCommonDate<TranquilityMonth> for Tranquility {
    fn to_common_date(self) -> CommonDate {
        self.0
    }

    fn from_common_date_unchecked(date: CommonDate) -> Self {
        debug_assert!(Self::valid_ymd(date).is_ok());
        Self(date)
    }

    fn valid_ymd(date: CommonDate) -> Result<(), CalendarError> {
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

    fn year_start_date(year: i32) -> CommonDate {
        if year == 0 {
            CommonDate::new(
                year,
                NON_MONTH,
                TranquilityComplementaryDay::MoonLandingDay as u8,
            )
        } else {
            CommonDate::new(year, 1, 1)
        }
    }

    fn year_end_date(year: i32) -> CommonDate {
        if year == 0 {
            CommonDate::new(
                year,
                NON_MONTH,
                TranquilityComplementaryDay::MoonLandingDay as u8,
            )
        } else if year == -1 {
            CommonDate::new(year, TranquilityMonth::Mendel as u8, 28)
        } else {
            CommonDate::new(
                year,
                NON_MONTH,
                TranquilityComplementaryDay::ArmstrongDay as u8,
            )
        }
    }

    fn month_length(_year: i32, _month: TranquilityMonth) -> u8 {
        28
    }
}

impl Quarter for Tranquility {
    fn quarter(self) -> NonZero<u8> {
        match (self.try_week_of_year(), self.epagomenae()) {
            (None, Some(TranquilityComplementaryDay::MoonLandingDay)) => NonZero::new(4).unwrap(),
            (None, Some(TranquilityComplementaryDay::ArmstrongDay)) => NonZero::new(4).unwrap(),
            (None, Some(TranquilityComplementaryDay::AldrinDay)) => NonZero::new(3).unwrap(),
            (Some(w), None) => NonZero::new((w - 1) / 13 + 1).expect("w > 0"),
            (_, _) => unreachable!(),
        }
    }
}

/// Represents a date *and time* in the Tranquility Calendar
pub type TranquilityMoment = CalendarMoment<Tranquility>;

impl TranquilityMoment {
    pub fn is_after_tranquility(self) -> bool {
        if self.date().0.year == 0 {
            self.time_of_day() > TRANQUILITY_EPOCH_CLOCK
        } else {
            self.date().0.year > 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::calendar::Julian;
    use crate::day_count::RataDie;

    use crate::day_count::FIXED_MAX;
    use crate::day_count::FIXED_MIN;

    use proptest::proptest;

    #[test]
    fn moon_landing_edge_cases() {
        let f0 = TranquilityMoment::epoch();
        let q0 = TranquilityMoment::from_fixed(f0);
        let c = CommonDate::new(0, 0, 0);
        assert_eq!(q0.to_common_date(), c);
        let f1 = q0.to_fixed();
        assert_eq!(f0, f1);
        assert_eq!(c, TranquilityMoment::year_end_date(0));
        assert_eq!(c, TranquilityMoment::year_start_date(0));
    }

    #[test]
    fn one_bt_edge_cases() {
        let c = CommonDate::new(-1, 13, 28);
        assert_eq!(c, TranquilityMoment::year_end_date(-1));
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
            // Pliny the Elder dies in eruption of Vesuvius at Pompeii (79)
            // 79-08-24 https://en.wikipedia.org/wiki/Eruption_of_Mount_Vesuvius_in_79_AD
            // Actual date seems controversial, additionally shouldn't this be Julian?
            (CommonDate::new(79, 8, 24), CommonDate::new(-1890, 2, 7)),
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
            (CommonDate::new(1492, 10, 12), CommonDate::new(-477, 3, 28)), //Apparently not Julian
            // USAF Chaptain Charles Yeager becomes first human to fly faster than the speed of sound
            // 1947-10-14 https://en.wikipedia.org/wiki/Bell_X-1
            (CommonDate::new(1947, 10, 14), CommonDate::new(-22, 4, 2)),
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
            (CommonDate::new(1922, 11, 4), CommonDate::new(-47, 4, 23)),
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
            // Launch date instead of untethered spacewalk date!
            (CommonDate::new(1984, 2, 3), CommonDate::new(15, 8, 2)),
            // Total lunar eclipse (1990)
            //1990-02-09 https://en.wikipedia.org/wiki/List_of_lunar_eclipses_in_the_20th_century
            // (CommonDate::new(1990, 2, 9), CommonDate::new(21, 8, 7)), //Incorrect??
            // Pope Gregory corrects the Julian calendar (1582) (See Julian test)
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
            // (CommonDate::new(1972, 3, 3), CommonDate::new(3, 9, 1)), //Incorrect?
            // Sir William Herschel discovers Uranus (1781)
            // 1781-03-13 wiki:https://en.wikipedia.org/wiki/Uranus
            // (CommonDate::new(1781, 3, 13), CommonDate::new(-189, 9, 5)), //Incorrect?
            // The Ides of March, the day that Julius Caesar died (-44) (See Julian test)
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
            // Robert E. Peary claims discovery of the North Pole (1909)
            // 1909-04-06 wiki:https://en.wikipedia.org/wiki/Robert_Peary
            (CommonDate::new(1909, 4, 6), CommonDate::new(-61, 10, 8)),
            // Cosmonaut Yuri Gagarin of the USSR orbits Earth, becoming the first human in space (1961)
            // 1961-04-12 https://en.wikipedia.org/wiki/Vostok_1
            (CommonDate::new(1961, 4, 12), CommonDate::new(-9, 10, 14)),
            // Two giant pandas, gifts from People's Republic of China, arrive at the National Zoo in Washington DC (1972)
            // 1972-04-16 wiki:https://en.wikipedia.org/wiki/Ling-Ling_and_Hsing-Hsing
            (CommonDate::new(1972, 4, 16), CommonDate::new(3, 10, 18)),
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
            // Most famous ancient solar eclipse occurs during a battle between Lydians and Medes (585 BC) (see Julian test)
            // -585-05-28 wiki:https://en.wikipedia.org/wiki/Eclipse_of_Thales
            // You would think this is a Julian date but no, this is a Gregorian date without year 0
            (CommonDate::new(-584, 5, 28), CommonDate::new(-2554, 12, 4)),
            // US launches the Mariner 9, first spacecraft to orbit another planet (1971)
            // 1971-05-30 https://en.wikipedia.org/wiki/Mariner_9
            (CommonDate::new(1971, 5, 30), CommonDate::new(2, 12, 6)),
            // Guglielmo Marconi is granted patent for the radio in Great Britain (1896)
            // 1896-06-02 https://en.wikipedia.org/wiki/Guglielmo_Marconi
            (CommonDate::new(1896, 6, 2), CommonDate::new(-74, 12, 9)),
            // Byron Allen pedals Gossamer Albatross aircraft across the English Channel (1979)
            // 1979-06-12 https://en.wikipedia.org/wiki/MacCready_Gossamer_Albatross
            (CommonDate::new(1979, 6, 12), CommonDate::new(10, 12, 19)),
            // Pioneer 10 exits solar system (1983)
            // 1983-06-13 https://en.wikipedia.org/wiki/Pioneer_10
            (CommonDate::new(1983, 6, 13), CommonDate::new(14, 12, 20)),
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
            // 1962-07-23 wiki:https://en.wikipedia.org/wiki/Telstar
            // (CommonDate::new(1962, 7, 23), CommonDate::new(-8, 13, 22)), //Incorrect?
            // First atomic bomb is detonated, Trinity Site, New Mexico (1945)
            // 1945-07-16 https://en.wikipedia.org/wiki/Trinity_(nuclear_test)
            (CommonDate::new(1945, 7, 16), CommonDate::new(-25, 13, 25)),
        ];
        for pair in d_list {
            let dg = Gregorian::try_from_common_date(pair.0).unwrap().to_fixed();
            let dq = TranquilityMoment::try_from_common_date(pair.1)
                .unwrap()
                .to_fixed();
            assert_eq!(dg.get_day_i(), dq.get_day_i(), "{:?}", pair);
        }
    }

    // #[test]
    // fn article_examples_julian() {
    //     let d_list = [
    //         // From Siggins' article
    //         // Pope Gregory corrects the Julian calendar (1582)
    //         // 1582-02-24 (Julian) wiki:https://en.wikipedia.org/wiki/Inter_gravissimas
    //         // Off by 23, that's pretty bad
    //         (CommonDate::new(1582, 2, 3), CommonDate::new(-388, 8, 12)),
    //         // The Ides of March, the day that Julius Caesar died (-44)
    //         // -44-03-15 (Julian) wiki:https://en.wikipedia.org/wiki/Julius_Caesar
    //         // Off by 2, could this be caused by using a proleptic Julian calendar?
    //         (CommonDate::new(-44, 3, 17), CommonDate::new(-2013, 9, 14)),
    //     ];
    //     for pair in d_list {
    //         let dg = Julian::try_from_common_date(pair.0).unwrap().to_fixed();
    //         let dq = TranquilityMoment::try_from_common_date(pair.1)
    //             .unwrap()
    //             .to_fixed();
    //         assert_eq!(dg.get_day_i(), dq.get_day_i(), "{:?}", pair);
    //     }
    // }

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
                let qc = q.epagomenae().unwrap();
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
                let qc = q.epagomenae().unwrap();
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
