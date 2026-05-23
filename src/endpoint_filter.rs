use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt;
use strum::IntoEnumIterator;

use crate::{BodyGroup, EndpointKey, Occupant};

static BODY_GROUP_BY_KEY: Lazy<HashMap<&'static str, BodyGroup>> =
    Lazy::new(|| BodyGroup::iter().map(|g| (g.canonical_key(), g)).collect());

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BodyGroupKeyParseError {
    key: String,
}

impl BodyGroupKeyParseError {
    fn new(key: impl Into<String>) -> Self {
        Self { key: key.into() }
    }
}

impl fmt::Display for BodyGroupKeyParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown body group key: {}", self.key)
    }
}

impl std::error::Error for BodyGroupKeyParseError {}

/// Stable, storage-friendly key for `BodyGroup`.
///
/// Serialization format is the group's canonical key string (e.g. "big3", "modern").
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct BodyGroupKey(pub BodyGroup);

impl BodyGroupKey {
    #[inline]
    pub const fn canonical_key(self) -> &'static str {
        self.0.canonical_key()
    }

    pub fn parse(s: &str) -> Result<Self, BodyGroupKeyParseError> {
        BODY_GROUP_BY_KEY
            .get(s)
            .copied()
            .map(Self)
            .ok_or_else(|| BodyGroupKeyParseError::new(s))
    }

    #[inline]
    pub const fn into_inner(self) -> BodyGroup {
        self.0
    }
}

impl TryFrom<String> for BodyGroupKey {
    type Error = BodyGroupKeyParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        BodyGroupKey::parse(&value)
    }
}

impl From<BodyGroupKey> for String {
    fn from(value: BodyGroupKey) -> Self {
        value.canonical_key().to_owned()
    }
}

impl From<BodyGroup> for BodyGroupKey {
    fn from(value: BodyGroup) -> Self {
        Self(value)
    }
}

impl From<BodyGroupKey> for BodyGroup {
    fn from(value: BodyGroupKey) -> Self {
        value.0
    }
}

/// User-facing configuration for display/selection of endpoints.
///
/// This filter is intended to be fast when compiled. Typical usage:
///
/// ```
/// use rubrum::*;
///
/// let filter = EndpointFilter {
///     // Only show Big3 bodies.
///     include_body_groups: vec![BodyGroupKey(BodyGroup::Big3)],
///     ..Default::default()
/// };
///
/// let compiled = filter.compile();
/// assert!(compiled.endpoint_allowed(Occupant::Body(Body::Sun)));
/// assert!(!compiled.endpoint_allowed(Occupant::Body(Body::Mercury)));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EndpointFilter {
    /// When no allow-list is provided, allow bodies to pass.
    #[serde(default = "default_true")]
    pub allow_bodies: bool,

    /// When no allow-list is provided, allow angles to pass.
    #[serde(default = "default_true")]
    pub allow_angles: bool,

    /// When no allow-list is provided, allow chart points to pass.
    #[serde(default = "default_true")]
    pub allow_chart_points: bool,

    /// When no allow-list is provided, allow lots to pass.
    #[serde(default = "default_true")]
    pub allow_lots: bool,

    /// Allow-list endpoints. If non-empty (or if `include_body_groups` is non-empty),
    /// only endpoints in the allow-list are allowed (before exclusions are applied).
    #[serde(default)]
    pub include_endpoints: Vec<EndpointKey>,

    /// Allow-list body groups. Expands to all bodies in the group.
    #[serde(default)]
    pub include_body_groups: Vec<BodyGroupKey>,

    /// Deny-list endpoints. Deny-list always wins.
    #[serde(default)]
    pub exclude_endpoints: Vec<EndpointKey>,

    /// Deny-list body groups. Expands to all bodies in the group. Deny-list always wins.
    #[serde(default)]
    pub exclude_body_groups: Vec<BodyGroupKey>,
}

const fn default_true() -> bool {
    true
}

