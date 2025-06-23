use radnelac::calendar::Armenian;
use radnelac::calendar::Coptic;
use radnelac::calendar::Cotsworth;
use radnelac::calendar::Egyptian;
use radnelac::calendar::Ethiopic;
use radnelac::calendar::FrenchRevArith;
use radnelac::calendar::Gregorian;
use radnelac::calendar::Holocene;
use radnelac::calendar::Julian;
use radnelac::calendar::Olympiad;
use radnelac::calendar::Positivist;
use radnelac::calendar::Roman;
use radnelac::calendar::Symmetry010;
use radnelac::calendar::Symmetry010Solstice;
use radnelac::calendar::Symmetry454;
use radnelac::calendar::Symmetry454Solstice;
use radnelac::calendar::TranquilityMoment;
use radnelac::calendar::ISO;
use radnelac::clock::TimeOfDay;
use radnelac::common::bound::BoundedDayCount;
use radnelac::common::bound::EffectiveBound;
use radnelac::day_count::Fixed;
use radnelac::day_count::FromFixed;
use radnelac::day_count::JulianDay;
use radnelac::day_count::ModifiedJulianDay;
use radnelac::day_count::RataDie;
use radnelac::day_count::ToFixed;
use radnelac::day_count::UnixMoment;
use radnelac::day_cycle::Akan;
use radnelac::day_cycle::Weekday;
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
    let t_unix = UnixMoment::new(t_system as i64);
    let t_fixed = t_unix.to_fixed();
    print_t(t_fixed);
}

fn print_t(t_fixed: Fixed) {
    let m_clk = TimeOfDay::from_fixed(t_fixed);
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
    let d_holocene = Holocene::from_fixed(t_fixed);
    let d_french0 = FrenchRevArith::<true>::from_fixed(t_fixed);
    let d_french1 = FrenchRevArith::<false>::from_fixed(t_fixed);
    let d_positivist = Positivist::from_fixed(t_fixed);
    let d_cotsworth = Cotsworth::from_fixed(t_fixed);
    let d_symmetry454 = Symmetry454::from_fixed(t_fixed);
    let d_symmetry010 = Symmetry010::from_fixed(t_fixed);
    let d_symmetry454s = Symmetry454Solstice::from_fixed(t_fixed);
    let d_symmetry010s = Symmetry010Solstice::from_fixed(t_fixed);
    let d_tranquility = TranquilityMoment::from_fixed(t_fixed);
    let y_roman = Roman::auc_year_from_julian(d_julian.year());
    let y_olympiad = Olympiad::olympiad_from_julian_year(d_julian.year());

    println!("{} ({:?})", m_clk, m_clk);
    println!("{:?}", t_unix);
    println!("{:?}", t_jd);
    println!("{:?}", t_mjd);
    println!("{:?}", t_rd);
    println!("{} ({:?})", w_week, w_week);
    println!("{} ({:?})", w_akan, w_akan);
    println!("{} ({:?})", d_egyptian, d_egyptian);
    println!("{} ({:?})", d_armenian, d_armenian);
    println!("{} ({:?})", d_gregorian, d_gregorian);
    println!("{} ({:?})", d_julian, d_julian);
    println!("{} ({:?})", d_coptic, d_coptic);
    println!("{} ({:?})", d_ethiopic, d_ethiopic);
    println!("{} ({:?})", d_roman, d_roman);
    println!("{:?}", d_iso);
    println!("{} ({:?})", d_holocene, d_holocene);
    println!(
        "{} ({:?} mode: {:?})",
        d_french0,
        d_french0,
        d_french0.is_adjusted()
    );
    println!(
        "{} ({:?} mode: {:?})",
        d_french1,
        d_french1,
        d_french1.is_adjusted()
    );
    println!("{} ({:?})", d_positivist, d_positivist);
    println!("{} ({:?})", d_cotsworth, d_cotsworth);
    println!(
        "{} ({:?} mode: {:?})",
        d_symmetry454,
        d_symmetry454,
        d_symmetry454.mode()
    );
    println!(
        "{} ({:?} mode: {:?})",
        d_symmetry010,
        d_symmetry010,
        d_symmetry010.mode()
    );
    println!(
        "{} ({:?} mode: {:?})",
        d_symmetry454s,
        d_symmetry454s,
        d_symmetry454s.mode()
    );
    println!(
        "{} ({:?} mode: {:?})",
        d_symmetry010s,
        d_symmetry010s,
        d_symmetry010s.mode()
    );
    println!("{} ({:?})", d_tranquility, d_tranquility);
    println!("{:?} AUC", y_roman);
    println!("{:?}", y_olympiad);
}
