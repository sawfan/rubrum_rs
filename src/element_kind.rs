use super::*;
use crate::ParseKeyError;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use strum_macros::{Display, EnumIter, IntoStaticStr};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ElementFormat {
    /// Stable storage/API key such as `"fire"`.
    Key,

    /// Human-readable name such as `"Fire"`.
    Name,

    /// General symbol/emoji such as `"🔥"`.
    Symbol,

    /// Alchemical symbol such as `"🜂"`.
    AlchemySymbol,

    /// Rust enum variant name such as `"Fire"`.
    Debug,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Display,
    PartialEq,
    Eq,
    EnumIter,
    IntoStaticStr,
    Hash,
    Serialize,
    Deserialize,
)]
pub enum ElementKind {
    Fire,
    Water,
    Air,
    Earth,
}

const EMBEDDED_ELEMENT_SYMBOLS: &str = include_str!("../config/element_symbols.toml");

static ELEMENT_NAME_MAP: Lazy<HashMap<String, String>> = Lazy::new(|| {
    load_toml_string_map(
        Path::new("config/element_symbols.toml"),
        EMBEDDED_ELEMENT_SYMBOLS,
        "name",
    )
});

static ELEMENT_SYMBOL_MAP: Lazy<HashMap<String, String>> = Lazy::new(|| {
    load_toml_string_map(
        Path::new("config/element_symbols.toml"),
        EMBEDDED_ELEMENT_SYMBOLS,
        "symbol",
    )
});

static ELEMENT_ALCHEMY_SYMBOL_MAP: Lazy<HashMap<String, String>> = Lazy::new(|| {
    load_toml_string_map(
        Path::new("config/element_symbols.toml"),
        EMBEDDED_ELEMENT_SYMBOLS,
        "alchemy_symbol",
    )
});

impl ElementKind {
    #[inline]
    pub const fn canonical_key(self) -> &'static str {
        match self {
            ElementKind::Fire => "fire",
            ElementKind::Water => "water",
            ElementKind::Air => "air",
            ElementKind::Earth => "earth",
        }
    }

    pub fn from_canonical_key(s: &str) -> Option<Self> {
        match s {
            "fire" => Some(ElementKind::Fire),
            "water" => Some(ElementKind::Water),
            "air" => Some(ElementKind::Air),
            "earth" => Some(ElementKind::Earth),
            _ => None,
        }
    }

    /// Formats an `ElementKind` using the requested format.
    ///
    /// This method supports a config-based override layer via `config/element_symbols.toml`.
    /// If an override entry is missing, it falls back to built-in defaults.
    pub fn format_element(self, fmt: ElementFormat) -> String {
        let key = self.canonical_key();

        let from_config = match fmt {
            ElementFormat::Key | ElementFormat::Debug => None,
            ElementFormat::Name => ELEMENT_NAME_MAP.get(key).cloned(),
            ElementFormat::Symbol => ELEMENT_SYMBOL_MAP.get(key).cloned(),
            ElementFormat::AlchemySymbol => ELEMENT_ALCHEMY_SYMBOL_MAP.get(key).cloned(),
        };

        if let Some(v) = from_config {
            return v;
        }

        // Built-in fallback values.
        match fmt {
            ElementFormat::Key => self.canonical_key().to_owned(),
            ElementFormat::Name => match self {
                ElementKind::Fire => "Fire".to_owned(),
                ElementKind::Water => "Water".to_owned(),
                ElementKind::Air => "Air".to_owned(),
                ElementKind::Earth => "Earth".to_owned(),
            },
            ElementFormat::Symbol => match self {
                ElementKind::Fire => "🔥".to_owned(),
                ElementKind::Water => "🌊".to_owned(),
                ElementKind::Air => "🌪️".to_owned(),
                ElementKind::Earth => "⛰️".to_owned(),
            },
            ElementFormat::AlchemySymbol => match self {
                ElementKind::Fire => "🜂".to_owned(),
                ElementKind::Water => "🜄".to_owned(),
                ElementKind::Air => "🜁".to_owned(),
                ElementKind::Earth => "🜃".to_owned(),
            },
            ElementFormat::Debug => format!("{:?}", self),
        }
    }
}

impl std::str::FromStr for ElementKind {
    type Err = ParseKeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_canonical_key(s).ok_or_else(|| ParseKeyError::new("ElementKind", s))
    }
}

impl TryFrom<&str> for ElementKind {
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
        assert_eq!(ElementKind::Fire.canonical_key(), "fire");
        assert_eq!(ElementKind::Earth.canonical_key(), "earth");
    }

    #[test]
    fn format_element_fallbacks_or_config() {
        // These should always yield something sensible, regardless of config availability.
        assert_eq!(
            ElementKind::Fire.format_element(ElementFormat::Name),
            "Fire"
        );
        assert_eq!(
            ElementKind::Water.format_element(ElementFormat::Symbol),
            "🌊"
        );
        assert_eq!(
            ElementKind::Air.format_element(ElementFormat::AlchemySymbol),
            "🜁"
        );
    }

    #[test]
    fn format_element_supports_multiple_screen_representations() {
        assert_eq!(ElementKind::Fire.format_element(ElementFormat::Key), "fire");
        assert_eq!(
            ElementKind::Fire.format_element(ElementFormat::Name),
            "Fire"
        );
        assert_eq!(
            ElementKind::Fire.format_element(ElementFormat::Symbol),
            "🔥"
        );
        assert_eq!(
            ElementKind::Fire.format_element(ElementFormat::AlchemySymbol),
            "🜂"
        );
        assert_eq!(
            ElementKind::Fire.format_element(ElementFormat::Debug),
            "Fire"
        );
    }

    #[test]
    fn canonical_key_round_trips_with_from_str() {
        use strum::IntoEnumIterator;

        for element in ElementKind::iter() {
            assert_eq!(
                ElementKind::from_canonical_key(element.canonical_key()),
                Some(element)
            );
            assert_eq!(element.canonical_key().parse::<ElementKind>(), Ok(element));
        }

        let err = "not_an_element".parse::<ElementKind>().unwrap_err();
        assert_eq!(err, ParseKeyError::new("ElementKind", "not_an_element"));
    }
}
