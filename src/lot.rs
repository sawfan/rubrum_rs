use crate::ParseKeyError;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use strum_macros::EnumIter;
use toml::Value;

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Serialize, Deserialize, EnumIter)]
pub enum LotFormat {
    /// Stable storage/API key such as `"fortune"`.
    Key,

    /// Human-readable English name such as `"Part of Fortune"`.
    Name,

    /// Short label or glyph from symbol config.
    Symbol,

    /// Rust enum variant name such as `"Fortune"`.
    Debug,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Serialize, Deserialize, EnumIter)]
pub enum Lot {
    /// Part of Fortune.
    Fortune,

    /// Part of Spirit.
    Spirit,
}

impl Lot {
    #[inline]
    pub const fn canonical_key(self) -> &'static str {
        match self {
            Lot::Fortune => "fortune",
            Lot::Spirit => "spirit",
        }
    }

    pub fn from_canonical_key(s: &str) -> Option<Self> {
        match s {
            "fortune" => Some(Lot::Fortune),
            "spirit" => Some(Lot::Spirit),
            _ => None,
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Lot::Fortune => "Part of Fortune",
            Lot::Spirit => "Part of Spirit",
        }
    }

    pub fn format_lot(self, fmt: LotFormat) -> String {
        match fmt {
            LotFormat::Key => self.canonical_key().to_owned(),
            LotFormat::Name => self.name().to_owned(),
            LotFormat::Symbol => self.symbol_text(),
            LotFormat::Debug => format!("{:?}", self),
        }
    }

    pub fn symbol_text(&self) -> String {
        lot_symbol_text(self)
    }
}

const EMBEDDED_LOT_SYMBOLS: &str = include_str!("../config/lot_symbols.toml");

static LOT_SYMBOLS: Lazy<HashMap<String, String>> = Lazy::new(|| {
    // Prefer a runtime override from the current working directory (useful for apps), but fall
    // back to the embedded rubrum defaults when the consumer doesn't provide config files.
    let config_path = Path::new("config/lot_symbols.toml");
    let contents =
        fs::read_to_string(config_path).unwrap_or_else(|_| EMBEDDED_LOT_SYMBOLS.to_owned());

    let value = contents.parse::<Value>().unwrap_or_else(|_| {
        EMBEDDED_LOT_SYMBOLS
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

pub fn try_lot_symbol_text(lot: &Lot) -> Option<String> {
    let key = format!("{:?}", lot);
    LOT_SYMBOLS.get(&key).cloned()
}

pub fn lot_symbol_text(lot: &Lot) -> String {
    try_lot_symbol_text(lot).unwrap_or_else(|| lot.name().to_owned())
}

use std::fmt;
impl fmt::Display for Lot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", LotSymbolText(self))
    }
}

pub struct LotSymbolText<'a>(&'a Lot);
impl<'a> fmt::Display for LotSymbolText<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.symbol_text())
    }
}

impl std::str::FromStr for Lot {
    type Err = ParseKeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_canonical_key(s).ok_or_else(|| ParseKeyError::new("Lot", s))
    }
}

impl TryFrom<&str> for Lot {
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
        assert_eq!(Lot::Fortune.canonical_key(), "fortune");
        assert_eq!(Lot::Spirit.canonical_key(), "spirit");
    }

    #[test]
    fn canonical_key_round_trips_with_from_str() {
        use strum::IntoEnumIterator;

        for lot in Lot::iter() {
            assert_eq!(Lot::from_canonical_key(lot.canonical_key()), Some(lot));
            assert_eq!(lot.canonical_key().parse::<Lot>(), Ok(lot));
        }

        let err = "not_a_lot".parse::<Lot>().unwrap_err();
        assert_eq!(err, ParseKeyError::new("Lot", "not_a_lot"));
    }

    #[test]
    fn format_lot_supports_multiple_screen_representations() {
        assert_eq!(Lot::Fortune.format_lot(LotFormat::Key), "fortune");
        assert_eq!(Lot::Fortune.format_lot(LotFormat::Name), "Part of Fortune");
        assert_eq!(Lot::Fortune.format_lot(LotFormat::Symbol), "⊗");
        assert_eq!(Lot::Fortune.format_lot(LotFormat::Debug), "Fortune");
    }
}
