use super::*;

pub mod exaltation_constants;
pub use exaltation_constants::*;

pub mod thema_mundi;
pub use thema_mundi::*;

pub mod aspect_constants;
pub use aspect_constants::*;

pub mod body_group_constants;
pub use body_group_constants::*;

pub const MAX_LESS_THAN_1: f64 = f64::from_bits(1.0f64.to_bits() - 1);
pub const MAX_LESS_THAN_30: f64 = f64::from_bits(30.0f64.to_bits() - 1);
pub const MAX_LESS_THAN_60: f64 = f64::from_bits(60.0f64.to_bits() - 1);

pub const SIGNS_COUNT: i32 = 12;
pub const SIGN_DEGREES: f64 = 30.0;
pub const DEGREES_IN_SIGN: f64 = 30.0;
pub const MAX_DEGREE: f64 = MAX_LESS_THAN_30; // 29.9999999999...
pub const CIRCLE_DEGREES: f64 = 360.0;
pub const MAX_LESS_THAN_360: f64 = f64::from_bits(CIRCLE_DEGREES.to_bits() - 1);
pub const MINUTES_IN_DEGREE: f64 = 60.0;
pub const SECONDS_IN_MINUTE: f64 = 60.0;
pub const THIRDS_IN_SECOND: f64 = 60.0;
pub const FOURTHS_IN_THIRD: f64 = 60.0;

pub const ARIES_START: f64 = 0.0; // 0.0000...
pub const ARIES_END: f64 = ARIES_START + MAX_DEGREE; // 29.999...
pub const TAURUS_START: f64 = ARIES_START + DEGREES_IN_SIGN; // 30.000...
pub const TAURUS_END: f64 = TAURUS_START + MAX_DEGREE; // 59.999...
pub const GEMINI_START: f64 = TAURUS_START + DEGREES_IN_SIGN; // 60.000...
pub const GEMINI_END: f64 = GEMINI_START + MAX_DEGREE; // 89.000...
pub const CANCER_START: f64 = GEMINI_START + DEGREES_IN_SIGN; // 90.000...
pub const CANCER_END: f64 = CANCER_START + MAX_DEGREE;
pub const LEO_START: f64 = CANCER_START + DEGREES_IN_SIGN;
pub const LEO_END: f64 = LEO_START + MAX_DEGREE;
pub const VIRGO_START: f64 = LEO_START + DEGREES_IN_SIGN;
pub const VIRGO_END: f64 = VIRGO_START + MAX_DEGREE;
pub const LIBRA_START: f64 = VIRGO_START + DEGREES_IN_SIGN;
pub const LIBRA_END: f64 = LIBRA_START + MAX_DEGREE;
pub const SCORPIO_START: f64 = LIBRA_START + DEGREES_IN_SIGN;
pub const SCORPIO_END: f64 = SCORPIO_START + MAX_DEGREE;
pub const SAGITTARIUS_START: f64 = SCORPIO_START + DEGREES_IN_SIGN;
pub const SAGITTARIUS_END: f64 = SAGITTARIUS_START + MAX_DEGREE;
pub const CAPRICORN_START: f64 = SAGITTARIUS_START + DEGREES_IN_SIGN;
pub const CAPRICORN_END: f64 = CAPRICORN_START + MAX_DEGREE;
pub const AQUARIUS_START: f64 = CAPRICORN_START + DEGREES_IN_SIGN;
pub const AQUARIUS_END: f64 = AQUARIUS_START + MAX_DEGREE;
pub const PISCES_START: f64 = AQUARIUS_START + DEGREES_IN_SIGN;
pub const PISCES_END: f64 = PISCES_START + MAX_DEGREE;

use Sign::*;
pub const SUN_RULED_SIGNS: [Sign; 1] = [Leo];
pub const MOON_RULED_SIGNS: [Sign; 1] = [Cancer];
pub const MERCURY_RULED_SIGNS: [Sign; 2] = [Gemini, Virgo];
pub const VENUS_RULED_SIGNS: [Sign; 2] = [Taurus, Libra];
pub const MARS_RULED_SIGNS: [Sign; 2] = [Aries, Scorpio];
pub const JUPITER_RULED_SIGNS: [Sign; 2] = [Sagittarius, Pisces];
pub const SATURN_RULED_SIGNS: [Sign; 2] = [Capricorn, Aquarius];
pub const URANUS_RULED_SIGNS: [Sign; 1] = [Aquarius];
//pub const NEPTUNE_RULED_SIGNS   : [Sign;0] = [];
pub const PLUTO_RULED_SIGNS: [Sign; 1] = [Scorpio];

