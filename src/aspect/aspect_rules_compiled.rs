use super::DegreeAspectKind;
use crate::{EndpointKey, Occupant, OrbCombine};
use std::collections::{HashMap, HashSet};

/// A precompiled representation of `AspectRules` for fast matching.
///
/// This is an internal optimization. The public configuration type remains `AspectRules`.
#[derive(Debug, Clone, PartialEq)]
pub struct CompiledAspectRules {
    pub include_endpoints: Option<HashSet<EndpointKey>>,
    pub exclude_endpoints: HashSet<EndpointKey>,

    pub include_bodies: bool,
    pub include_chart_points: bool,
    pub include_angles: bool,
    pub include_lots: bool,

    pub orb_combine: OrbCombine,
    pub orb_overrides: OrbOverridesCompiled,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct OrbOverridesCompiled {
    per_kind: HashMap<(EndpointKey, DegreeAspectKind), f64>,
    all_kinds: HashMap<EndpointKey, f64>,
}

impl OrbOverridesCompiled {
    pub fn insert(&mut self, endpoint: EndpointKey, kind: Option<DegreeAspectKind>, orb_deg: f64) {
        match kind {
            Some(k) => {
                self.per_kind.insert((endpoint, k), orb_deg);
            }
            None => {
                self.all_kinds.insert(endpoint, orb_deg);
            }
        }
    }

    pub fn orb_for_endpoint(&self, endpoint: EndpointKey, kind: &DegreeAspectKind) -> Option<f64> {
        if let Some(v) = self.per_kind.get(&(endpoint, kind.clone())) {
            return Some(*v);
        }

        self.all_kinds.get(&endpoint).copied()
    }
}

impl CompiledAspectRules {
    /// Returns true if this endpoint passes the configured filters.
    pub fn endpoint_allowed(&self, occupant: &Occupant) -> bool {
        match occupant {
            Occupant::Empty => return false,
            Occupant::Body(_) if !self.include_bodies => return false,
            Occupant::ChartPoint(_) if !self.include_chart_points => return false,
            Occupant::Angle(_) if !self.include_angles => return false,
            Occupant::Lot(_) if !self.include_lots => return false,
            _ => {}
        }

        let Some(key) = EndpointKey::from_occupant(*occupant) else {
            return false;
        };

        if self.exclude_endpoints.contains(&key) {
            return false;
        }

        if let Some(include) = &self.include_endpoints {
            return include.contains(&key);
        }

        true
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
        let a_key = EndpointKey::from_occupant(*a);
        let b_key = EndpointKey::from_occupant(*b);

        let a_orb = a_key
            .and_then(|k| self.orb_overrides.orb_for_endpoint(k, kind))
            .unwrap_or(base_orb_deg);
        let b_orb = b_key
            .and_then(|k| self.orb_overrides.orb_for_endpoint(k, kind))
            .unwrap_or(base_orb_deg);

        self.orb_combine.combine(a_orb, b_orb)
    }
}
