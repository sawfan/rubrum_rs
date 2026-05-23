use super::*;

//
// https://en.wikipedia.org/wiki/Thema_Mundi
//
//

//pub const THEMA_MUNDI_CLASSICAL_PLANET_PLACEMENTS: ClassicalPlanetPlacement
//= ClassicalPlanetPlacement::new_const(
//    THEMA_MUNDI_SUN,
//    THEMA_MUNDI_MOON,
//    THEMA_MUNDI_MERCURY,
//    THEMA_MUNDI_VENUS,
//    THEMA_MUNDI_MARS,
//    THEMA_MUNDI_JUPITER,
//    THEMA_MUNDI_SATURN,
//);

pub const THEMA_MUNDI_BODY_SIGN_DEGREES: [BodySignDegree; 7] = [
    THEMA_MUNDI_MOON,
    THEMA_MUNDI_SUN,
    THEMA_MUNDI_MERCURY,
    THEMA_MUNDI_VENUS,
    THEMA_MUNDI_MARS,
    THEMA_MUNDI_JUPITER,
    THEMA_MUNDI_SATURN,
];

pub const THEMA_MUNDI_MOON_SIGN: Sign = Cancer;
pub const THEMA_MUNDI_MOON_DEGREES: f64 = 15.0;
pub const THEMA_MUNDI_MOON_SIGN_DEGREE: SignDegree =
    SignDegree::from_sign_and_degree30_f64(THEMA_MUNDI_MOON_SIGN, THEMA_MUNDI_MOON_DEGREES);
pub const THEMA_MUNDI_MOON: BodySignDegree =
    BodySignDegree::new_const(Moon, THEMA_MUNDI_MOON_SIGN_DEGREE);

pub const THEMA_MUNDI_SUN_SIGN: Sign = Leo;
pub const THEMA_MUNDI_SUN_DEGREES: f64 = 15.0;
pub const THEMA_MUNDI_SUN_SIGN_DEGREE: SignDegree =
    SignDegree::from_sign_and_degree30_f64(THEMA_MUNDI_SUN_SIGN, THEMA_MUNDI_SUN_DEGREES);
pub const THEMA_MUNDI_SUN: BodySignDegree =
    BodySignDegree::new_const(Sun, THEMA_MUNDI_SUN_SIGN_DEGREE);

pub const THEMA_MUNDI_MERCURY_SIGN: Sign = Virgo;
pub const THEMA_MUNDI_MERCURY_DEGREES: f64 = 15.0;
pub const THEMA_MUNDI_MERCURY_SIGN_DEGREE: SignDegree =
    SignDegree::from_sign_and_degree30_f64(THEMA_MUNDI_MERCURY_SIGN, THEMA_MUNDI_MERCURY_DEGREES);
pub const THEMA_MUNDI_MERCURY: BodySignDegree =
    BodySignDegree::new_const(Mercury, THEMA_MUNDI_MERCURY_SIGN_DEGREE);

pub const THEMA_MUNDI_VENUS_SIGN: Sign = Libra;
pub const THEMA_MUNDI_VENUS_DEGREES: f64 = 15.0;
pub const THEMA_MUNDI_VENUS_SIGN_DEGREE: SignDegree =
    SignDegree::from_sign_and_degree30_f64(THEMA_MUNDI_VENUS_SIGN, THEMA_MUNDI_VENUS_DEGREES);
pub const THEMA_MUNDI_VENUS: BodySignDegree =
    BodySignDegree::new_const(Venus, THEMA_MUNDI_VENUS_SIGN_DEGREE);

pub const THEMA_MUNDI_MARS_SIGN: Sign = Scorpio;
pub const THEMA_MUNDI_MARS_DEGREES: f64 = 15.0;
pub const THEMA_MUNDI_MARS_SIGN_DEGREE: SignDegree =
    SignDegree::from_sign_and_degree30_f64(THEMA_MUNDI_MARS_SIGN, THEMA_MUNDI_MARS_DEGREES);
pub const THEMA_MUNDI_MARS: BodySignDegree =
    BodySignDegree::new_const(Mars, THEMA_MUNDI_MARS_SIGN_DEGREE);

pub const THEMA_MUNDI_JUPITER_SIGN: Sign = Sagittarius;
pub const THEMA_MUNDI_JUPITER_DEGREES: f64 = 15.0;
pub const THEMA_MUNDI_JUPITER_SIGN_DEGREE: SignDegree =
    SignDegree::from_sign_and_degree30_f64(THEMA_MUNDI_JUPITER_SIGN, THEMA_MUNDI_JUPITER_DEGREES);
pub const THEMA_MUNDI_JUPITER: BodySignDegree =
    BodySignDegree::new_const(Jupiter, THEMA_MUNDI_JUPITER_SIGN_DEGREE);

pub const THEMA_MUNDI_SATURN_SIGN: Sign = Capricorn;
pub const THEMA_MUNDI_SATURN_DEGREES: f64 = 15.0;
pub const THEMA_MUNDI_SATURN_SIGN_DEGREE: SignDegree =
    SignDegree::from_sign_and_degree30_f64(THEMA_MUNDI_SATURN_SIGN, THEMA_MUNDI_SATURN_DEGREES);
pub const THEMA_MUNDI_SATURN: BodySignDegree =
    BodySignDegree::new_const(Saturn, THEMA_MUNDI_SATURN_SIGN_DEGREE);

pub const THEMA_MUNDI_ASC_SIGN: Sign = Cancer;
pub const THEMA_MUNDI_ASC_DEGREES: f64 = 15.0;
pub const THEMA_MUNDI_ASC_SIGN_DEGREE: SignDegree =
    SignDegree::from_sign_and_degree30_f64(THEMA_MUNDI_ASC_SIGN, THEMA_MUNDI_ASC_DEGREES);

//pub const THEMA_MUNDI_ASC: BodySignDegree
//= BodySignDegree::new_const(Asc, THEMA_MUNDI__SIGN_DEGREE);
