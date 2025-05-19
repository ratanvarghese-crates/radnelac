use radnelac::calendar::armenian::Armenian;
use radnelac::calendar::coptic::Coptic;
use radnelac::calendar::egyptian::Egyptian;
use radnelac::calendar::ethiopic::Ethiopic;
use radnelac::calendar::gregorian::Gregorian;
use radnelac::calendar::iso::ISO;
use radnelac::calendar::julian::Julian;
use radnelac::calendar::olympiad::Olympiad;
use radnelac::calendar::roman::Roman;
use radnelac::common::bound::BoundedDayCount;
use radnelac::common::bound::EffectiveBound;
use radnelac::day_count::fixed::Fixed;
use radnelac::day_count::fixed::FromFixed;
use radnelac::day_count::fixed::ToFixed;
use radnelac::day_count::jd::JulianDay;
use radnelac::day_count::mjd::ModifiedJulianDay;
use radnelac::day_count::rd::RataDie;
use radnelac::day_count::unix::UnixMoment;
use radnelac::day_cycle::akan::Akan;
use radnelac::day_cycle::week::Weekday;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    println!("Today is:");
    print_today();
    println!("\n\n");
    println!("Effective Minimum is:");
    print_t(Fixed::effective_min());
    println!("\n\n");
    println!("Effective Maximum is:");
    print_t(Fixed::effective_max());
}

fn print_today() {
    let t_system = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX_EPOCH"),
    };
    let t_unix = UnixMoment::checked_new(t_system as i64).unwrap();
    let t_fixed = t_unix.to_fixed();
    print_t(t_fixed);
}

fn print_t(t_fixed: Fixed) {
    let t_unix = UnixMoment::from_fixed(t_fixed);
    let t_jd = JulianDay::from_fixed(t_fixed);
    let t_mjd = ModifiedJulianDay::from_fixed(t_fixed);
    let t_rd = RataDie::from_fixed(t_fixed);
    let w_week = Weekday::from_fixed(t_fixed);
    let w_akan = Akan::from_fixed(t_fixed);
    let d_egyptian = Egyptian::from_fixed(t_fixed);
    let d_armenian = Armenian::from_fixed(t_fixed);
    let d_gregorian = Gregorian::from_fixed(t_fixed);
    let d_julian = Julian::from_fixed(t_fixed);
    let d_roman = Roman::from_fixed(t_fixed);
    let d_coptic = Coptic::from_fixed(t_fixed);
    let d_ethiopic = Ethiopic::from_fixed(t_fixed);
    let d_iso = ISO::from_fixed(t_fixed);
    let y_roman = Roman::auc_year_from_julian(d_julian.year());
    let y_olympiad = Olympiad::olympiad_from_julian_year(d_julian.year());

    println!("{:?}", t_unix);
    println!("{:?}", t_jd);
    println!("{:?}", t_mjd);
    println!("{:?}", t_rd);
    println!("{:?}", w_week);
    println!("{:?}", w_akan);
    println!("{:?}", d_egyptian);
    println!("{:?}", d_armenian);
    println!("{:?}", d_gregorian);
    println!("{:?}", d_julian);
    println!("{:?}", d_coptic);
    println!("{:?}", d_ethiopic);
    println!("{:?}", d_roman);
    println!("{:?}", d_iso);
    println!("{:?} AUC", y_roman);
    println!("{:?}", y_olympiad);
}
