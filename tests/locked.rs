use num_traits::FromPrimitive;
use num_traits::ToPrimitive;
use proptest::prop_assume;
use proptest::proptest;
use radnelac::calendar::Armenian;
use radnelac::calendar::ArmenianMonth;
use radnelac::calendar::CommonDate;
use radnelac::calendar::Coptic;
use radnelac::calendar::CopticMonth;
use radnelac::calendar::Cotsworth;
use radnelac::calendar::CotsworthMonth;
use radnelac::calendar::Egyptian;
use radnelac::calendar::EgyptianMonth;
use radnelac::calendar::Ethiopic;
use radnelac::calendar::EthiopicMonth;
use radnelac::calendar::Gregorian;
use radnelac::calendar::GregorianMonth;
use radnelac::calendar::HasIntercalaryDays;
use radnelac::calendar::HasLeapYears;
use radnelac::calendar::Holocene;
use radnelac::calendar::HoloceneMonth;
use radnelac::calendar::Julian;
use radnelac::calendar::JulianMonth;
use radnelac::calendar::Positivist;
use radnelac::calendar::PositivistMonth;
use radnelac::calendar::ToFromCommonDate;
use radnelac::calendar::TranquilityMoment;
use radnelac::calendar::TranquilityMonth;
use radnelac::day_count::BoundedDayCount;
use radnelac::day_count::Epoch;
use radnelac::day_count::Fixed;
use radnelac::day_count::FromFixed;
use radnelac::day_count::ToFixed;
use radnelac::day_count::FIXED_MAX;
use radnelac::day_cycle::Akan;
use radnelac::day_cycle::AkanStem;
use radnelac::day_cycle::Weekday;

const MAX_YEARS: i32 = (FIXED_MAX / 365.25) as i32;

fn locked_multi<R, S, T, U>(d0: CommonDate, d1: CommonDate)
where
    R: FromPrimitive + ToPrimitive,
    S: FromPrimitive + ToPrimitive,
    T: ToFromCommonDate<R> + ToFixed + Epoch,
    U: ToFromCommonDate<S> + ToFixed + Epoch,
{
    let a = T::try_from_common_date(d0).unwrap();
    let e = U::try_from_common_date(d1).unwrap();
    let fa = a.to_fixed();
    let fe = e.to_fixed();
    let diff_f = fa.get() - fe.get();
    let diff_e = T::epoch().get() - U::epoch().to_day().get();
    assert_eq!(diff_f, diff_e);
}

fn locked<R, S, T, U>(d: CommonDate)
where
    R: FromPrimitive + ToPrimitive,
    S: FromPrimitive + ToPrimitive,
    T: ToFromCommonDate<R> + ToFixed + Epoch,
    U: ToFromCommonDate<S> + ToFixed + Epoch,
{
    locked_multi::<R, S, T, U>(d, d);
}

fn locked_alt_multi<R, S, T, U>(d0: CommonDate, d1: CommonDate)
where
    R: FromPrimitive + ToPrimitive,
    S: FromPrimitive + ToPrimitive,
    T: ToFromCommonDate<R> + ToFixed + Epoch,
    U: ToFromCommonDate<S> + ToFixed + Epoch,
{
    let fh = T::try_from_common_date(d0).unwrap().to_fixed();
    let fg = U::try_from_common_date(d1).unwrap().to_fixed();
    assert_eq!(fh, fg, "d0 = {d0:?}, d1 = {d1:?}");
}

fn locked_alt<R, S, T, U>(d0: CommonDate, ydiff: i32)
where
    R: FromPrimitive + ToPrimitive,
    S: FromPrimitive + ToPrimitive,
    T: ToFromCommonDate<R> + ToFixed + Epoch,
    U: ToFromCommonDate<S> + ToFixed + Epoch,
{
    let d1 = CommonDate {
        year: d0.year - ydiff,
        month: d0.month,
        day: d0.day,
    };
    locked_alt_multi::<R, S, T, U>(d0, d1);
}

