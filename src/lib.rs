// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Calculations in a variety of different timekeeping systems.
//!
//! ## Introduction
//!
//! This is a crate for calendrical calculations: given a day represented in
//! one timekeeping system, this crate can create the representation for the same
//! day in another timekeeping system.
//!
//! Additionally, the crate can convert dates to strings in some predefined formats.
//!
//! For example, here is a conversion from the Gregorian calendar to the Julian:
//!
//! ```
//! use radnelac::calendar::*;
//! use radnelac::day_count::*;
//!
//! let g = Gregorian::try_new(2025, GregorianMonth::July, 26).unwrap();
//! let j = g.convert::<Julian>();
//! assert_eq!(j, Julian::try_new(2025, JulianMonth::July, 13).unwrap());
//! ```
//!
//! ## Crate Features
//!
//! Some functionality in this crate can be enabled and disabled using the Cargo "features"
//! mechanism. Applications should disable features they are not using to reduce the number
//! of dependencies, size of binaries and time spent compiling.
//!
//! The following feature is available:
//!
//! - `display` (*enabled by default*): implements [std::fmt::Display] and string conversion for all supported timekeeping systems
//!
//! ## Limitations
//!
//! ### Out-of-Scope Functionality
//!
//! This crate is focused on *calendrical* calculations involving days, weeks, months
//! and years. Measurements of time more precise than a day are not usable by most
//! functions or data structures.
//!
//! As such, there many time-related features that are out-of-scope:
//! - anything involving time zones
//! - anything involving daylight saving time
//! - anything involving leap seconds
//! - anything involving benchmarking or profiling code
//! - anything involving millisecond or nanosecond precision
//!
//! There are also many calendar-related features that would be potentially within-scope,
//! but not supported yet:
//! - predicting astronomical events (ex. lunar phases, eclipses, equinoxes)
//! - astronomical calendars (ex. Chinese Lunar calendar)
//! - arranging days in a grid
//! - parsing dates
//! - reading and writing CalDAV
//!
//! ### Proleptic Dates
//!
//! Calendars are assumed to be **proleptic**. Wiktionary defines proleptic[^1] as:
//! > Extrapolated to dates prior to its first adoption; of those used to adjust to
//! > or from the Julian calendar or Gregorian calendar.
//!
//! This can lead to confusion when using a calendar before the date of its adoption.
//!
//! For example, when the Gregorian calendar was introduced in the Papal States in 1582,
//! it replaced the Julian calendar. During the transition, 10 days were officially skipped
//! in the Papal States, so that October 4th was immediately followed by October 15th.
//!
//! This crate does **not** implement such skipping. When using the Gregorian calendar
//! functions in this crate, October 4th is always followed by October 5th. To work with
//! historical dates before the Gregorian reform, applications must explicitly switch to
//! the Julian calendar (or whatever other calendar is appropriate).
//!
//! Explicitly switching between calendars makes sense for applications, because the
//! Gregorian reform was implemented at different times in different regions.
//!
//! ### Year Zero and Negative Years
//!
//! Additionally most calendars, including the Gregorian, are assumed to have a Year 0. One
//! notable exception is the Julian. If a calendar does not allow Year 0, this property is
//! mentioned on that calendar's page in the documentation.
//!
//! The following is a quotation from Chapter 1.16 of *Calendrical Calculations: The Ultimate
//! Edition* by Reingold & Dershowitz, which applies quite well to this situation.
//! > All our functions give "correct" (mathematically sensible) results for negative
//! > years and for dates prior to the epoch of a calendar. However, these results may be
//! > *culturally* wrong in the sense that, say, the Copts may not refer to a year 0 or -1.
//! > It may be considered heretical on some calendars to refer to dates before the creation
//! > of the world.
//!
//! [^1]: <https://en.wiktionary.org/wiki/proleptic>
#[macro_use]
extern crate num_derive;

/// Timekeeping systems which focus on events within a single day
pub mod clock {
    mod time_of_day;

    pub use time_of_day::ClockTime;
    pub use time_of_day::TimeOfDay;
}
mod common {
    pub mod error;
    pub mod math;
}
pub use common::error::CalendarError;
/// Timekeeping systems which identify a day using a single field
pub mod day_count {
    mod prelude;

    mod fixed;
    mod jd;
    mod mjd;
    mod rd;
    mod unix;

    pub use prelude::*;

    pub use fixed::CalculatedBounds;
    pub use fixed::Epoch;
    pub use fixed::Fixed;
    pub use fixed::FromFixed;
    pub use fixed::ToFixed;
    pub use fixed::FIXED_MAX;
    pub use fixed::FIXED_MIN;
    pub use jd::JulianDay;
    pub use mjd::ModifiedJulianDay;
    pub use rd::RataDie;
    pub use unix::UnixMoment;
}
/// Timekeeping systems which continually repeat
pub mod day_cycle {
    mod prelude;

