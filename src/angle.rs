use crate::ParseKeyError;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use strum_macros::EnumIter;
use toml::Value;

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Serialize, Deserialize, EnumIter)]
pub enum AngleFormat {
    /// Stable storage/API key such as `"ascendant"`.
    Key,

    /// Human-readable English name such as `"Ascendant"`.
    Name,

    /// Short label or glyph from symbol config such as `"Asc"`.
    Symbol,

    /// Rust enum variant name such as `"Ascendant"`.
    Debug,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Serialize, Deserialize, EnumIter)]
pub enum Angle {
    Ascendant,
    Midheaven,
    Descendant,
    ImumCoeli,
    Vertex,
    AntiVertex,
}

const EMBEDDED_ANGLE_SYMBOLS: &str = include_str!("../config/angle_symbols.toml");

static ANGLE_SYMBOLS: Lazy<HashMap<String, String>> = Lazy::new(|| {
    // Prefer a runtime override from the current working directory (useful for apps), but fall
    // back to the embedded rubrum defaults when the consumer doesn't provide config files.
    let config_path = Path::new("config/angle_symbols.toml");
    let contents =
        fs::read_to_string(config_path).unwrap_or_else(|_| EMBEDDED_ANGLE_SYMBOLS.to_owned());

    let value = contents.parse::<Value>().unwrap_or_else(|_| {
        EMBEDDED_ANGLE_SYMBOLS
            .parse::<Value>()
            .unwrap_or(Value::Table(Default::default()))
    });

    let mut map = HashMap::new();
    if let Value::Table(tbl) = value {
        if let Some(Value::Table(symbols)) = tbl.get("symbols") {
            for (k, v) in symbols {
                if let Value::String(s) = v {
                    map.insert(k.clone(), s.clone());
                }
            }
        }
    }
    map
});

pub fn try_angle_symbol_text(angle: &Angle) -> Option<String> {
    let key = format!("{:?}", angle);
    ANGLE_SYMBOLS.get(&key).cloned()
}

pub fn angle_symbol_text(angle: &Angle) -> String {
    try_angle_symbol_text(angle).unwrap_or_else(|| angle.name().to_owned())
}

use std::fmt;
impl fmt::Display for Angle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", AngleSymbolText(self))
    }
}

pub struct AngleSymbolText<'a>(&'a Angle);
impl<'a> fmt::Display for AngleSymbolText<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.symbol_text())
    }
}

impl Angle {
    #[inline]
    pub const fn canonical_key(self) -> &'static str {
        match self {
            Angle::Ascendant => "ascendant",
            Angle::Midheaven => "midheaven",
            Angle::Descendant => "descendant",
            Angle::ImumCoeli => "imum_coeli",
            Angle::Vertex => "vertex",
            Angle::AntiVertex => "antivertex",
        }
    }

    pub fn from_canonical_key(s: &str) -> Option<Self> {
        match s {
            "ascendant" => Some(Angle::Ascendant),
            "midheaven" => Some(Angle::Midheaven),
            "descendant" => Some(Angle::Descendant),
            "imum_coeli" => Some(Angle::ImumCoeli),
            "vertex" => Some(Angle::Vertex),
            "antivertex" => Some(Angle::AntiVertex),
            _ => None,
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Angle::Ascendant => "Ascendant",
            Angle::Midheaven => "Midheaven",
            Angle::Descendant => "Descendant",
            Angle::ImumCoeli => "Imum Coeli",
            Angle::Vertex => "Vertex",
            Angle::AntiVertex => "Anti-Vertex",
        }
    }

    pub fn format_angle(self, fmt: AngleFormat) -> String {
        match fmt {
            AngleFormat::Key => self.canonical_key().to_owned(),
            AngleFormat::Name => self.name().to_owned(),
            AngleFormat::Symbol => self.symbol_text(),
            AngleFormat::Debug => format!("{:?}", self),
        }
    }

    pub fn symbol_text(&self) -> String {
        angle_symbol_text(self)
    }
}

impl std::str::FromStr for Angle {
    type Err = ParseKeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_canonical_key(s).ok_or_else(|| ParseKeyError::new("Angle", s))
    }
}

impl TryFrom<&str> for Angle {
    type Error = ParseKeyError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_key_basics() {
        assert_eq!(Angle::Ascendant.canonical_key(), "ascendant");
        assert_eq!(Angle::Midheaven.canonical_key(), "midheaven");
        assert_eq!(Angle::Vertex.canonical_key(), "vertex");
    }

    #[test]
    fn canonical_key_round_trips_with_from_str() {
        use strum::IntoEnumIterator;

        for angle in Angle::iter() {
            assert_eq!(
                Angle::from_canonical_key(angle.canonical_key()),
                Some(angle)
            );
            assert_eq!(angle.canonical_key().parse::<Angle>(), Ok(angle));
        }

        let err = "not_an_angle".parse::<Angle>().unwrap_err();
        assert_eq!(err, ParseKeyError::new("Angle", "not_an_angle"));
    }

    #[test]
    fn format_angle_supports_multiple_screen_representations() {
        assert_eq!(Angle::Ascendant.format_angle(AngleFormat::Key), "ascendant");
        assert_eq!(
            Angle::Ascendant.format_angle(AngleFormat::Name),
            "Ascendant"
        );
        assert_eq!(Angle::Ascendant.format_angle(AngleFormat::Symbol), "Asc");
        assert_eq!(
            Angle::Ascendant.format_angle(AngleFormat::Debug),
            "Ascendant"
        );
    }
}
