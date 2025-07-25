//! A library for calendrical calculations in a variety of different timekeeping systems.

#[macro_use]
extern crate num_derive;

pub mod clock {
    mod clock_time;
    mod time_of_day;

    pub use clock_time::ClockTime;
    pub use time_of_day::TimeOfDay;
}
mod common {
    pub mod bound;
    pub mod date;
    pub mod error;
    pub mod math;
}
pub use common::bound;
pub use common::date;
pub use common::error::CalendarError;
pub mod day_count {
    mod fixed;
    mod jd;
    mod mjd;
    mod rd;
    mod unix;

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
pub mod day_cycle {
    mod akan;
    mod week;

    pub use akan::Akan;
    pub use akan::AkanPrefix;
    pub use akan::AkanStem;
    pub use week::Weekday;
}
pub mod calendar {
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

    pub use armenian::Armenian;
    pub use armenian::ArmenianDaysOfMonth;
    pub use armenian::ArmenianMonth;
    pub use coptic::Coptic;
    pub use coptic::CopticMonth;
    pub use cotsworth::Cotsworth;
    pub use cotsworth::CotsworthComplementaryDay;
    pub use cotsworth::CotsworthMonth;
    pub use egyptian::Egyptian;
    pub use egyptian::EgyptianDaysUponTheYear;
    pub use egyptian::EgyptianMonth;
    pub use ethiopic::Ethiopic;
    pub use ethiopic::EthiopicMonth;
    pub use french_rev_arith::FrenchRevArith;
    pub use french_rev_arith::FrenchRevMonth;
    pub use french_rev_arith::FrenchRevWeekday;
    pub use french_rev_arith::Sansculottide;
    pub use gregorian::Gregorian;
    pub use gregorian::GregorianMonth;
    pub use holocene::Holocene;
    pub use holocene::HoloceneMonth;
    pub use iso::ISO;
    pub use julian::Julian;
    pub use julian::JulianMonth;
    pub use olympiad::Olympiad;
    pub use positivist::Positivist;
    pub use positivist::PositivistComplementaryDay;
    pub use positivist::PositivistMonth;
    pub use roman::Roman;
    pub use roman::RomanMonth;
    pub use roman::RomanMonthlyEvent;
    pub use symmetry::Symmetry;
    pub use symmetry::Symmetry010;
    pub use symmetry::Symmetry010Solstice;
    pub use symmetry::Symmetry454;
    pub use symmetry::Symmetry454Solstice;
    pub use symmetry::SymmetryMonth;
    pub use tranquility::TranquilityComplementaryDay;
    pub use tranquility::TranquilityMoment;
    pub use tranquility::TranquilityMonth;
}
#[allow(unused)]
pub mod display {
    mod private;

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
    mod preset_fmt;
    mod roman;
    mod symmetry;
    mod tranquility;
    mod week;

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
    pub use preset_fmt::*;
    pub use roman::*;
    pub use symmetry::*;
    pub use tranquility::*;
    pub use week::*;
}
