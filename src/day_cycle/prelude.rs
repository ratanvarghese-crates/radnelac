use crate::common::math::TermNum;
use num_traits::AsPrimitive;
use num_traits::FromPrimitive;

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
