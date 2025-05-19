use crate::common::error::CalendarError;
use crate::common::math::SmallNum;
use crate::common::math::TermNum;
use crate::common::math::TermNum64;
use num_traits::clamp;
use num_traits::AsPrimitive;
use num_traits::FromPrimitive;

pub trait EffectiveBound: Copy + Clone + PartialEq + PartialOrd {
    fn effective_min() -> Self;
    fn effective_max() -> Self;
}

pub trait BoundedDayCount<T: TermNum64>: EffectiveBound {
    fn unchecked_new(t: T) -> Self;
    fn get(self) -> T;

    fn in_effective_bounds(t: T) -> Result<(), CalendarError> {
        if t.is_a_number() {
            let min = Self::effective_min().get();
            let max = Self::effective_max().get();
            if t >= min && t <= max {
                Ok(())
            } else {
                Err(CalendarError::OutOfBounds)
            }
        } else {
            Err(CalendarError::EncounteredNaN)
        }
    }

    // Default implementations for *new
    fn new<V: SmallNum>(t: V) -> Self {
        Self::unchecked_new(t.to_term_num_64())
    }

    fn cast_new<U: TermNum64 + AsPrimitive<T>>(t: U) -> Result<Self, CalendarError> {
        Self::checked_new(t.as_())
    }

    fn checked_new(t: T) -> Result<Self, CalendarError> {
        match Self::in_effective_bounds(t) {
            Ok(_) => Ok(Self::unchecked_new(t)),
            Err(error) => Err(error),
        }
    }

    fn clamped_new<U: TermNum + AsPrimitive<T>>(t: U) -> Self {
        let min = Self::effective_min().get();
        let max = Self::effective_max().get();
        let u: T = if t.is_a_number() { t.as_() } else { T::zero() };
        Self::unchecked_new(clamp(u, min, max))
    }

    //Default implementations for arithmetic
    fn unchecked_add<U: TermNum + AsPrimitive<T>>(self, t: U) -> T {
        self.get() + t.as_()
    }

    fn unchecked_sub<U: TermNum + AsPrimitive<T>>(self, t: U) -> T {
        self.get() + t.as_()
    }

    fn checked_add<U: TermNum + AsPrimitive<T>>(self, t: U) -> Result<Self, CalendarError> {
        Self::checked_new(self.get() + t.as_())
    }

    fn checked_sub<U: TermNum + AsPrimitive<T>>(self, t: U) -> Result<Self, CalendarError> {
        Self::checked_new(self.get() + t.as_())
    }

    fn cast_add<U: TermNum + AsPrimitive<T>>(self, t: U) -> Result<Self, CalendarError> {
        Self::cast_new(self.get() + t.as_())
    }

    fn cast_sub<U: TermNum + AsPrimitive<T>>(self, t: U) -> Result<Self, CalendarError> {
        Self::cast_new(self.get() + t.as_())
    }

    fn clamped_add<U: TermNum + AsPrimitive<T>>(self, t: U) -> Self {
        Self::clamped_new(self.get() + t.as_())
    }

    fn clamped_sub<U: TermNum + AsPrimitive<T>>(self, t: U) -> Self {
        Self::clamped_new(self.get() - t.as_())
    }
}

pub trait BoundedCycle<const N: u8, const M: u8>: FromPrimitive {
    fn cycle_length() -> u8 {
        N
    }

    fn from_unbounded(x: i64) -> Self {
        let m = if M == 0 {
            x.modulus(N.as_())
        } else if M == 1 {
            x.adjusted_remainder(N.as_())
        } else {
            panic!("M must be 0 or 1 for BoundedCycle")
        };
        Self::from_i64(m).expect("Modulus guaranteed within range.")
    }
}
