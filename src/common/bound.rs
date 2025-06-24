use crate::common::error::CalendarError;
use crate::common::math::TermNum;
use num_traits::AsPrimitive;
use num_traits::FromPrimitive;

pub trait EffectiveBound: Copy + Clone + PartialEq + PartialOrd {
    fn effective_min() -> Self;
    fn effective_max() -> Self;
}

pub trait BoundedDayCount<T: TermNum>: EffectiveBound {
    fn new(t: T) -> Self;
    fn get(self) -> T;

    fn almost_in_effective_bounds(t: T, dt: T) -> Result<(), CalendarError> {
        if t.is_a_number() {
            let min = Self::effective_min().get() - dt;
            let max = Self::effective_max().get() + dt;
            if t >= min && t <= max {
                Ok(())
            } else {
                Err(CalendarError::OutOfBounds)
            }
        } else {
            Err(CalendarError::EncounteredNaN)
        }
    }

    fn in_effective_bounds(t: T) -> Result<(), CalendarError> {
        Self::almost_in_effective_bounds(t, T::zero())
    }

    fn cast_new<U: AsPrimitive<T>>(t: U) -> Self {
        Self::new(t.as_())
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
