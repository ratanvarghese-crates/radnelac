use std::ops::Add;
use std::ops::Sub;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct FixedMoment(pub f64);

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct FixedDate(pub f64);

impl From<FixedDate> for FixedMoment {
    fn from(date: FixedDate) -> FixedMoment {
        FixedMoment(date.0)
    }
}

impl From<FixedMoment> for FixedDate {
    fn from(t: FixedMoment) -> FixedDate {
        FixedDate(t.0.floor())
    }
}

impl Sub for FixedDate {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        FixedDate(self.0 - other.0)
    }
}

impl Add for FixedDate {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        FixedDate(self.0 + other.0)
    }
}

impl Sub for FixedMoment {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        FixedMoment(self.0 - other.0)
    }
}

impl Add for FixedMoment {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        FixedMoment(self.0 + other.0)
    }
}

pub trait Epoch {
    type Output: Add + Sub;
    fn epoch() -> Self::Output;
}
