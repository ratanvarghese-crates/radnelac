// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::common::error::CalendarError;
use crate::common::math::TermNum;
use num_traits::AsPrimitive;

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
