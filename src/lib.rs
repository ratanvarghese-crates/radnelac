// Calendrical Calculations chapter 1
mod epoch {
    mod common;
    pub mod fixed;
    pub mod jd;
    pub mod mjd;
    pub mod rd;
    pub mod unix;
}
mod clock;
mod error;
mod math;
mod calendar {
    pub mod akan;
    pub mod armenian;
    pub mod common;
    pub mod egyptian;
    pub mod gregorian;
    pub mod julian;
}
