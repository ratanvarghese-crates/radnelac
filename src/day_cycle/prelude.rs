// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::common::math::TermNum;
use crate::day_count::Fixed;
use num_traits::AsPrimitive;
use num_traits::FromPrimitive;
use num_traits::ToPrimitive;
use std::cmp::PartialEq;
use std::fmt::Debug;

pub trait BoundedCycle<const N: u8, const M: u8>:
    FromPrimitive + ToPrimitive + PartialEq + Debug
{
    fn cycle_length() -> u8 {
        N
    }

    fn min() -> u8 {
        M
    }

    fn max() -> u8 {
        match M {
            0 => N - 1,
            1 => N,
            _ => panic!("M must be 0 or 1 for BoundedCycle"),
        }
    }

    fn from_unbounded(x: i64) -> Self {
        let m = match M {
            0 => x.modulus(N.as_()),
            1 => x.adjusted_remainder(N.as_()),
            _ => panic!("M must be 0 or 1 for BoundedCycle"),
        };
        Self::from_i64(m).expect("Modulus guaranteed within range.")
    }

    fn to_unbounded(&self) -> i64 {
        self.to_i64().expect("Guaranteed result")
    }
}

pub trait OnOrBefore<const N: u8, const M: u8>: BoundedCycle<N, M> {
    fn raw_on_or_before(self, date: i64) -> Fixed;

    fn on_or_before(self, date: Fixed) -> Fixed {
        //LISTING 1.62,1.69 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        //Modified to generalize across different day cycles
        self.raw_on_or_before(date.get_day_i())
    }

    fn on_or_after(self, date: Fixed) -> Fixed {
        //LISTING 1.65,1.69 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        //Modified to generalize across different day cycles
        self.raw_on_or_before(date.get_day_i() + ((Self::cycle_length() as i64) - 1))
    }

    fn nearest(self, date: Fixed) -> Fixed {
        //LISTING 1.66,1.69 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        //Modified to generalize across different day cycles
        self.raw_on_or_before(date.get_day_i() + ((Self::cycle_length() as i64) / 2))
    }

    fn before(self, date: Fixed) -> Fixed {
        //LISTING 1.67,1.69 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        //Modified to generalize across different day cycles
        self.raw_on_or_before(date.get_day_i() - 1)
    }

    fn after(self, date: Fixed) -> Fixed {
        //LISTING 1.68,1.69 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        //Modified to generalize across different day cycles
        self.raw_on_or_before(date.get_day_i() + (Self::cycle_length() as i64))
    }
}
