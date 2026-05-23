use super::*;
use crate::constants::aspect_constants::*;
use crate::ParseKeyError;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use DegreeAspectKind::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
pub enum DegreeAspectKindFormat {
    /// Stable storage/API key such as `"trine"`.
    Key,

    /// Human-readable English name such as `"Trine"`.
    Name,

    /// Aspect glyph/short symbol such as `"△"`.
    Symbol,

    /// Exact aspect angle in degrees such as `"120°"`.
    Degrees,

    /// Rust enum variant name such as `"Trine"`.
    Debug,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
#[serde(try_from = "String", into = "String")]
pub enum DegreeAspectKind {
    Conjunction,
    Vigintile,   // 	Also known as semidecile.
    Semisextile, //  	Also known as dodecile.
    Undecile,
    Decile,
    Novile,
    SemiSquare, // Also known as Octile
    Septile,
    Sextile,
    Quintile,
    Binovile,
    Square,
    Biseptile,
    Tredecile,
    Trine,
    Sesquiquadrate,
    Biquintile,
    Quincunx,
    Triseptile,
    Quadranovile,
    Opposition,
    Occultation,
    LunarEclipse,
}

impl DegreeAspectKind {
    #[inline]
    pub const fn canonical_key(&self) -> &'static str {
        match self {
            Conjunction => "conjunction",
            Vigintile => "vigintile",
            Semisextile => "semisextile",
            Undecile => "undecile",
            Decile => "decile",
            Novile => "novile",
            SemiSquare => "semisquare",
            Septile => "septile",
            Sextile => "sextile",
            Quintile => "quintile",
            Binovile => "binovile",
            Square => "square",
            Biseptile => "biseptile",
            Tredecile => "tredecile",
            Trine => "trine",
            Sesquiquadrate => "sesquiquadrate",
            Biquintile => "biquintile",
            Quincunx => "quincunx",
            Triseptile => "triseptile",
            Quadranovile => "quadranovile",
            Opposition => "opposition",
            Occultation => "occultation",
            LunarEclipse => "lunar_eclipse",
        }
    }

    pub fn from_canonical_key(s: &str) -> Option<Self> {
        match s {
            "conjunction" | "Conjunction" => Some(Conjunction),
            "vigintile" | "Vigintile" => Some(Vigintile),
            "semisextile" | "Semisextile" => Some(Semisextile),
            "undecile" | "Undecile" => Some(Undecile),
            "decile" | "Decile" => Some(Decile),
            "novile" | "Novile" => Some(Novile),
            "semisquare" | "semi_square" | "SemiSquare" | "Semi-Square" => Some(SemiSquare),
            "septile" | "Septile" => Some(Septile),
            "sextile" | "Sextile" => Some(Sextile),
            "quintile" | "Quintile" => Some(Quintile),
            "binovile" | "Binovile" => Some(Binovile),
            "square" | "Square" => Some(Square),
            "biseptile" | "Biseptile" => Some(Biseptile),
            "tredecile" | "Tredecile" => Some(Tredecile),
            "trine" | "Trine" => Some(Trine),
            "sesquiquadrate" | "Sesquiquadrate" => Some(Sesquiquadrate),
            "biquintile" | "Biquintile" => Some(Biquintile),
            "quincunx" | "Quincunx" => Some(Quincunx),
            "triseptile" | "Triseptile" => Some(Triseptile),
            "quadranovile" | "Quadranovile" => Some(Quadranovile),
            "opposition" | "Opposition" => Some(Opposition),
            "occultation" | "Occultation" => Some(Occultation),
            "lunar_eclipse" | "LunarEclipse" | "Lunar Eclipse" => Some(LunarEclipse),
            _ => None,
        }
    }

