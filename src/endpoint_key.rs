use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use strum::IntoEnumIterator;

use crate::{Angle, Body, ChartPoint, Lot, Occupant};

static BODY_BY_KEY: Lazy<HashMap<&'static str, Body>> =
    Lazy::new(|| Body::iter().map(|b| (b.canonical_key(), b)).collect());

static ANGLE_BY_KEY: Lazy<HashMap<&'static str, Angle>> =
    Lazy::new(|| Angle::iter().map(|a| (a.canonical_key(), a)).collect());

static CHART_POINT_BY_KEY: Lazy<HashMap<&'static str, ChartPoint>> =
    Lazy::new(|| ChartPoint::iter().map(|p| (p.canonical_key(), p)).collect());

static LOT_BY_KEY: Lazy<HashMap<&'static str, Lot>> =
    Lazy::new(|| Lot::iter().map(|l| (l.canonical_key(), l)).collect());

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndpointKeyParseError {
    key: String,
}

impl EndpointKeyParseError {
    fn new(key: impl Into<String>) -> Self {
        Self { key: key.into() }
    }
}

impl fmt::Display for EndpointKeyParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown endpoint key: {}", self.key)
    }
}

impl std::error::Error for EndpointKeyParseError {}

/// A typed stable key for identifying an aspect endpoint.
///
/// Serialization format is the occupant's canonical key string (e.g. "sun", "ascendant",
/// "mean_node").
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub enum EndpointKey {
    Body(Body),
    ChartPoint(ChartPoint),
    Angle(Angle),
    Lot(Lot),
}

impl EndpointKey {
    #[inline]
    pub const fn canonical_key(self) -> &'static str {
        match self {
            EndpointKey::Body(b) => b.canonical_key(),
            EndpointKey::ChartPoint(p) => p.canonical_key(),
            EndpointKey::Angle(a) => a.canonical_key(),
            EndpointKey::Lot(l) => l.canonical_key(),
        }
    }

    pub fn from_occupant(occupant: Occupant) -> Option<Self> {
        match occupant {
            Occupant::Empty => None,
            Occupant::Body(b) => Some(Self::Body(b)),
            Occupant::ChartPoint(p) => Some(Self::ChartPoint(p)),
            Occupant::Angle(a) => Some(Self::Angle(a)),
            Occupant::Lot(l) => Some(Self::Lot(l)),
        }
    }

    pub fn parse(s: &str) -> Result<Self, EndpointKeyParseError> {
        if let Some(b) = BODY_BY_KEY.get(s).copied() {
            return Ok(Self::Body(b));
        }

        if let Some(a) = ANGLE_BY_KEY.get(s).copied() {
            return Ok(Self::Angle(a));
        }

        if let Some(p) = CHART_POINT_BY_KEY.get(s).copied() {
            return Ok(Self::ChartPoint(p));
        }

        if let Some(l) = LOT_BY_KEY.get(s).copied() {
            return Ok(Self::Lot(l));
        }

        Err(EndpointKeyParseError::new(s))
    }
}

impl TryFrom<String> for EndpointKey {
    type Error = EndpointKeyParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        EndpointKey::parse(&value)
    }
}

impl From<EndpointKey> for String {
    fn from(value: EndpointKey) -> Self {
        value.canonical_key().to_owned()
    }
}

impl From<Body> for EndpointKey {
    fn from(value: Body) -> Self {
        EndpointKey::Body(value)
    }
}

impl From<Angle> for EndpointKey {
    fn from(value: Angle) -> Self {
        EndpointKey::Angle(value)
    }
}

impl From<ChartPoint> for EndpointKey {
    fn from(value: ChartPoint) -> Self {
        EndpointKey::ChartPoint(value)
    }
}

impl From<Lot> for EndpointKey {
    fn from(value: Lot) -> Self {
        EndpointKey::Lot(value)
    }
}

impl fmt::Display for EndpointKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.canonical_key())
    }
}

impl EndpointKey {
    /// Compare two endpoint keys using a stable ordering.
    ///
    /// This is primarily for producing deterministic output (e.g. sorting aspect edges).
    #[inline]
    pub fn cmp_stable(a: &Self, b: &Self) -> Ordering {
        a.canonical_key().cmp(b.canonical_key())
    }
}

impl PartialOrd for EndpointKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EndpointKey {
    fn cmp(&self, other: &Self) -> Ordering {
        Self::cmp_stable(self, other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_known_endpoint_keys() {
        assert_eq!(
            EndpointKey::parse("sun").unwrap(),
            EndpointKey::Body(Body::Sun)
        );
        assert_eq!(
            EndpointKey::parse("ascendant").unwrap(),
            EndpointKey::Angle(Angle::Ascendant)
        );
        assert_eq!(
            EndpointKey::parse("mean_node").unwrap(),
            EndpointKey::ChartPoint(ChartPoint::MeanNode)
        );
        assert_eq!(
            EndpointKey::parse("fortune").unwrap(),
            EndpointKey::Lot(Lot::Fortune)
        );
    }

    #[test]
    fn canonical_key_round_trips() {
        let keys = [
            EndpointKey::Body(Body::Sun),
            EndpointKey::Body(Body::Pluto),
            EndpointKey::Angle(Angle::Ascendant),
            EndpointKey::ChartPoint(ChartPoint::MeanNode),
            EndpointKey::Lot(Lot::Fortune),
        ];

        for k in keys {
            let s = k.canonical_key();
            let parsed = EndpointKey::parse(s).unwrap();
            assert_eq!(parsed, k);
        }
    }

    #[test]
    fn serde_round_trips_as_canonical_string() {
        // JSON round-trip is a good serde proxy for any config format.
        let original = EndpointKey::Body(Body::Sun);

        let json = serde_json::to_string(&original).unwrap();
        assert_eq!(json, "\"sun\"");

        let parsed: EndpointKey = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn rejects_unknown_endpoint_keys() {
        let err = EndpointKey::parse("definitely_not_real").unwrap_err();
        assert_eq!(err.to_string(), "unknown endpoint key: definitely_not_real");

        // Also ensure serde rejects unknown keys.
        let err = serde_json::from_str::<EndpointKey>("\"definitely_not_real\"").unwrap_err();
        assert!(err.to_string().contains("unknown endpoint key"));
    }

    #[test]
    fn parses_from_toml_string() {
        let input = "key = 'sun'";

        #[derive(Debug, serde::Deserialize)]
        struct Wrapper {
            key: EndpointKey,
        }

        let parsed: Wrapper = toml::from_str(input).unwrap();
        assert_eq!(parsed.key, EndpointKey::Body(Body::Sun));
    }

    #[test]
    fn toml_rejects_unknown_keys() {
        let input = "key = 'definitely_not_real'";

        #[derive(Debug, serde::Deserialize)]
        struct Wrapper {
            key: EndpointKey,
        }

        let err = toml::from_str::<Wrapper>(input).unwrap_err();
        assert!(err.to_string().contains("unknown endpoint key"));
    }
}
