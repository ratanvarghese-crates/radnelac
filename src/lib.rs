// Calendrical Calculations chapter 1
mod epoch {
    pub mod fixed;
    pub mod jd;
    pub mod rd;
    pub mod unix;
}
mod clock;
mod error;
mod math;
mod calendar {
    mod armenian;
    mod common;
    mod egyptian;
}