    pub const fn name(&self) -> &'static str {
        match self {
            Conjunction => "Conjunction",
            Vigintile => "Vigintile",
            Semisextile => "Semisextile",
            Undecile => "Undecile",
            Decile => "Decile",
            Novile => "Novile",
            SemiSquare => "Semi-Square",
            Septile => "Septile",
            Sextile => "Sextile",
            Quintile => "Quintile",
            Binovile => "Binovile",
            Square => "Square",
            Biseptile => "Biseptile",
            Tredecile => "Tredecile",
            Trine => "Trine",
            Sesquiquadrate => "Sesquiquadrate",
            Biquintile => "Biquintile",
            Quincunx => "Quincunx",
            Triseptile => "Triseptile",
            Quadranovile => "Quadranovile",
            Opposition => "Opposition",
            Occultation => "Occultation",
            LunarEclipse => "Lunar Eclipse",
        }
    }

    pub fn format_degree_aspect_kind(&self, fmt: DegreeAspectKindFormat) -> String {
        match fmt {
            DegreeAspectKindFormat::Key => self.canonical_key().to_owned(),
            DegreeAspectKindFormat::Name => self.name().to_owned(),
            DegreeAspectKindFormat::Symbol => self.symbol_text().to_owned(),
            DegreeAspectKindFormat::Degrees => format_degrees(self.aspect_kind_degree_f64()),
            DegreeAspectKindFormat::Debug => format!("{:?}", self),
        }
    }

    pub fn symbol_text(&self) -> &str {
        aspect_kind_symbol_text(self)
    }

    pub fn aspect_kind_degree_f64(&self) -> f64 {
        aspect_kind_degree_f64(self)
    }
}

fn format_degrees(degrees: f64) -> String {
    if degrees.fract().abs() < f64::EPSILON {
        format!("{}°", degrees as i64)
    } else {
        format!("{}°", degrees)
    }
}

pub fn aspect_kind_symbol_text(aspect: &DegreeAspectKind) -> &str {
    match aspect {
        Conjunction => "☌",
        Vigintile => "V",
        Semisextile => "⚺",
        Undecile => "U",
        Decile => "D",
        Novile => "N",
        SemiSquare => "∠",
        Septile => "S",
        Sextile => "⚹",
        Quintile => "Q",
        Binovile => "N2",
        Square => "□",
        Biseptile => "S2",
        Tredecile => "D3",
        Trine => "△",
        Sesquiquadrate => "⚼",
        Biquintile => "Q2",
        Quincunx => "⚻",
        Triseptile => "S3",
        Quadranovile => "N4",
        Opposition => "☍",
        Occultation => "🝵",
        LunarEclipse => "🝶",
    }
}

pub fn aspect_kind_text_description(aspect: &DegreeAspectKind) -> Option<&str> {
    match aspect {
        Conjunction => Some("Two or more planets in the same house (zodiacal sign)."),
        Semisextile => Some("One sign apart"),
        SemiSquare => Some("Half the angle of Square"),
        Sextile => Some("Two signs apart"),
        Square => Some("Three signs apart / Same modality"),
        Trine => Some("Four signs apart / Same elemental triplicity"),
        Quincunx => Some("Five signs apart"),
        Opposition => Some("Six signs apart"),
        Occultation => Some(
            "Conjunction with eclipse. Solar eclipse when the Sun and Moon are in conjunction.",
        ),
        LunarEclipse => Some(
            "Opposition with eclipse, or (rarely) any body in the shadow of the other. Lunar eclipse when the Sun and Moon are in opposition.",
        ),
        _ => None,
    }
}

