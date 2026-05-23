use super::*;

/// A precomputed exact aspect target.
///
/// This type is useful for algorithms that work by generating the exact target
/// degree(s) for each aspect kind and then checking whether other points land
/// within orb of those targets.
#[derive(Debug, Clone)]
pub struct ExactDegreeAspect {
    pub degree_aspect_kind: DegreeAspectKind,
    pub sign_degree_aspect_match_kind: SignDegreeAspectMatchKind,
}

impl ExactDegreeAspect {
    pub fn new(
        degree_aspect_kind: DegreeAspectKind,
        sign_degree_aspect_match_kind: SignDegreeAspectMatchKind,
    ) -> Self {
        Self {
            degree_aspect_kind,
            sign_degree_aspect_match_kind,
        }
    }
}
