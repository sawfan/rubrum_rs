use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;
use strum::IntoEnumIterator;

use super::DegreeAspectKind;
use crate::{CompiledAspectRules, EndpointKey, Occupant, OrbOverridesCompiled};

fn default_true() -> bool {
    true
}

fn default_orb_combine() -> OrbCombine {
    OrbCombine::Max
}

/// How to combine two endpoint-specific orb limits into a single pairwise limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrbCombine {
    /// Use the larger of the two endpoint orb limits.
    ///
    /// This is a common "luminary widens the orb" policy.
    Max,

    /// Use the smaller of the two endpoint orb limits.
    ///
    /// This is a stricter policy (e.g. "outer planets always stay tight").
    Min,

    /// Use the arithmetic mean of the two endpoint orb limits.
    Mean,
}

impl OrbCombine {
    #[inline]
    pub fn combine(self, a: f64, b: f64) -> f64 {
        match self {
            OrbCombine::Max => a.max(b),
            OrbCombine::Min => a.min(b),
            OrbCombine::Mean => (a + b) / 2.0,
        }
    }
}

/// Override a base aspect orb for a specific endpoint.
///
/// Endpoint keys are the stable `EndpointKey` canonical strings (e.g. "sun", "pluto",
/// "ascendant").
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrbOverride {
    pub endpoint: EndpointKey,

    /// Optional aspect kind.
    ///
    /// If omitted, the override applies to all aspect kinds.
    #[serde(default)]
    pub kind: Option<DegreeAspectKind>,

    /// Override orb limit (degrees) for this endpoint.
    pub orb_deg: f64,
}

/// Configuration for which aspects to compute and the orb policy to apply.
///
/// This is intended to be user-facing configuration (e.g. loaded from a TOML file) and should be
/// kept separate from rendering concerns.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AspectRules {
    /// Individual aspect rules.
    ///
    /// If empty, consumers may choose a default set (see `Default`).
    #[serde(default)]
    pub aspects: Vec<AspectRule>,

    /// How endpoint-specific orbs are combined for a pair.
    #[serde(default = "default_orb_combine")]
    pub orb_combine: OrbCombine,

    /// Endpoint-specific orb overrides (e.g. widen Sun/Moon, tighten Pluto).
    #[serde(default)]
    pub orb_overrides: Vec<OrbOverride>,

    /// Optional filter: if non-empty, only endpoints in this list are considered.
    #[serde(default)]
    pub include_endpoints: Vec<EndpointKey>,

    /// Optional filter: endpoints in this list are always excluded.
    #[serde(default)]
    pub exclude_endpoints: Vec<EndpointKey>,

    /// Filter: include body-to-body aspects.
    #[serde(default = "default_true")]
    pub include_bodies: bool,

    /// Filter: include aspects involving chart points (e.g. nodes, Lilith).
    #[serde(default)]
    pub include_chart_points: bool,

    /// Filter: include aspects involving angles (ASC/MC/etc).
    #[serde(default)]
    pub include_angles: bool,

    /// Filter: include aspects involving lots (e.g. Part of Fortune).
    #[serde(default)]
    pub include_lots: bool,
}

impl Default for AspectRules {
    fn default() -> Self {
        // Conservative defaults: major Ptolemaic aspects only.
        //
        // NOTE: Orbs are intentionally modest here. Consumers can override by providing a rules
        // file. When in doubt, it is better to compute broadly and filter at display-time.
        Self {
            aspects: vec![
                AspectRule::enabled(DegreeAspectKind::Conjunction, 8.0),
                AspectRule::enabled(DegreeAspectKind::Sextile, 4.0),
                AspectRule::enabled(DegreeAspectKind::Square, 6.0),
                AspectRule::enabled(DegreeAspectKind::Trine, 6.0),
                AspectRule::enabled(DegreeAspectKind::Opposition, 8.0),
            ],
            orb_combine: OrbCombine::Max,
            orb_overrides: Vec::new(),
            include_endpoints: Vec::new(),
            exclude_endpoints: Vec::new(),
            include_bodies: true,
            include_chart_points: false,
            include_angles: false,
            include_lots: false,
        }
    }
}

impl AspectRules {
    /// Major/Ptolemaic aspect preset using the crate's conservative default orbs.
    pub fn major() -> Self {
        Self::default()
    }