impl Default for EndpointFilter {
    fn default() -> Self {
        Self {
            allow_bodies: true,
            allow_angles: true,
            allow_chart_points: true,
            allow_lots: true,
            include_endpoints: vec![],
            include_body_groups: vec![],
            exclude_endpoints: vec![],
            exclude_body_groups: vec![],
        }
    }
}

impl EndpointFilter {
    /// Compiles the filter into a representation optimized for repeated checks.
    pub fn compile(&self) -> CompiledEndpointFilter {
        let mut excluded: HashSet<EndpointKey> = self.exclude_endpoints.iter().copied().collect();
        for group in &self.exclude_body_groups {
            for body in group.into_inner().bodies() {
                excluded.insert(EndpointKey::Body(*body));
            }
        }

        let allowlist_mode =
            !self.include_endpoints.is_empty() || !self.include_body_groups.is_empty();

        let allowlist = if allowlist_mode {
            let mut allowed: HashSet<EndpointKey> =
                self.include_endpoints.iter().copied().collect();
            for group in &self.include_body_groups {
                for body in group.into_inner().bodies() {
                    allowed.insert(EndpointKey::Body(*body));
                }
            }
            Some(allowed)
        } else {
            None
        };

        CompiledEndpointFilter {
            allow_bodies: self.allow_bodies,
            allow_angles: self.allow_angles,
            allow_chart_points: self.allow_chart_points,
            allow_lots: self.allow_lots,
            allowlist,
            excluded,
        }
    }
}

/// Compiled endpoint filter with fast membership checks.
#[derive(Debug, Clone)]
pub struct CompiledEndpointFilter {
    allow_bodies: bool,
    allow_angles: bool,
    allow_chart_points: bool,
    allow_lots: bool,

    /// When present, acts as the allow-list.
    allowlist: Option<HashSet<EndpointKey>>,

    /// Deny-list always wins.
    excluded: HashSet<EndpointKey>,
}

impl CompiledEndpointFilter {
    /// Returns true if this endpoint key is allowed by the compiled filter.
    #[inline]
    pub fn endpoint_key_allowed(&self, endpoint: EndpointKey) -> bool {
        if self.excluded.contains(&endpoint) {
            return false;
        }

        if let Some(allowlist) = &self.allowlist {
            return allowlist.contains(&endpoint);
        }

        match endpoint {
            EndpointKey::Body(_) => self.allow_bodies,
            EndpointKey::Angle(_) => self.allow_angles,
            EndpointKey::ChartPoint(_) => self.allow_chart_points,
            EndpointKey::Lot(_) => self.allow_lots,
        }
    }

