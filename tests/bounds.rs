use radnelac::calendar::*;
use radnelac::day_count::BoundedDayCount;
use radnelac::day_count::EffectiveBound;
use radnelac::day_count::Fixed;
use radnelac::day_count::FromFixed;

fn bounds_actually_work<T: EffectiveBound + FromFixed + std::cmp::PartialOrd>() {
    assert!(T::from_fixed(Fixed::effective_min()) < T::from_fixed(Fixed::cast_new(0)));
    assert!(T::from_fixed(Fixed::effective_max()) > T::from_fixed(Fixed::cast_new(0)));
    assert!(T::effective_min() < T::effective_max())
}

#[test]
fn armenian() {
    bounds_actually_work::<Armenian>();
    bounds_actually_work::<ArmenianMoment>();
}

#[test]
fn coptic() {
    bounds_actually_work::<Coptic>();
    bounds_actually_work::<CopticMoment>();
}

#[test]
fn cotsworth() {
    bounds_actually_work::<Cotsworth>();
    bounds_actually_work::<CotsworthMoment>();
}

#[test]
fn egyptian() {
    bounds_actually_work::<Egyptian>();
    bounds_actually_work::<EgyptianMoment>();
}

#[test]
fn ethiopic() {
    bounds_actually_work::<Ethiopic>();
    bounds_actually_work::<EthiopicMoment>();
}

#[test]
fn french_rev_arith() {
    bounds_actually_work::<FrenchRevArith<true>>();
    bounds_actually_work::<FrenchRevArith<false>>();
    bounds_actually_work::<FrenchRevArithMoment<true>>();
    bounds_actually_work::<FrenchRevArithMoment<false>>();
}

#[test]
fn gregorian() {
    bounds_actually_work::<Gregorian>();
    bounds_actually_work::<GregorianMoment>();
}

#[test]
fn holocene() {
    bounds_actually_work::<Holocene>();
    bounds_actually_work::<HoloceneMoment>();
}

#[test]
fn iso() {
    bounds_actually_work::<ISO>();
    bounds_actually_work::<ISOMoment>();
}

#[test]
fn julian() {
    bounds_actually_work::<Julian>();
    bounds_actually_work::<JulianMoment>();
}

#[test]
fn positivist() {
    bounds_actually_work::<Positivist>();
    bounds_actually_work::<PositivistMoment>();
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
    bounds_actually_work::<Symmetry010Moment>();
    bounds_actually_work::<Symmetry454Moment>();
    bounds_actually_work::<Symmetry010SolsticeMoment>();
    bounds_actually_work::<Symmetry454SolsticeMoment>();
}

#[test]
fn tranquility() {
    bounds_actually_work::<Tranquility>();
    bounds_actually_work::<TranquilityMoment>();
}