    /// Alias for [`AspectRules::major`].
    pub fn ptolemaic() -> Self {
        Self::major()
    }

    /// Minor aspect preset with commonly used minor aspects and modest default orbs.
    pub fn minor() -> Self {
        Self {
            aspects: vec![
                AspectRule::enabled(DegreeAspectKind::Semisextile, 2.0),
                AspectRule::enabled(DegreeAspectKind::SemiSquare, 2.0),
                AspectRule::enabled(DegreeAspectKind::Quintile, 2.0),
                AspectRule::enabled(DegreeAspectKind::Sesquiquadrate, 2.0),
                AspectRule::enabled(DegreeAspectKind::Quincunx, 3.0),
                AspectRule::enabled(DegreeAspectKind::Biquintile, 2.0),
            ],
            ..Default::default()
        }
    }

    /// Major plus minor aspects.
    pub fn major_and_minor() -> Self {
        let mut rules = Self::major();
        rules.aspects.extend(Self::minor().aspects);
        rules
    }

    /// Enables every supported `DegreeAspectKind` using a caller-provided default orb.
    pub fn all_supported(default_orb_deg: f64) -> Self {
        Self {
            aspects: DegreeAspectKind::iter()
                .map(|kind| AspectRule::enabled(kind, default_orb_deg))
                .collect(),
            ..Default::default()
        }
    }

    /// Validate rule configuration before using it in computation.
    pub fn validate(&self) -> Result<(), AspectRulesValidationError> {
        for rule in &self.aspects {
            validate_orb(rule.orb_deg, "aspect rule")?;
        }

        for override_rule in &self.orb_overrides {
            validate_orb(override_rule.orb_deg, "orb override")?;
        }

        let mut seen = HashSet::new();
        for rule in &self.aspects {
            if !seen.insert(rule.kind.clone()) {
                return Err(AspectRulesValidationError::DuplicateAspectKind(
                    rule.kind.clone(),
                ));
            }
        }

        Ok(())
    }

    pub fn enabled_aspects(&self) -> impl Iterator<Item = &AspectRule> {
        self.aspects.iter().filter(|a| a.enabled)
    }

    pub fn is_empty(&self) -> bool {
        self.aspects.is_empty()
    }

    pub fn compile(&self) -> CompiledAspectRules {
        let mut effective_rules = self.clone();
        if effective_rules.aspects.is_empty() {
            effective_rules.aspects = AspectRules::default().aspects;
        }

        let include_endpoints = if effective_rules.include_endpoints.is_empty() {
            None
        } else {
            Some(effective_rules.include_endpoints.iter().copied().collect())
        };

        let exclude_endpoints = effective_rules.exclude_endpoints.iter().copied().collect();

        let mut orb_overrides = OrbOverridesCompiled::default();
        for o in &effective_rules.orb_overrides {
            orb_overrides.insert(o.endpoint, o.kind.clone(), o.orb_deg);
        }

        CompiledAspectRules {
            include_endpoints,
            exclude_endpoints,
            include_bodies: effective_rules.include_bodies,
            include_chart_points: effective_rules.include_chart_points,
            include_angles: effective_rules.include_angles,
            include_lots: effective_rules.include_lots,
            orb_combine: effective_rules.orb_combine,
            orb_overrides,
        }
    }

    /// Returns true if this endpoint passes the configured filters.
    pub fn endpoint_allowed(&self, occupant: &Occupant) -> bool {
        let compiled = self.compile();
        compiled.endpoint_allowed(occupant)
    }

    /// Compute a pairwise orb limit for a specific aspect kind, taking endpoint overrides into
    /// account.
    pub fn orb_deg_for_pair(
        &self,
        kind: &DegreeAspectKind,
        base_orb_deg: f64,
        a: &Occupant,
        b: &Occupant,
    ) -> f64 {
        let compiled = self.compile();
        compiled.orb_deg_for_pair(kind, base_orb_deg, a, b)
    }
}