    /// Returns true if this occupant is allowed by the compiled filter.
    #[inline]
    pub fn endpoint_allowed(&self, occupant: Occupant) -> bool {
        EndpointKey::from_occupant(occupant).is_some_and(|key| self.endpoint_key_allowed(key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Angle, Body, ChartPoint, Lot};

    #[test]
    fn default_allows_all_endpoint_kinds() {
        let compiled = EndpointFilter::default().compile();

        assert!(compiled.endpoint_allowed(Occupant::Body(Body::Sun)));
        assert!(compiled.endpoint_allowed(Occupant::Angle(Angle::Ascendant)));
        assert!(compiled.endpoint_allowed(Occupant::ChartPoint(ChartPoint::MeanNode)));
        assert!(compiled.endpoint_allowed(Occupant::Lot(Lot::Fortune)));
        assert!(!compiled.endpoint_allowed(Occupant::Empty));
    }

    #[test]
    fn allowlist_only_allows_included_endpoints_and_groups() {
        let filter = EndpointFilter {
            include_endpoints: vec![EndpointKey::Body(Body::Mercury)],
            include_body_groups: vec![BodyGroupKey(BodyGroup::Big3)],
            ..Default::default()
        };

        let compiled = filter.compile();

        assert!(compiled.endpoint_allowed(Occupant::Body(Body::Sun)));
        assert!(compiled.endpoint_allowed(Occupant::Body(Body::Moon)));
        assert!(compiled.endpoint_allowed(Occupant::Body(Body::Mercury)));

        // Not in include list.
        assert!(!compiled.endpoint_allowed(Occupant::Body(Body::Venus)));
        // Not in include list.
        assert!(!compiled.endpoint_allowed(Occupant::Angle(Angle::Ascendant)));
    }

    #[test]
    fn exclude_wins_over_include() {
        let filter = EndpointFilter {
            include_body_groups: vec![BodyGroupKey(BodyGroup::Big3)],
            exclude_endpoints: vec![EndpointKey::Body(Body::Sun)],
            ..Default::default()
        };

        let compiled = filter.compile();

        assert!(!compiled.endpoint_allowed(Occupant::Body(Body::Sun)));
        assert!(compiled.endpoint_allowed(Occupant::Body(Body::Moon)));
    }

    #[test]
    fn kind_toggles_apply_when_no_allowlist_is_present() {
        let filter = EndpointFilter {
            allow_bodies: false,
            allow_angles: true,
            allow_chart_points: false,
            allow_lots: true,
            ..Default::default()
        };

        let compiled = filter.compile();

        assert!(!compiled.endpoint_allowed(Occupant::Body(Body::Sun)));
        assert!(compiled.endpoint_allowed(Occupant::Angle(Angle::Ascendant)));
        assert!(!compiled.endpoint_allowed(Occupant::ChartPoint(ChartPoint::MeanNode)));
        assert!(compiled.endpoint_allowed(Occupant::Lot(Lot::Fortune)));
    }

    #[test]
    fn exclude_body_group_expands_correctly() {
        let filter = EndpointFilter {
            exclude_body_groups: vec![BodyGroupKey(BodyGroup::Modern)],
            ..Default::default()
        };

        let compiled = filter.compile();

        assert!(!compiled.endpoint_allowed(Occupant::Body(Body::Uranus)));
        assert!(!compiled.endpoint_allowed(Occupant::Body(Body::Neptune)));
        assert!(!compiled.endpoint_allowed(Occupant::Body(Body::Pluto)));
        // Sanity check: other bodies unaffected.
        assert!(compiled.endpoint_allowed(Occupant::Body(Body::Saturn)));
    }

    #[test]
    fn serde_round_trips_in_json_with_string_keys() {
        let filter = EndpointFilter {
            include_endpoints: vec![EndpointKey::Body(Body::Sun)],
            include_body_groups: vec![BodyGroupKey(BodyGroup::Modern)],
            exclude_endpoints: vec![EndpointKey::Angle(Angle::Ascendant)],
            ..Default::default()
        };

        let json = serde_json::to_string(&filter).unwrap();
        assert!(json.contains("\"sun\""));
        assert!(json.contains("\"modern\""));
        assert!(json.contains("\"ascendant\""));

        let round_tripped: EndpointFilter = serde_json::from_str(&json).unwrap();
        assert_eq!(round_tripped, filter);
    }

    #[test]
    fn parses_from_toml() {
        let input = r#"
allow_bodies = true
allow_angles = false
include_body_groups = ["big3"]
exclude_endpoints = ["moon"]
"#;

        let filter: EndpointFilter = toml::from_str(input).unwrap();
        let compiled = filter.compile();

        // Include list triggers allow-list mode => only Big3 allowed initially.
        // Exclude list removes Moon.
        assert!(compiled.endpoint_allowed(Occupant::Body(Body::Sun)));
        assert!(!compiled.endpoint_allowed(Occupant::Body(Body::Moon)));

        // Mercury is not in Big3.
        assert!(!compiled.endpoint_allowed(Occupant::Body(Body::Mercury)));
        // Angles are not included in allow-list.
        assert!(!compiled.endpoint_allowed(Occupant::Angle(Angle::Ascendant)));
    }

    #[test]
    fn toml_rejects_unknown_body_group_keys() {
        let input = r#"
include_body_groups = ["definitely_not_real"]
"#;

        let err = toml::from_str::<EndpointFilter>(input).unwrap_err();
        assert!(err.to_string().contains("unknown body group key"));
    }
}
