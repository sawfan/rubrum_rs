use super::*;

pub const SUN_EXALTATION_SIGN: Sign = Aries;
pub const MOON_EXALTATION_SIGN: Sign = Taurus;
pub const MERCURY_EXALTATION_SIGN: Sign = Virgo;
pub const VENUS_EXALTATION_SIGN: Sign = Pisces;
pub const MARS_EXALTATION_SIGN: Sign = Capricorn;
pub const JUPITER_EXALTATION_SIGN: Sign = Cancer;
pub const SATURN_EXALTATION_SIGN: Sign = Libra;

pub const SUN_EXALTATION_F64: f64 = 19.0;
pub const MOON_EXALTATION_F64: f64 = 3.0;
pub const MERCURY_EXALTATION_F64: f64 = 15.0;
pub const VENUS_EXALTATION_F64: f64 = 27.0;
pub const MARS_EXALTATION_F64: f64 = 28.0;
pub const JUPITER_EXALTATION_F64: f64 = 15.0;
pub const SATURN_EXALTATION_F64: f64 = 21.0;

pub const SUN_EXALTATION: SignDegree =
    SignDegree::from_sign_and_degree30_f64(SUN_EXALTATION_SIGN, SUN_EXALTATION_F64);

pub const MOON_EXALTATION: SignDegree =
    SignDegree::from_sign_and_degree30_f64(MOON_EXALTATION_SIGN, MOON_EXALTATION_F64);

pub const MERCURY_EXALTATION: SignDegree =
    SignDegree::from_sign_and_degree30_f64(MERCURY_EXALTATION_SIGN, MERCURY_EXALTATION_F64);

pub const VENUS_EXALTATION: SignDegree =
    SignDegree::from_sign_and_degree30_f64(VENUS_EXALTATION_SIGN, VENUS_EXALTATION_F64);

pub const MARS_EXALTATION: SignDegree =
    SignDegree::from_sign_and_degree30_f64(MARS_EXALTATION_SIGN, MARS_EXALTATION_F64);

pub const JUPITER_EXALTATION: SignDegree =
    SignDegree::from_sign_and_degree30_f64(JUPITER_EXALTATION_SIGN, JUPITER_EXALTATION_F64);

pub const SATURN_EXALTATION: SignDegree =
    SignDegree::from_sign_and_degree30_f64(SATURN_EXALTATION_SIGN, SATURN_EXALTATION_F64);
