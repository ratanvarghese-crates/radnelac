macro_rules! simple_bounded {
    ($t: ident, $u: ident, $min: ident, $max: ident) => {
        impl Bounded for $u {
            fn min_value() -> $u {
                $u { 0: $min as $t }
            }

            fn max_value() -> $u {
                $u { 0: $max as $t }
            }
        }

        impl TryFrom<$t> for $u {
            type Error = CalendarError;
            fn try_from(date: $t) -> Result<$u, Self::Error> {
                if (date.is_weird()) {
                    Err(CalendarError::NaNInfinite)
                } else if (date < $u::min_value().0) {
                    Err(CalendarError::OutOfBounds)
                } else if (date > $u::max_value().0) {
                    Err(CalendarError::OutOfBounds)
                } else {
                    Ok($u { 0: date })
                }
            }
        }

        impl From<$u> for $t {
            fn from(date: $u) -> $t {
                date.0
            }
        }
    };
}

macro_rules! bounded_small_int_guaranteed {
    ($t:ident, $u: ident, $v: ident) => {
        impl From<$t> for $v {
            fn from(date: $t) -> $v {
                $v::try_from(date as $u).expect("Known to be within bounds.")
            }
        }

        impl TryFrom<$v> for $t {
            type Error = CalendarError;
            fn try_from(date: $v) -> Result<$t, Self::Error> {
                if (date.0 < ($t::min_value() as $u) || date.0 > ($t::max_value() as $u)) {
                    Err(CalendarError::OutOfBounds)
                } else {
                    Ok(date.0 as $t)
                }
            }
        }
    };
}

pub(crate) use bounded_small_int_guaranteed;
pub(crate) use simple_bounded;