proptest! {
    #[test]
    fn armenian_locked_to_egyptian(year in -MAX_YEARS..MAX_YEARS, month in 1..12, day in 1..30) {
        let d = CommonDate{ year: year, month: month as u8, day: day as u8 };
        locked::<ArmenianMonth, EgyptianMonth, Armenian, Egyptian>(d);
        let a = Armenian::try_from_common_date(d).unwrap();
        let e = Egyptian::try_from_common_date(d).unwrap();
        assert_eq!(a.complementary().is_some(), e.complementary().is_some());
    }

    #[test]
    fn armenian_locked_to_egyptian_epagomenae(year in -MAX_YEARS..MAX_YEARS, day in 1..5) {
        let d = CommonDate{ year: year, month: 13, day: day as u8 };
        locked::<ArmenianMonth, EgyptianMonth, Armenian, Egyptian>(d);
        let a = Armenian::try_from_common_date(d).unwrap();
        let e = Egyptian::try_from_common_date(d).unwrap();
        assert_eq!(a.complementary().is_some(), e.complementary().is_some());
    }

    #[test]
    fn holocene_locked_to_gregorian(year in -MAX_YEARS..MAX_YEARS, month in 1..12, day in 1..28) {
        let d = CommonDate{ year: year, month: month as u8, day: day as u8 };
        locked::<HoloceneMonth, GregorianMonth, Holocene, Gregorian>(d);
        locked_alt::<HoloceneMonth, GregorianMonth, Holocene, Gregorian>(d, 10000);
    }

    #[test]
    fn coptic_to_julian_month_boundaries(year in -MAX_YEARS..MAX_YEARS) {
        // https://en.wikipedia.org/wiki/Coptic_calendar
        prop_assume!(year != -285);
        let cy = year;
        let jy = year + if year > -285 { 283 } else { 282 };
        let correction = if (year % 4) == 0 { 1 } else { 0 };
        let d_list = [
            (CommonDate::new(cy, 1, 1), CommonDate::new(jy, 8, 29 + correction)),
            (CommonDate::new(cy, 1, 30), CommonDate::new(jy, 9, 27 + correction)),
            (CommonDate::new(cy, 2, 1), CommonDate::new(jy, 9, 28 + correction)),
            (CommonDate::new(cy, 2, 30), CommonDate::new(jy, 10, 27 + correction)),
            (CommonDate::new(cy, 3, 1), CommonDate::new(jy, 10, 28 + correction)),
            (CommonDate::new(cy, 3, 30), CommonDate::new(jy, 11, 26 + correction)),
            (CommonDate::new(cy, 4, 1), CommonDate::new(jy, 11, 27 + correction)),
            (CommonDate::new(cy, 4, 30), CommonDate::new(jy, 12, 26 + correction)),
            (CommonDate::new(cy, 5, 1), CommonDate::new(jy, 12, 27 + correction)),
            (CommonDate::new(cy, 5, 30), CommonDate::new(jy + 1, 1, 25 + correction)),
            (CommonDate::new(cy, 6, 1), CommonDate::new(jy + 1, 1, 26 + correction)),
            (CommonDate::new(cy, 6, 30), CommonDate::new(jy + 1, 2, 24 + correction)),
            (CommonDate::new(cy, 7, 1), CommonDate::new(jy + 1, 2, 25 + correction)),
            (CommonDate::new(cy, 7, 30), CommonDate::new(jy + 1, 3, 26)),
            (CommonDate::new(cy, 8, 1), CommonDate::new(jy + 1, 3, 27)),
            (CommonDate::new(cy, 8, 30), CommonDate::new(jy + 1, 4, 25)),
            (CommonDate::new(cy, 9, 1), CommonDate::new(jy + 1, 4, 26)),
            (CommonDate::new(cy, 9, 30), CommonDate::new(jy + 1, 5, 25)),
            (CommonDate::new(cy, 10, 1), CommonDate::new(jy + 1, 5, 26)),
            (CommonDate::new(cy, 10, 30), CommonDate::new(jy + 1, 6, 24)),
            (CommonDate::new(cy, 11, 1), CommonDate::new(jy + 1, 6, 25)),
            (CommonDate::new(cy, 11, 30), CommonDate::new(jy + 1, 7, 24)),
            (CommonDate::new(cy, 12, 1), CommonDate::new(jy + 1, 7, 25)),
            (CommonDate::new(cy, 12, 30), CommonDate::new(jy + 1, 8, 23)),
            (CommonDate::new(cy, 13, 1), CommonDate::new(jy + 1, 8, 24)),
            (CommonDate::new(cy, 13, 5), CommonDate::new(jy + 1, 8, 28)),
        ];
        for pair in d_list {
            locked_alt_multi::<CopticMonth, JulianMonth, Coptic, Julian>(pair.0, pair.1);
        }
    }


    #[test]
    fn ethiopic_locked_to_coptic(year in -MAX_YEARS..MAX_YEARS, month in 1..12, day in 1..30) {
        let d = CommonDate{ year: year, month: month as u8, day: day as u8 };
        locked::<EthiopicMonth, CopticMonth, Ethiopic, Coptic>(d);
    }

    #[test]
    fn cotsworth_to_gregorian_month_boundaries(year in -MAX_YEARS..MAX_YEARS) {
        // https://en.wikipedia.org/wiki/International_Fixed_Calendar
        let d_list = [
            (CommonDate::new(year, 1, 1), CommonDate::new(year, 1, 1)),
            (CommonDate::new(year, 1, 28), CommonDate::new(year, 1, 28)),
            (CommonDate::new(year, 2, 1), CommonDate::new(year, 1, 29)),
            (CommonDate::new(year, 2, 28), CommonDate::new(year, 2, 25)),
            (CommonDate::new(year, 3, 1), CommonDate::new(year, 2, 26)),
            (CommonDate::new(year, 3, 28), CommonDate::new(year, 3, 25)),
            (CommonDate::new(year, 4, 1), CommonDate::new(year, 3, 26)),
            (CommonDate::new(year, 4, 28), CommonDate::new(year, 4, 22)),
            (CommonDate::new(year, 5, 1), CommonDate::new(year, 4, 23)),
            (CommonDate::new(year, 5, 28), CommonDate::new(year, 5, 20)),
            (CommonDate::new(year, 6, 1), CommonDate::new(year, 5, 21)),
            (CommonDate::new(year, 6, 28), CommonDate::new(year, 6, 17)),
            (CommonDate::new(year, 7, 1), CommonDate::new(year, 6, 18)),
            (CommonDate::new(year, 7, 28), CommonDate::new(year, 7, 15)),
            (CommonDate::new(year, 8, 1), CommonDate::new(year, 7, 16)),
            (CommonDate::new(year, 8, 28), CommonDate::new(year, 8, 12)),
            (CommonDate::new(year, 9, 1), CommonDate::new(year, 8, 13)),
            (CommonDate::new(year, 9, 28), CommonDate::new(year, 9, 9)),
            (CommonDate::new(year, 10, 1), CommonDate::new(year, 9, 10)),
            (CommonDate::new(year, 10, 28), CommonDate::new(year, 10, 7)),
            (CommonDate::new(year, 11, 1), CommonDate::new(year, 10, 8)),
            (CommonDate::new(year, 11, 28), CommonDate::new(year, 11, 4)),
            (CommonDate::new(year, 12, 1), CommonDate::new(year, 11, 5)),
            (CommonDate::new(year, 12, 28), CommonDate::new(year, 12, 2)),
            (CommonDate::new(year, 13, 1), CommonDate::new(year, 12, 3)),
            (CommonDate::new(year, 13, 28), CommonDate::new(year, 12, 30)),
        ];
        for pair in d_list {
            if Gregorian::is_leap(year) && pair.0 > CommonDate::new(year, 3, 1) && pair.0 < CommonDate::new(year, 7, 1) {
                let gd = CommonDate::new(pair.1.year, pair.1.month, pair.1.day - 1);
                locked_alt_multi::<CotsworthMonth, GregorianMonth, Cotsworth, Gregorian>(pair.0, gd);
            } else {
                locked_alt_multi::<CotsworthMonth, GregorianMonth, Cotsworth, Gregorian>(pair.0, pair.1);
            }
        }
    }

    #[test]
    fn cotsworth_to_gregorian_complementary_days(year in -MAX_YEARS..MAX_YEARS) {
        let d0 = CommonDate::new(year, 13, 29);
        let d1 = CommonDate::new(year, 12, 31);
        locked_alt_multi::<CotsworthMonth, GregorianMonth, Cotsworth, Gregorian>(d0, d1);
        if Gregorian::is_leap(year) {
            let d2 = CommonDate::new(year, 6, 29);
            let d3 = CommonDate::new(year, 6, 17);
            locked_alt_multi::<CotsworthMonth, GregorianMonth, Cotsworth, Gregorian>(d2, d3);
        }
    }

    #[test]
    fn positivist_to_gregorian_month_boundaries(year in -(MAX_YEARS - 1788)..(MAX_YEARS - 1788)) {
        // https://books.google.ca/books?id=S_BRAAAAMAAJ
        // See the "Summary Tableau" right before the Appendix
        let py = year;
        let gy = year + 1788;
        let d_list = [
            (CommonDate::new(py, 1, 1), CommonDate::new(gy, 1, 1)),
            (CommonDate::new(py, 1, 28), CommonDate::new(gy, 1, 28)),
            (CommonDate::new(py, 2, 1), CommonDate::new(gy, 1, 29)),
            (CommonDate::new(py, 2, 28), CommonDate::new(gy, 2, 25)),
            (CommonDate::new(py, 3, 1), CommonDate::new(gy, 2, 26)),
            (CommonDate::new(py, 3, 28), CommonDate::new(gy, 3, 25)),
            (CommonDate::new(py, 4, 1), CommonDate::new(gy, 3, 26)),
            (CommonDate::new(py, 4, 28), CommonDate::new(gy, 4, 22)),
            (CommonDate::new(py, 5, 1), CommonDate::new(gy, 4, 23)),
            (CommonDate::new(py, 5, 28), CommonDate::new(gy, 5, 20)),
            (CommonDate::new(py, 6, 1), CommonDate::new(gy, 5, 21)),
            (CommonDate::new(py, 6, 28), CommonDate::new(gy, 6, 17)),
            (CommonDate::new(py, 7, 1), CommonDate::new(gy, 6, 18)),
            (CommonDate::new(py, 7, 28), CommonDate::new(gy, 7, 15)),
            (CommonDate::new(py, 8, 1), CommonDate::new(gy, 7, 16)),
            (CommonDate::new(py, 8, 28), CommonDate::new(gy, 8, 12)),
            (CommonDate::new(py, 9, 1), CommonDate::new(gy, 8, 13)),
            (CommonDate::new(py, 9, 28), CommonDate::new(gy, 9, 9)),
            (CommonDate::new(py, 10, 1), CommonDate::new(gy, 9, 10)),
            (CommonDate::new(py, 10, 28), CommonDate::new(gy, 10, 7)),
            (CommonDate::new(py, 11, 1), CommonDate::new(gy, 10, 8)),
            (CommonDate::new(py, 11, 28), CommonDate::new(gy, 11, 4)),
            (CommonDate::new(py, 12, 1), CommonDate::new(gy, 11, 5)),
            (CommonDate::new(py, 12, 28), CommonDate::new(gy, 12, 2)),
            (CommonDate::new(py, 13, 1), CommonDate::new(gy, 12, 3)),
            (CommonDate::new(py, 13, 28), CommonDate::new(gy, 12, 30)),
        ];
        for pair in d_list {
            if Gregorian::is_leap(gy) && pair.0 > CommonDate::new(py, 3, 1) {
                let gd = CommonDate::new(pair.1.year, pair.1.month, pair.1.day - 1);
                locked_alt_multi::<PositivistMonth, GregorianMonth, Positivist, Gregorian>(pair.0, gd);
            } else {
                locked_alt_multi::<PositivistMonth, GregorianMonth, Positivist, Gregorian>(pair.0, pair.1);
            }
        }
    }

    #[test]
    fn positivist_to_gregorian_complementary_days(year in -(MAX_YEARS - 1788)..(MAX_YEARS - 1788)) {
        let d0 = CommonDate{ year: year, month: 14, day: 1 };
        let gy = year + 1788;
        let gm = 12;
        let gd = if Gregorian::is_leap(gy) { 30 } else { 31 };
        let d1 = CommonDate{ year: gy, month: gm, day: gd };
        locked_alt_multi::<PositivistMonth, GregorianMonth, Positivist, Gregorian>(d0, d1);
        if Gregorian::is_leap(gy) {
            let d2 = CommonDate{ year: year, month: 14, day: 2 };
            let d3 = CommonDate{ year: gy, month: gm, day: 31 };
            locked_alt_multi::<PositivistMonth, GregorianMonth, Positivist, Gregorian>(d2, d3);
        }
    }

    #[test]
    fn tranquility_to_gregorian_complementary_days(year in -(MAX_YEARS - 1970)..(MAX_YEARS - 1970)) {
        prop_assume!(year != 0 && year != -1);
        let ty = year;
        let gy = year + if ty > 0 { 1969 } else { 1970 };
        let d0 = CommonDate::new(ty, 0, 1);
        let d1 = CommonDate::new(gy, 7, 20);
        locked_alt_multi::<TranquilityMonth, GregorianMonth, TranquilityMoment, Gregorian>(d0, d1);
        if Gregorian::is_leap(gy) {
            let d2 = CommonDate::new(ty, 0, 2);
            let d3 = CommonDate::new(gy, 2, 29);
            locked_alt_multi::<TranquilityMonth, GregorianMonth, TranquilityMoment, Gregorian>(d2, d3);
        }
    }

    #[test]
    fn tranquility_to_gregorian_month_boundaries(year in -(MAX_YEARS - 1970)..(MAX_YEARS - 1970)) {
        // https://web.archive.org/web/20180818233025/https://en.wikipedia.org/wiki/Tranquility_calendar
        prop_assume!(year != 0);
        let ty = year;
        let gy = year - 1 + if ty > 0 { 1969 } else { 1970 };
        let d_list = [
            (CommonDate::new(ty, 1, 1), CommonDate::new(gy, 7, 21)),
            (CommonDate::new(ty, 1, 28), CommonDate::new(gy, 8, 17)),
            (CommonDate::new(ty, 2, 1), CommonDate::new(gy, 8, 18)),
            (CommonDate::new(ty, 2, 28), CommonDate::new(gy, 9, 14)),
            (CommonDate::new(ty, 3, 1), CommonDate::new(gy, 9, 15)),
            (CommonDate::new(ty, 3, 28), CommonDate::new(gy, 10, 12)),
            (CommonDate::new(ty, 4, 1), CommonDate::new(gy, 10, 13)),
            (CommonDate::new(ty, 4, 28), CommonDate::new(gy, 11, 9)),
            (CommonDate::new(ty, 5, 1), CommonDate::new(gy, 11, 10)),
            (CommonDate::new(ty, 5, 28), CommonDate::new(gy, 12, 7)),
            (CommonDate::new(ty, 6, 1), CommonDate::new(gy, 12, 8)),
            (CommonDate::new(ty, 6, 28), CommonDate::new(gy + 1, 1, 4)),
            (CommonDate::new(ty, 7, 1), CommonDate::new(gy + 1, 1, 5)),
            (CommonDate::new(ty, 7, 28), CommonDate::new(gy + 1, 2, 1)),
            (CommonDate::new(ty, 8, 1), CommonDate::new(gy + 1, 2, 2)),
            (CommonDate::new(ty, 8, 28), CommonDate::new(gy + 1, 3, 1)),
            (CommonDate::new(ty, 9, 1), CommonDate::new(gy + 1, 3, 2)),
            (CommonDate::new(ty, 9, 28), CommonDate::new(gy + 1, 3, 29)),
            (CommonDate::new(ty, 10, 1), CommonDate::new(gy + 1, 3, 30)),
            (CommonDate::new(ty, 10, 28), CommonDate::new(gy + 1, 4, 26)),
            (CommonDate::new(ty, 11, 1), CommonDate::new(gy + 1, 4, 27)),
            (CommonDate::new(ty, 11, 28), CommonDate::new(gy + 1, 5, 24)),
            (CommonDate::new(ty, 12, 1), CommonDate::new(gy + 1, 5, 25)),
            (CommonDate::new(ty, 12, 28), CommonDate::new(gy + 1, 6, 21)),
            (CommonDate::new(ty, 13, 1), CommonDate::new(gy + 1, 6, 22)),
            (CommonDate::new(ty, 13, 28), CommonDate::new(gy + 1, 7, 19)),
        ];
        for pair in d_list {
            locked_alt_multi::<TranquilityMonth, GregorianMonth, TranquilityMoment, Gregorian>(pair.0, pair.1);
        }
    }

    #[test]
    fn akan_stem_locked_to_weekday(x in ((-FIXED_MAX)+42.0)..(FIXED_MAX-42.0)) {
        //https://en.wikipedia.org/wiki/Akan_calendar
        let f = Fixed::new(x);
        let w = Weekday::from_fixed(f);
        let a = Akan::from_fixed(f).stem();
        let expected_a = match w {
            Weekday::Monday => AkanStem::Dwo,
            Weekday::Tuesday => AkanStem::Bene,
            Weekday::Wednesday => AkanStem::Wukuo,
            Weekday::Thursday => AkanStem::Yaw,
            Weekday::Friday => AkanStem::Fie,
            Weekday::Saturday => AkanStem::Memene,
            Weekday::Sunday => AkanStem::Kwasi,
        };
        assert_eq!(a, expected_a);
    }

}
