use super::*;
use DegreeAspectKind::*;

//
// TODO: Need to make more advanced function that takes bodies into consideration!
// ie CONJUNCTION_ORB_DEGREES_WITHOUT_SUN_OR_MOON
//
pub fn filter_orb(degree_aspect_kind: &DegreeAspectKind, diff: f64) -> bool {
    let deg = match degree_aspect_kind {
        Conjunction => CONJUNCTION_ORB_DEGREES_WITH_SUN_OR_MOON,
        Opposition => OPPOSITION_ORB_DEGREES,
        Sextile => SEXTILE_ORB_DEGREES,
        Square => SQUARE_ORB_DEGREES,
        Trine => TRINE_ORB_DEGREES,
        Semisextile => SEMISEXTILE_ORB_DEGREES,
        Quincunx => QUINCUNX_ORB_DEGREES,
        Septile => SEPTILE_ORB_DEGREES,
        SemiSquare => OCTILE_ORB_DEGREES,
        Sesquiquadrate => SESQUIQUADRATE_ORB_DEGREES,
        Novile => NOVILE_ORB_DEGREES,
        Undecile => UNDECILE_ORB_DEGREES,
        _ => DEFAULT_ORB_DEGREES,
    };

    diff <= deg
}
