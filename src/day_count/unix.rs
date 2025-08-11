// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::day_count::fixed::CalculatedBounds;
use crate::day_count::fixed::Epoch;
use crate::day_count::fixed::Fixed;
use crate::day_count::fixed::FromFixed;
use crate::day_count::fixed::ToFixed;
use crate::day_count::prelude::BoundedDayCount;

//LISTING 1.9 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
const UNIX_EPOCH: f64 = 719163.0;
const UNIX_DAY: f64 = 24.0 * 60.0 * 60.0;

/// Represents seconds since the Unix epoch
///
/// The Unix epoch is 00:00:00 UTC on January 1, 1970 CE in the proleptic Gregorian calendar.
///
/// This is internally an integer of more than 32 bits.
///
/// Note that the range of dates supported by this library is different from the range of dates
/// supported by standard Unix utilities. This library can support years beyond 2038 CE
/// (Gregorian) but if you plan to work with times in the distant past or future, please see
/// the documentation for `Fixed`.
///
/// Further reading:
/// + [Wikipedia](https://en.wikipedia.org/wiki/Unix_time)
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct UnixMoment(i64);

impl CalculatedBounds for UnixMoment {}

impl FromFixed for UnixMoment {
    fn from_fixed(t: Fixed) -> UnixMoment {
        //LISTING 1.11 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        //Modified from the original with `round()`
        UnixMoment((UNIX_DAY * (t.get() - UNIX_EPOCH)).round() as i64)
    }
}

impl ToFixed for UnixMoment {
    fn to_fixed(self) -> Fixed {
        //LISTING 1.10 (*Calendrical Calculations: The Ultimate Edition* by Reingold & Dershowitz.)
        Fixed::new(UNIX_EPOCH + ((self.0 as f64) / UNIX_DAY))
    }
}

impl Epoch for UnixMoment {
    fn epoch() -> Fixed {
        Fixed::new(UNIX_EPOCH)
    }
}

impl BoundedDayCount<i64> for UnixMoment {
    fn new(t: i64) -> UnixMoment {
        debug_assert!(UnixMoment::in_effective_bounds(t).is_ok());
        UnixMoment(t)
    }
    fn get(self) -> i64 {
        self.0
    }
}
