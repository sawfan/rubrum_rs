use crate::ParseKeyError;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use strum_macros::EnumIter;
use toml::Value;

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Serialize, Deserialize, EnumIter)]
pub enum ChartPointFormat {
    /// Stable storage/API key such as `"mean_node"`.
    Key,

    /// Human-readable English name such as `"Mean Node"`.
    Name,

    /// Short label or glyph from symbol config such as `"☊"`.
    Symbol,

    /// Rust enum variant name such as `"MeanNode"`.
    Debug,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Serialize, Deserialize, EnumIter)]
pub enum ChartPoint {
    /// Ecliptic nutation point.
    EclNut,

    /// Mean lunar node.
    MeanNode,

    /// True lunar node.
    TrueNode,

    /// Mean lunar apogee ("Black Moon" / Lilith in some contexts).
    MeanApog,

    /// Osculating lunar apogee.
    OscuApog,

    /// Interpolated apogee.
    IntpApog,

    /// Interpolated perigee.
    IntpPerg,
}

const EMBEDDED_CHART_POINT_SYMBOLS: &str = include_str!("../config/chart_point_symbols.toml");

static CHART_POINT_SYMBOLS: Lazy<HashMap<String, String>> = Lazy::new(|| {
    // Prefer a runtime override from the current working directory (useful for apps), but fall
    // back to the embedded rubrum defaults when the consumer doesn't provide config files.
    let config_path = Path::new("config/chart_point_symbols.toml");
    let contents =
        fs::read_to_string(config_path).unwrap_or_else(|_| EMBEDDED_CHART_POINT_SYMBOLS.to_owned());

    let value = contents.parse::<Value>().unwrap_or_else(|_| {
        EMBEDDED_CHART_POINT_SYMBOLS
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

pub fn try_chart_point_symbol_text(point: &ChartPoint) -> Option<String> {
    let key = format!("{:?}", point);
    CHART_POINT_SYMBOLS.get(&key).cloned()
}

pub fn chart_point_symbol_text(point: &ChartPoint) -> String {
    try_chart_point_symbol_text(point).unwrap_or_else(|| point.name().to_owned())
}

use std::fmt;
impl fmt::Display for ChartPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ChartPointSymbolText(self))
    }
}

pub struct ChartPointSymbolText<'a>(&'a ChartPoint);
impl<'a> fmt::Display for ChartPointSymbolText<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.symbol_text())
    }
}

impl ChartPoint {
    #[inline]
    pub const fn canonical_key(self) -> &'static str {
        match self {
            ChartPoint::EclNut => "ecl_nut",
            ChartPoint::MeanNode => "mean_node",
            ChartPoint::TrueNode => "true_node",
            ChartPoint::MeanApog => "mean_apog",
            ChartPoint::OscuApog => "oscu_apog",
            ChartPoint::IntpApog => "intp_apog",
            ChartPoint::IntpPerg => "intp_perg",
        }
    }

    pub fn from_canonical_key(s: &str) -> Option<Self> {
        match s {
            "ecl_nut" => Some(ChartPoint::EclNut),
            "mean_node" => Some(ChartPoint::MeanNode),
            "true_node" => Some(ChartPoint::TrueNode),
            "mean_apog" => Some(ChartPoint::MeanApog),
            "oscu_apog" => Some(ChartPoint::OscuApog),
            "intp_apog" => Some(ChartPoint::IntpApog),
            "intp_perg" => Some(ChartPoint::IntpPerg),
            _ => None,
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            ChartPoint::EclNut => "Ecliptic Nutation",
            ChartPoint::MeanNode => "Mean Node",
            ChartPoint::TrueNode => "True Node",
            ChartPoint::MeanApog => "Mean Apogee",
            ChartPoint::OscuApog => "Osculating Apogee",
            ChartPoint::IntpApog => "Interpolated Apogee",
            ChartPoint::IntpPerg => "Interpolated Perigee",
        }
    }

    pub fn format_chart_point(self, fmt: ChartPointFormat) -> String {
        match fmt {
            ChartPointFormat::Key => self.canonical_key().to_owned(),
            ChartPointFormat::Name => self.name().to_owned(),
            ChartPointFormat::Symbol => self.symbol_text(),
            ChartPointFormat::Debug => format!("{:?}", self),
        }
    }

    pub fn symbol_text(&self) -> String {
        chart_point_symbol_text(self)
    }
}

impl std::str::FromStr for ChartPoint {
    type Err = ParseKeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_canonical_key(s).ok_or_else(|| ParseKeyError::new("ChartPoint", s))
    }
}

impl TryFrom<&str> for ChartPoint {
    type Error = ParseKeyError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_key_round_trips_with_from_str() {
        use strum::IntoEnumIterator;

        for point in ChartPoint::iter() {
            assert_eq!(
                ChartPoint::from_canonical_key(point.canonical_key()),
                Some(point)
            );
            assert_eq!(point.canonical_key().parse::<ChartPoint>(), Ok(point));
        }

        let err = "not_a_chart_point".parse::<ChartPoint>().unwrap_err();
        assert_eq!(err, ParseKeyError::new("ChartPoint", "not_a_chart_point"));
    }

    #[test]
    fn format_chart_point_supports_multiple_screen_representations() {
        assert_eq!(
            ChartPoint::MeanNode.format_chart_point(ChartPointFormat::Key),
            "mean_node"
        );
        assert_eq!(
            ChartPoint::MeanNode.format_chart_point(ChartPointFormat::Name),
            "Mean Node"
        );
        assert_eq!(
            ChartPoint::MeanNode.format_chart_point(ChartPointFormat::Symbol),
            "☊"
        );
        assert_eq!(
            ChartPoint::MeanNode.format_chart_point(ChartPointFormat::Debug),
            "MeanNode"
        );
    }
}
