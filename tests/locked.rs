use proptest::proptest;
use radnelac::calendar::Armenian;
use radnelac::calendar::Coptic;
use radnelac::calendar::Cotsworth;
use radnelac::calendar::Egyptian;
use radnelac::calendar::Ethiopic;
use radnelac::calendar::Gregorian;
use radnelac::calendar::Holocene;
use radnelac::calendar::Positivist;
use radnelac::common::bound::BoundedDayCount;
use radnelac::common::date::CommonDate;
use radnelac::common::date::HasLeapYears;
use radnelac::common::date::ToFromCommonDate;
use radnelac::day_count::Epoch;
use radnelac::day_count::ToFixed;
use radnelac::day_count::FIXED_MAX;

const MAX_YEARS: i32 = (FIXED_MAX / 365.25) as i32;

fn locked_multi<T, U>(d0: CommonDate, d1: CommonDate)
where
    T: ToFromCommonDate + ToFixed + Epoch,
    U: ToFromCommonDate + ToFixed + Epoch,
{
    let a = T::try_from_common_date(d0).unwrap();
    let e = U::try_from_common_date(d1).unwrap();
    let fa = a.to_fixed();
    let fe = e.to_fixed();
    let diff_f = fa.get() - fe.get();
    let diff_e = T::epoch().get() - U::epoch().to_day().get();
    assert_eq!(diff_f, diff_e);
}

fn locked<T, U>(d: CommonDate)
where
    T: ToFromCommonDate + ToFixed + Epoch,
    U: ToFromCommonDate + ToFixed + Epoch,
{
    locked_multi::<T, U>(d, d);
}

fn locked_alt_multi<T, U>(d0: CommonDate, d1: CommonDate)
where
    T: ToFromCommonDate + ToFixed + Epoch,
    U: ToFromCommonDate + ToFixed + Epoch,
{
    let fh = T::try_from_common_date(d0).unwrap().to_fixed();
    let fg = U::try_from_common_date(d1).unwrap().to_fixed();
    assert_eq!(fh, fg);
}

fn locked_alt<T, U>(d0: CommonDate, ydiff: i32)
where
    T: ToFromCommonDate + ToFixed + Epoch,
    U: ToFromCommonDate + ToFixed + Epoch,
{
    let d1 = CommonDate {
        year: d0.year - ydiff,
        month: d0.month,
        day: d0.day,
    };
    locked_alt_multi::<T, U>(d0, d1);
}

proptest! {
    #[test]
    fn armenian_locked_to_egyptian(year in -MAX_YEARS..MAX_YEARS, month in 1..12, day in 1..30) {
        let d = CommonDate{ year: year, month: month as u8, day: day as u8 };
        locked::<Armenian, Egyptian>(d);
    }

    #[test]
    fn holocene_locked_to_gregorian(year in -MAX_YEARS..MAX_YEARS, month in 1..12, day in 1..28) {
        let d = CommonDate{ year: year, month: month as u8, day: day as u8 };
        locked::<Holocene, Gregorian>(d);
        locked_alt::<Holocene, Gregorian>(d, 10000);
    }

    #[test]
    fn ethiopic_locked_to_coptic(year in -MAX_YEARS..MAX_YEARS, month in 1..12, day in 1..30) {
        let d = CommonDate{ year: year, month: month as u8, day: day as u8 };
        locked::<Ethiopic, Coptic>(d);
    }

    #[test]
    fn cotsworth_start_with_gregorian(year in -MAX_YEARS..MAX_YEARS) {
        let d = CommonDate{ year: year, month: 1, day: 1 };
        locked::<Cotsworth, Gregorian>(d);
    }

    #[test]
    fn cotsworth_end_with_gregorian(year in -MAX_YEARS..MAX_YEARS) {
        let d0 = CommonDate{ year: year, month: 13, day: 29 };
        let d1 = CommonDate{ year: year, month: 12, day: 31 };
        locked_multi::<Cotsworth, Gregorian>(d0, d1);
    }

    #[test]
    fn positivist_start_with_gregorian(year in -MAX_YEARS..MAX_YEARS) {
        let d0 = CommonDate{ year: year, month: 1, day: 1 };
        locked_alt::<Positivist, Gregorian>(d0, -1788);
    }

    #[test]
    fn positivist_end_with_gregorian(year in -MAX_YEARS..MAX_YEARS) {
        let d0 = CommonDate{ year: year, month: 14, day: 1 };
        let gy = year + 1788;
        let gm = 12;
        let gd = if Gregorian::is_leap(gy) { 30 } else { 31 };
        let d1 = CommonDate{ year: gy, month: gm, day: gd };
        locked_alt_multi::<Positivist, Gregorian>(d0, d1);
        if Gregorian::is_leap(gy) {
            let d2 = CommonDate{ year: year, month: 14, day: 2 };
            let d3 = CommonDate{ year: gy, month: gm, day: 31 };
            locked_alt_multi::<Positivist, Gregorian>(d2, d3);
        }
    }
}