pub const SUN_DETRIMENT_SIGNS: [Sign; 1] = [Aquarius];
pub const MOON_DETRIMENT_SIGNS: [Sign; 1] = [Capricorn];
pub const MERCURY_DETRIMENT_SIGNS: [Sign; 2] = [Sagittarius, Pisces];
pub const VENUS_DETRIMENT_SIGNS: [Sign; 2] = [Aries, Scorpio];
pub const MARS_DETRIMENT_SIGNS: [Sign; 2] = [Libra, Taurus];
pub const JUPITER_DETRIMENT_SIGNS: [Sign; 2] = [Gemini, Virgo];
pub const SATURN_DETRIMENT_SIGNS: [Sign; 2] = [Cancer, Leo];
//pub const URANUS_DETRIMENT_SIGNS    : [Sign;1] = [];
//pub const NEPTUNE_DETRIMENT_SIGNS   : [Sign;0] = [];
//pub const PLUTO_DETRIMENT_SIGNS     : [Sign;1] = [];

pub const SUN_FALL_SIGN: Sign = Libra;
pub const MOON_FALL_SIGN: Sign = Scorpio;
pub const MERCURY_FALL_SIGN: Sign = Pisces;
pub const VENUS_FALL_SIGN: Sign = Virgo;
pub const MARS_FALL_SIGN: Sign = Cancer;
pub const JUPITER_FALL_SIGN: Sign = Capricorn;
pub const SATURN_FALL_SIGN: Sign = Aries;

//pub const SUN_FALL : SignDegree = SignDegree::from_sign_and_degree30_f64(
//    SUN_FALL_SIGN,
//    SUN_FALL_F64
//);
//
//pub const MOON_FALL : SignDegree = SignDegree::from_sign_and_degree30_f64(
//    MOON_FALL_SIGN,
//    MOON_FALL_F64
//);
//
//pub const MERCURY_FALL : SignDegree = SignDegree::from_sign_and_degree30_f64(
//    MERCURY_FALL_SIGN,
//    MERCURY_FALL_F64
//);
//
//pub const VENUS_FALL : SignDegree = SignDegree::from_sign_and_degree30_f64(
//    VENUS_FALL_SIGN,
//    VENUS_FALL_F64
//);
//
//pub const MARS_FALL : SignDegree = SignDegree::from_sign_and_degree30_f64(
//    MARS_FALL_SIGN,
//    MARS_FALL_F64
//);
//
//pub const JUPITER_FALL : SignDegree = SignDegree::from_sign_and_degree30_f64(
//    JUPITER_FALL_SIGN,
//    JUPITER_FALL_F64
//);
//
//pub const SATURN_FALL : SignDegree = SignDegree::from_sign_and_degree30_f64(
//    SATURN_FALL_SIGN,
//    SATURN_FALL_F64
//);

use Body::*;
pub const LEO_RULING_BODY: Body = Sun;
pub const CANCER_RULING_BODY: Body = Moon;
pub const GEMINI_RULING_BODY: Body = Mercury;
pub const VIRGO_RULING_BODY: Body = Mercury;
pub const TAURUS_RULING_BODY: Body = Venus;
pub const LIBRA_RULING_BODY: Body = Venus;
pub const ARIES_RULING_BODY: Body = Mars;
pub const SCORPIO_RULING_BODY: Body = Mars;
pub const SAGITTARIUS_RULING_BODY: Body = Jupiter;
pub const PISCES_RULING_BODY: Body = Jupiter;
pub const CAPRICTORN_RULING_BODY: Body = Saturn;
pub const AQUARIUS_RULING_BODY: Body = Saturn;
//pub const AQUARIUS_RULING_BODY    : Body = Uranus;
//
pub const FIRE_SIGNS: [Sign; 3] = [Aries, Leo, Sagittarius];
pub const EARTH_SIGNS: [Sign; 3] = [Taurus, Virgo, Capricorn];
pub const AIR_SIGNS: [Sign; 3] = [Gemini, Libra, Aquarius];
pub const WATER_SIGNS: [Sign; 3] = [Cancer, Scorpio, Pisces];

//Fire — Aries, Leo, Sagittarius - hot, dry
//Earth — Taurus, Virgo, Capricorn - cold, dry
//Air — Gemini, Libra, Aquarius - hot, wet
//Water — Cancer, Scorpio, Pisces - cold, wet

//pub const MERCURY_EXALTATION
//pub const MERCURY_EXALTATION_DEGREE

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_less_than_constants_are_previous_representable_f64s() {
        assert!(MAX_LESS_THAN_1 < 1.0);
        assert_eq!(MAX_LESS_THAN_1.to_bits() + 1, 1.0f64.to_bits());

        assert!(MAX_LESS_THAN_30 < 30.0);
        assert_eq!(MAX_LESS_THAN_30.to_bits() + 1, 30.0f64.to_bits());

        assert!(MAX_LESS_THAN_60 < 60.0);
        assert_eq!(MAX_LESS_THAN_60.to_bits() + 1, 60.0f64.to_bits());

        assert!(MAX_LESS_THAN_360 < 360.0);
        assert_eq!(MAX_LESS_THAN_360.to_bits() + 1, 360.0f64.to_bits());
    }
}
