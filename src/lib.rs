#[macro_use]
extern crate num_derive;

pub mod common {
    pub mod bound;
    pub mod date;
    pub mod error;
    pub mod math;
}
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
    pub use coptic::Coptic;
    pub use cotsworth::Cotsworth;
    pub use egyptian::Egyptian;
    pub use ethiopic::Ethiopic;
    pub use french_rev_arith::FrenchRevArith;
    pub use gregorian::Gregorian;
    pub use holocene::Holocene;
    pub use iso::ISO;
    pub use julian::Julian;
    pub use olympiad::Olympiad;
    pub use positivist::Positivist;
    pub use roman::Roman;
    pub use symmetry::Symmetry010;
    pub use symmetry::Symmetry010Solstice;
    pub use symmetry::Symmetry454;
    pub use symmetry::Symmetry454Solstice;
    pub use tranquility::TranquilityMoment;
}
pub mod clock;
