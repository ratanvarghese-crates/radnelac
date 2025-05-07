// Calendrical Calculations chapter 1
use crate::math::from_mixed_radix;
use crate::math::modulus;
use crate::math::to_mixed_radix;

const RD_EPOCH: f64 = 0.0;

pub const fn rd(t: f64) -> f64 {
    t - RD_EPOCH
}

const JD_EPOCH: f64 = -1721424.5;

pub const fn moment_from_jd(jd: f64) -> f64 {
    jd + JD_EPOCH
}

pub const fn moment_to_jd(t: f64) -> f64 {
    t - JD_EPOCH
}

const MJD_EPOCH: f64 = 678576.0;

pub const fn fixed_from_mjd(mjd: f64) -> f64 {
    mjd + MJD_EPOCH
}

pub const fn fixed_to_mjd(date: f64) -> f64 {
    date - MJD_EPOCH
}

const UNIX_EPOCH: f64 = 719163.0;

pub const fn moment_from_unix(s: f64) -> f64 {
    UNIX_EPOCH + (s / (24.0 * 60.0 * 60.0))
}

pub const fn moment_to_unix(t: f64) -> f64 {
    24.0 * 60.0 * 60. * (t - UNIX_EPOCH)
}

pub fn fixed_from_moment(t: f64) -> f64 {
    t.floor()
}

pub fn fixed_from_jd(jd: f64) -> f64 {
    fixed_from_moment(moment_from_jd(jd))
}

pub const fn fixed_to_jd(date: f64) -> f64 {
    moment_to_jd(date)
}

pub fn time_from_moment(t: f64) -> f64 {
    modulus(t, 1.0)
}

pub fn time_from_clock(hours: f64, minutes: f64, seconds: f64) -> f64 {
    let a = [hours, minutes, seconds];
    let b = [24.0, 60.0, 60.0];
    from_mixed_radix(&a, &b, 0.0) / 24.0
}

pub fn clock_from_moment(t: f64) -> Vec<f64> {
    let b = [24.0, 60.0, 60.0];
    to_mixed_radix(t, &b, 0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rd_is_epoch() {
        let result = rd(RD_EPOCH);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn jd_roundtrip() {
        let j0: f64 = 12345678.9;
        let j1 = moment_to_jd(moment_from_jd(j0));
        assert_eq!(j0, j1);
        let j2 = fixed_to_jd(fixed_from_jd(j0));
        assert_eq!(j0.floor(), j2.floor());
    }

    #[test]
    fn mjd_roundtrip() {
        let j0: f64 = 12345678.9;
        let j1 = fixed_to_mjd(fixed_from_mjd(j0));
        assert_eq!(j0, j1);
    }

    #[test]
    fn mjd_from_jd() {
        let x = 12345678.0;
        let j0 = moment_to_jd(x);
        let mjd0 = fixed_to_mjd(x);
        assert_eq!(mjd0, j0 - 2400000.5);
    }

    #[test]
    fn unix_roundtrip() {
        let unix0: f64 = 1746587115.0;
        let unix1 = moment_to_unix(moment_from_unix(unix0));
        assert_eq!(unix0.floor(), unix1.floor());
    }

    #[test]
    fn time() {
        assert_eq!(time_from_moment(JD_EPOCH), 0.5);
    }
}
