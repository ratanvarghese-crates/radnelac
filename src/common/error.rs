use std::error::Error;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub enum CalendarError {
    InvalidYear,
    InvalidMonth,
    InvalidDay,
    InvalidHour,
    InvalidMinute,
    InvalidSecond,
    InvalidDayOfYear,
    DivisionByZero,
    OutOfBounds,
    MixedRadixWrongSize,
    MixedRadixZeroBase,
    EncounteredNaN,
    ImpossibleResult,
}

impl Display for CalendarError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalendarError::InvalidYear => write!(f, "Invalid Year"),
            CalendarError::InvalidMonth => write!(f, "Invalid Month"),
            CalendarError::InvalidDay => write!(f, "Invalid Day"),
            CalendarError::InvalidHour => write!(f, "Invalid Hour"),
            CalendarError::InvalidMinute => write!(f, "Invalid Minute"),
            CalendarError::InvalidSecond => write!(f, "Invalid Second"),
            CalendarError::InvalidDayOfYear => write!(f, "Invalid day of year"),
            CalendarError::DivisionByZero => write!(f, "Division By Zero"),
            CalendarError::OutOfBounds => write!(f, "Out Of Bounds"),
            CalendarError::MixedRadixWrongSize => write!(f, "Mixed radix slices have wrong size"),
            CalendarError::MixedRadixZeroBase => write!(f, "Mixed radix base contains zero"),
            CalendarError::EncounteredNaN => write!(f, "Encountered Not a Number (NaN)"),
            CalendarError::ImpossibleResult => write!(f, "Impossible result"),
        }
    }
}

impl Error for CalendarError {}
