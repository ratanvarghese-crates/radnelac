use radnelac::calendar::Armenian;
use radnelac::calendar::Coptic;
use radnelac::calendar::Cotsworth;
use radnelac::calendar::Egyptian;
use radnelac::calendar::Ethiopic;
use radnelac::calendar::FrenchRevArith;
use radnelac::calendar::Gregorian;
use radnelac::calendar::Holocene;
use radnelac::calendar::Julian;
use radnelac::calendar::Positivist;
use radnelac::calendar::Roman;
use radnelac::calendar::Symmetry010;
use radnelac::calendar::Symmetry010Solstice;
use radnelac::calendar::Symmetry454;
use radnelac::calendar::Symmetry454Solstice;
use radnelac::calendar::TranquilityMoment;
use radnelac::calendar::ISO;
use radnelac::common::bound::BoundedDayCount;
use radnelac::common::bound::EffectiveBound;
use radnelac::day_count::Fixed;
use radnelac::day_count::FromFixed;

fn bounds_actually_work<T: FromFixed + std::cmp::PartialOrd>() {
    assert!(T::from_fixed(Fixed::effective_min()) < T::from_fixed(Fixed::cast_new(0)));
    assert!(T::from_fixed(Fixed::effective_max()) > T::from_fixed(Fixed::cast_new(0)));
}

#[test]
fn armenian() {
    bounds_actually_work::<Armenian>();
}

#[test]
fn coptic() {
    bounds_actually_work::<Coptic>();
}

#[test]
fn cotsworth() {
    bounds_actually_work::<Cotsworth>();
}

#[test]
fn egyptian() {
    bounds_actually_work::<Egyptian>();
}

#[test]
fn ethiopic() {
    bounds_actually_work::<Ethiopic>();
}

#[test]
fn french_rev_arith() {
    bounds_actually_work::<FrenchRevArith<true>>();
    bounds_actually_work::<FrenchRevArith<false>>();
}

#[test]
fn gregorian() {
    bounds_actually_work::<Gregorian>();
}

#[test]
fn holocene() {
    bounds_actually_work::<Holocene>();
}

#[test]
fn iso() {
    bounds_actually_work::<ISO>();
}

#[test]
fn julian() {
    bounds_actually_work::<Julian>();
}

#[test]
fn positivist() {
    bounds_actually_work::<Positivist>();
}

#[test]
fn roman() {
    bounds_actually_work::<Roman>();
}

#[test]
fn symmetry() {
    bounds_actually_work::<Symmetry010>();
    bounds_actually_work::<Symmetry454>();
    bounds_actually_work::<Symmetry010Solstice>();
    bounds_actually_work::<Symmetry454Solstice>();
}

#[test]
fn tranquility() {
    bounds_actually_work::<TranquilityMoment>();
}
