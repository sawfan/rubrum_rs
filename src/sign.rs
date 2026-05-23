use super::*;

use crate::ParseKeyError;
use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use Sign::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SignFormat {
    /// Stable storage/API key such as `"aries"`.
    Key,

    /// Human-readable English name such as `"Aries"`.
    Name,

    /// Zodiac glyph such as `"♈"`.
    Symbol,

    /// Rust enum variant name such as `"Aries"`.
    Debug,
}

#[derive(
    Debug, Clone, PartialEq, Eq, Copy, Hash, Serialize, Deserialize, IntoPrimitive, TryFromPrimitive,
)]
#[repr(u8)]
pub enum Sign {
    Aries = 0,
    Taurus,
    Gemini,
    Cancer,
    Leo,
    Virgo,
    Libra,
    Scorpio,
    Sagittarius,
    Capricorn,
    Aquarius,
    Pisces,
}

impl Sign {
    #[inline]
    pub const fn canonical_key(self) -> &'static str {
        match self {
            Sign::Aries => "aries",
            Sign::Taurus => "taurus",
            Sign::Gemini => "gemini",
            Sign::Cancer => "cancer",
            Sign::Leo => "leo",
            Sign::Virgo => "virgo",
            Sign::Libra => "libra",
            Sign::Scorpio => "scorpio",
            Sign::Sagittarius => "sagittarius",
            Sign::Capricorn => "capricorn",
            Sign::Aquarius => "aquarius",
            Sign::Pisces => "pisces",
        }
    }

    pub fn from_canonical_key(s: &str) -> Option<Self> {
        match s {
            "aries" => Some(Sign::Aries),
            "taurus" => Some(Sign::Taurus),
            "gemini" => Some(Sign::Gemini),
            "cancer" => Some(Sign::Cancer),
            "leo" => Some(Sign::Leo),
            "virgo" => Some(Sign::Virgo),
            "libra" => Some(Sign::Libra),
            "scorpio" => Some(Sign::Scorpio),
            "sagittarius" => Some(Sign::Sagittarius),
            "capricorn" => Some(Sign::Capricorn),
            "aquarius" => Some(Sign::Aquarius),
            "pisces" => Some(Sign::Pisces),
            _ => None,
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Sign::Aries => "Aries",
            Sign::Taurus => "Taurus",
            Sign::Gemini => "Gemini",
            Sign::Cancer => "Cancer",
            Sign::Leo => "Leo",
            Sign::Virgo => "Virgo",
            Sign::Libra => "Libra",
            Sign::Scorpio => "Scorpio",
            Sign::Sagittarius => "Sagittarius",
            Sign::Capricorn => "Capricorn",
            Sign::Aquarius => "Aquarius",
            Sign::Pisces => "Pisces",
        }
    }

    pub fn format_sign(self, fmt: SignFormat) -> String {
        match fmt {
            SignFormat::Key => self.canonical_key().to_owned(),
            SignFormat::Name => self.name().to_owned(),
            SignFormat::Symbol => sign_emoji::sign_symbol_text(&self),
            SignFormat::Debug => format!("{:?}", self),
        }
    }

    pub fn default_native() -> Sign {
        Sign::Aries
    }

    pub fn default_astrological_native() -> Sign {
        Sign::Leo
    }

    pub fn default_native_order() -> Vec<Sign> {
        Self::default_native().native_order()
    }

    pub fn native_order(&self) -> Vec<Sign> {
        let rising = *self;
        let mut next = rising.next();
        let mut ordered = vec![rising];

        for _ in 0..11 {
            ordered.push(next);
            next = next.next();
        }

        ordered
    }

    pub fn next(&self) -> Sign {
        match self {
            Aries => Taurus,
            Taurus => Gemini,
            Gemini => Cancer,
            Cancer => Leo,
            Leo => Virgo,
            Virgo => Libra,
            Libra => Scorpio,
            Scorpio => Sagittarius,
            Sagittarius => Capricorn,
            Capricorn => Aquarius,
            Aquarius => Pisces,
            Pisces => Aries,
        }
    }

    pub fn symbol_text(&self) -> String {
        sign_emoji::sign_symbol_text(self)
    }

    pub const fn start_degree_f64(&self) -> f64 {
        match self {
            Aries => ARIES_START,
            Taurus => TAURUS_START,
            Gemini => GEMINI_START,
            Cancer => CANCER_START,
            Leo => LEO_START,
            Virgo => VIRGO_START,
            Libra => LIBRA_START,
            Scorpio => SCORPIO_START,
            Sagittarius => SAGITTARIUS_START,
            Capricorn => CAPRICORN_START,
            Aquarius => AQUARIUS_START,
            Pisces => PISCES_START,
        }
    }
}

impl std::str::FromStr for Sign {
    type Err = ParseKeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_canonical_key(s).ok_or_else(|| ParseKeyError::new("Sign", s))
    }
}

impl TryFrom<&str> for Sign {
    type Error = ParseKeyError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

use std::fmt;
impl fmt::Display for Sign {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}

    #[test]
    fn canonical_key_round_trips_with_from_str() {
        let signs = [
            Sign::Aries,
            Sign::Taurus,
            Sign::Gemini,
            Sign::Cancer,
            Sign::Leo,
            Sign::Virgo,
            Sign::Libra,
            Sign::Scorpio,
            Sign::Sagittarius,
            Sign::Capricorn,
            Sign::Aquarius,
            Sign::Pisces,
        ];

        for sign in signs {
            assert_eq!(Sign::from_canonical_key(sign.canonical_key()), Some(sign));
            assert_eq!(sign.canonical_key().parse::<Sign>(), Ok(sign));
        }

        let err = "not_a_sign".parse::<Sign>().unwrap_err();
        assert_eq!(err, ParseKeyError::new("Sign", "not_a_sign"));
    }

    #[test]
    fn format_sign_supports_multiple_screen_representations() {
        assert_eq!(Sign::Aries.format_sign(SignFormat::Key), "aries");
        assert_eq!(Sign::Aries.format_sign(SignFormat::Name), "Aries");
        assert_eq!(Sign::Aries.format_sign(SignFormat::Symbol), "♈");
        assert_eq!(Sign::Aries.format_sign(SignFormat::Debug), "Aries");
    }
}
