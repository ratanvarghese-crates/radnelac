#[macro_use]
extern crate num_derive;

pub mod common {
    pub mod bound;
    pub mod date;
    pub mod error;
    pub mod math;
}
pub mod day_count {
    pub mod fixed;
    pub mod jd;
    pub mod mjd;
    pub mod rd;
    pub mod unix;
}
pub mod day_cycle {
    pub mod akan;
    pub mod week;
}
pub mod calendar {
    pub mod armenian;
    pub mod coptic;
    pub mod egyptian;
    pub mod ethiopic;
    pub mod gregorian;
    pub mod holocene;
    pub mod iso;
    pub mod julian;
    pub mod olympiad;
    pub mod roman;
}
pub mod clock;