pub fn aspect_kind_degree_f64(aspect: &DegreeAspectKind) -> f64 {
    match aspect {
        Conjunction => CONJUNCTION_DEGREES,
        Vigintile => VIGINTILE_DEGREES,
        Semisextile => SEMISEXTILE_DEGREES,
        Undecile => UNDECILE_DEGREES,
        Decile => DECILE_DEGREES,
        Novile => NOVILE_DEGREES,
        SemiSquare => SEMISQUARE_DEGREES,
        Septile => SEPTILE_DEGREES,
        Sextile => SEXTILE_DEGREES,
        Quintile => QUINTILE_DEGREES,
        Binovile => BINOVILE_DEGREES,
        Square => SQUARE_DEGREES,
        Biseptile => BISEPTILE_DEGREES,
        Tredecile => TREDECILE_DEGREES,
        Trine => TRINE_DEGREES,
        Sesquiquadrate => SESQUIQUADRATE_DEGREES,
        Biquintile => BIQUINTILE_DEGREES,
        Quincunx => QUINCUNX_DEGREES,
        Triseptile => TRISEPTILE_DEGREES,
        Quadranovile => QUADRANOVILE_DEGREES,
        Opposition => OPPOSITION_DEGREES,
        Occultation => OCCULTATION_DEGREES,
        LunarEclipse => LUNAR_ECLIPSE_DEGREES,
    }
}

use std::fmt;
impl fmt::Display for DegreeAspectKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::str::FromStr for DegreeAspectKind {
    type Err = ParseKeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_canonical_key(s).ok_or_else(|| ParseKeyError::new("DegreeAspectKind", s))
    }
}

impl TryFrom<&str> for DegreeAspectKind {
    type Error = ParseKeyError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl From<DegreeAspectKind> for String {
    fn from(value: DegreeAspectKind) -> Self {
        value.canonical_key().to_owned()
    }
}

impl TryFrom<String> for DegreeAspectKind {
    type Error = ParseKeyError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().parse()
    }
}

pub struct DegreeAspectKindSymbolText<'a>(pub &'a DegreeAspectKind);
impl<'a> fmt::Display for DegreeAspectKindSymbolText<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.symbol_text())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn degrees_are_non_negative() {
        for k in DegreeAspectKind::iter() {
            assert!(k.aspect_kind_degree_f64() >= 0.0);
        }
    }

    #[test]
    fn canonical_key_round_trips_with_from_str() {
        for k in DegreeAspectKind::iter() {
            assert_eq!(
                DegreeAspectKind::from_canonical_key(k.canonical_key()),
                Some(k.clone())
            );
            assert_eq!(k.canonical_key().parse::<DegreeAspectKind>(), Ok(k));
        }

        assert_eq!(
            "semi_square".parse::<DegreeAspectKind>(),
            Ok(DegreeAspectKind::SemiSquare)
        );

        let err = "not_an_aspect".parse::<DegreeAspectKind>().unwrap_err();
        assert_eq!(err, ParseKeyError::new("DegreeAspectKind", "not_an_aspect"));
    }

    #[test]
    fn serde_uses_canonical_keys_but_accepts_legacy_variant_names() {
        let json = serde_json::to_string(&DegreeAspectKind::Trine).unwrap();
        assert_eq!(json, "\"trine\"");

        let parsed: DegreeAspectKind = serde_json::from_str("\"Trine\"").unwrap();
        assert_eq!(parsed, DegreeAspectKind::Trine);
    }

    #[test]
    fn format_degree_aspect_kind_supports_multiple_screen_representations() {
        assert_eq!(
            DegreeAspectKind::Trine.format_degree_aspect_kind(DegreeAspectKindFormat::Key),
            "trine"
        );
        assert_eq!(
            DegreeAspectKind::Trine.format_degree_aspect_kind(DegreeAspectKindFormat::Name),
            "Trine"
        );
        assert_eq!(
            DegreeAspectKind::Trine.format_degree_aspect_kind(DegreeAspectKindFormat::Symbol),
            "△"
        );
        assert_eq!(
            DegreeAspectKind::Trine.format_degree_aspect_kind(DegreeAspectKindFormat::Degrees),
            "120°"
        );
        assert_eq!(
            DegreeAspectKind::Trine.format_degree_aspect_kind(DegreeAspectKindFormat::Debug),
            "Trine"
        );
    }
}
