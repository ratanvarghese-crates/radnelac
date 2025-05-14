use std::error::Error;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub enum CalendarError {
    InvalidHour,
    InvalidMinute,
    InvalidSecond,
    DivisionByZero,
    OutOfBounds,
    MixedRadixWrongSize,
    MixedRadixZeroBase,
    NaNInfinite,
    ImpossibleResult,
}

impl Display for CalendarError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalendarError::InvalidHour => write!(f, "Invalid Hour"),
            CalendarError::InvalidMinute => write!(f, "Invalid Minute"),
            CalendarError::InvalidSecond => write!(f, "Invalid Second"),
            CalendarError::DivisionByZero => write!(f, "Division By Zero"),
            CalendarError::OutOfBounds => write!(f, "Out Of Bounds"),
            CalendarError::MixedRadixWrongSize => write!(f, "Mixed radix slices have wrong size"),
            CalendarError::MixedRadixZeroBase => write!(f, "Mixed radix base contains zero"),
            CalendarError::NaNInfinite => write!(f, "NaN or infinite"),
            CalendarError::ImpossibleResult => write!(f, "Impossible result"),
        }
    }
}

impl Error for CalendarError {}