fn validate_orb(value: f64, context: &'static str) -> Result<(), AspectRulesValidationError> {
    if !value.is_finite() {
        return Err(AspectRulesValidationError::InvalidOrb {
            context,
            orb_deg: value,
            reason: "must be finite",
        });
    }

    if value < 0.0 {
        return Err(AspectRulesValidationError::InvalidOrb {
            context,
            orb_deg: value,
            reason: "must be non-negative",
        });
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
pub enum AspectRulesValidationError {
    InvalidOrb {
        context: &'static str,
        orb_deg: f64,
        reason: &'static str,
    },
    DuplicateAspectKind(DegreeAspectKind),
}

impl fmt::Display for AspectRulesValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AspectRulesValidationError::InvalidOrb {
                context,
                orb_deg,
                reason,
            } => write!(f, "invalid {context} orb {orb_deg}: {reason}"),
            AspectRulesValidationError::DuplicateAspectKind(kind) => {
                write!(f, "duplicate aspect kind: {}", kind.canonical_key())
            }
        }
    }
}

impl std::error::Error for AspectRulesValidationError {}

/// One enabled/disabled aspect kind and its allowed orb.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AspectRule {
    pub kind: DegreeAspectKind,

    /// If true, this aspect kind is considered during matching.
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Allowed distance (in degrees) from exactness.
    pub orb_deg: f64,
}

impl AspectRule {
    pub fn enabled(kind: DegreeAspectKind, orb_deg: f64) -> Self {
        Self {
            kind,
            enabled: true,
            orb_deg,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Body;

    #[test]
    fn orb_override_precedence_and_combine() {
        let mut rules = AspectRules {
            orb_overrides: vec![
                OrbOverride {
                    endpoint: EndpointKey::Body(Body::Sun),
                    kind: None,
                    orb_deg: 10.0,
                },
                OrbOverride {
                    endpoint: EndpointKey::Body(Body::Sun),
                    kind: Some(DegreeAspectKind::Conjunction),
                    orb_deg: 12.0,
                },
                OrbOverride {
                    endpoint: EndpointKey::Body(Body::Pluto),
                    kind: None,
                    orb_deg: 2.0,
                },
            ],
            orb_combine: OrbCombine::Max,
            ..Default::default()
        };
        let max_orb = rules.orb_deg_for_pair(
            &DegreeAspectKind::Conjunction,
            8.0,
            &Occupant::Body(Body::Sun),
            &Occupant::Body(Body::Pluto),
        );
        assert_eq!(max_orb, 12.0);

        rules.orb_combine = OrbCombine::Min;
        let min_orb = rules.orb_deg_for_pair(
            &DegreeAspectKind::Conjunction,
            8.0,
            &Occupant::Body(Body::Sun),
            &Occupant::Body(Body::Pluto),
        );
        assert_eq!(min_orb, 2.0);
    }

    #[test]
    fn endpoint_filters_work() {
        let mut rules = AspectRules {
            include_endpoints: vec![EndpointKey::Body(Body::Sun)],
            ..Default::default()
        };

        assert!(rules.endpoint_allowed(&Occupant::Body(Body::Sun)));
        assert!(!rules.endpoint_allowed(&Occupant::Body(Body::Moon)));

        rules.exclude_endpoints = vec![EndpointKey::Body(Body::Sun)];
        assert!(!rules.endpoint_allowed(&Occupant::Body(Body::Sun)));
    }

    #[test]
    fn presets_are_valid() {
        AspectRules::major().validate().unwrap();
        AspectRules::minor().validate().unwrap();
        AspectRules::major_and_minor().validate().unwrap();
        AspectRules::all_supported(1.0).validate().unwrap();
    }

    #[test]
    fn validate_rejects_invalid_orbs_and_duplicates() {
        let invalid = AspectRules {
            aspects: vec![AspectRule::enabled(DegreeAspectKind::Trine, -1.0)],
            ..Default::default()
        };
        assert!(matches!(
            invalid.validate(),
            Err(AspectRulesValidationError::InvalidOrb { .. })
        ));

        let duplicate = AspectRules {
            aspects: vec![
                AspectRule::enabled(DegreeAspectKind::Trine, 6.0),
                AspectRule::enabled(DegreeAspectKind::Trine, 5.0),
            ],
            ..Default::default()
        };
        assert_eq!(
            duplicate.validate(),
            Err(AspectRulesValidationError::DuplicateAspectKind(
                DegreeAspectKind::Trine
            ))
        );
    }
}
