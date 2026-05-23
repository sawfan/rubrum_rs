use super::*;

use strum::IntoEnumIterator;

#[derive(Debug, Clone)]
pub struct PlacementExactAspectMatches {
    pub placement: Placement,
    pub exact: Vec<ExactDegreeAspect>,
}

impl PlacementExactAspectMatches {
    pub fn new(placement: Placement, exact: Vec<ExactDegreeAspect>) -> Self {
        Self { placement, exact }
    }

    /// Build exact aspect targets for multiple placements.
    ///
    /// Placements without a `SignDegree` coordinate are skipped.
    pub fn from_multiple(placements: Vec<Placement>) -> Vec<Self> {
        placements
            .into_iter()
            .filter_map(|placement| {
                let sign_degree = placement.sign_degree()?;

                let aspect_matches = DegreeAspectKind::iter()
                    .map(|aspect_kind| {
                        let aspect_degree =
                            find_exact_sign_degree_for_aspect_kind(&aspect_kind, &sign_degree);

                        ExactDegreeAspect::new(aspect_kind, aspect_degree)
                    })
                    .collect::<Vec<ExactDegreeAspect>>();

                Some(PlacementExactAspectMatches::new(placement, aspect_matches))
            })
            .collect::<Vec<PlacementExactAspectMatches>>()
    }
}