    mod akan;
    mod week;

    pub use prelude::*;

    pub use akan::Akan;
    pub use akan::AkanPrefix;
    pub use akan::AkanStem;
    pub use week::Weekday;
}
/// Timekeeping systems which identify a day using multiple fields
pub mod calendar {
    mod moment;
    mod prelude;

    mod armenian;
    mod coptic;
    mod cotsworth;
    mod egyptian;
    mod ethiopic;
    mod french_rev_arith;
    mod gregorian;
    mod holocene;
    mod iso;
    mod julian;
    mod olympiad;
    mod positivist;
    mod roman;
    mod symmetry;
    mod tranquility;

    pub use moment::CalendarMoment;
    pub use prelude::*;

    pub use armenian::Armenian;
    pub use armenian::ArmenianDaysOfMonth;
    pub use armenian::ArmenianMoment;
    pub use armenian::ArmenianMonth;
    pub use coptic::Coptic;
    pub use coptic::CopticMoment;
    pub use coptic::CopticMonth;
    pub use cotsworth::Cotsworth;
    pub use cotsworth::CotsworthComplementaryDay;
    pub use cotsworth::CotsworthMoment;
    pub use cotsworth::CotsworthMonth;
    pub use egyptian::Egyptian;
    pub use egyptian::EgyptianDaysUponTheYear;
    pub use egyptian::EgyptianMoment;
    pub use egyptian::EgyptianMonth;
    pub use ethiopic::Ethiopic;
    pub use ethiopic::EthiopicMoment;
    pub use ethiopic::EthiopicMonth;
    pub use french_rev_arith::FrenchRevArith;
    pub use french_rev_arith::FrenchRevArithMoment;
    pub use french_rev_arith::FrenchRevMonth;
    pub use french_rev_arith::FrenchRevWeekday;
    pub use french_rev_arith::Sansculottide;
    pub use gregorian::Gregorian;
    pub use gregorian::GregorianMoment;
    pub use gregorian::GregorianMonth;
    pub use holocene::Holocene;
    pub use holocene::HoloceneMoment;
    pub use holocene::HoloceneMonth;
    pub use iso::ISOMoment;
    pub use iso::ISO;
    pub use julian::Julian;
    pub use julian::JulianMoment;
    pub use julian::JulianMonth;
    pub use olympiad::Olympiad;
    pub use positivist::Positivist;
    pub use positivist::PositivistComplementaryDay;
    pub use positivist::PositivistMoment;
    pub use positivist::PositivistMonth;
    pub use roman::Roman;
    pub use roman::RomanMonth;
    pub use roman::RomanMonthlyEvent;
    pub use symmetry::Symmetry;
    pub use symmetry::Symmetry010;
    pub use symmetry::Symmetry010Moment;
    pub use symmetry::Symmetry010Solstice;
    pub use symmetry::Symmetry010SolsticeMoment;
    pub use symmetry::Symmetry454;
    pub use symmetry::Symmetry454Moment;
    pub use symmetry::Symmetry454Solstice;
    pub use symmetry::Symmetry454SolsticeMoment;
    pub use symmetry::SymmetryMonth;
    pub use tranquility::Tranquility;
    pub use tranquility::TranquilityComplementaryDay;
    pub use tranquility::TranquilityMoment;
    pub use tranquility::TranquilityMonth;
}
/// Formatting datestamps and timestamps
/// ## Crate Features
///
/// This module is only available if `display` is enabled.
#[cfg(feature = "display")]
#[allow(unused)]
pub mod display {
    mod moment;
    mod prelude;
    mod private;
    mod text {
        pub mod en;
        pub mod fr;
        pub mod prelude;
    }

    mod akan;
    mod armenian;
    mod clock;
    mod coptic;
    mod cotsworth;
    mod egyptian;
    mod ethiopic;
    mod french_rev;
    mod gregorian;
    mod holocene;
    mod iso;
    mod julian;
    mod positivist;
    mod roman;
    mod symmetry;
    mod tranquility;
    mod week;

    pub use moment::*;
    pub use prelude::*;

    pub use akan::*;
    pub use armenian::*;
    pub use clock::*;
    pub use coptic::*;
    pub use cotsworth::*;
    pub use egyptian::*;
    pub use ethiopic::*;
    pub use french_rev::*;
    pub use gregorian::*;
    pub use holocene::*;
    pub use iso::*;
    pub use julian::*;
    pub use positivist::*;
    pub use roman::*;
    pub use symmetry::*;
    pub use tranquility::*;
    pub use week::*;
}
