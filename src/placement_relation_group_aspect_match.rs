use super::*;

#[derive(Debug, Clone)]
pub struct PlacementAspectMatch {
    pub kind: DegreeAspectKind,
    pub origin: Placement,
    pub destination: Placement,
    /// Smallest angular distance (in degrees) from exactness.
    pub diff: f64,
}

#[derive(Debug, Clone)]
pub struct PlacementRelationGroupAspectMatch {
    pub relation_group: PlacementRelationGroup,
    pub exacts: Vec<PlacementExactAspectMatches>,
}

impl PlacementRelationGroupAspectMatch {
    pub fn new(
        relation_group: PlacementRelationGroup,
        exacts: Vec<PlacementExactAspectMatches>,
    ) -> Self {
        Self {
            relation_group,
            exacts,
        }
    }

    pub fn from_placements(placements: Vec<Placement>) -> Self {
        let relation_group = PlacementRelationGroup::new(placements.clone());
        let exacts = PlacementExactAspectMatches::from_multiple(placements);
        Self::new(relation_group, exacts)
    }

    pub fn aspect_matches(&self) -> Vec<PlacementAspectMatch> {
        self.aspect_matches_with_rules(&AspectRules::default())
    }

    /// Compute aspect matches using the supplied rules (including per-endpoint orb overrides).
    pub fn aspect_matches_with_rules(&self, rules: &AspectRules) -> Vec<PlacementAspectMatch> {
        // Compile once to avoid rebuilding lookup tables in hot loops.
        let compiled_rules = rules.compile();

        // If rules provided an explicit aspect list, treat it as an allow-list.
        let aspect_allow_list = if rules.aspects.is_empty() {
            None
        } else {
            Some(
                rules
                    .aspects
                    .iter()
                    .filter(|a| a.enabled)
                    .map(|a| a.kind.clone())
                    .collect::<std::collections::HashSet<_>>(),
            )
        };

        let mut matches = vec![];

        for relation in &self.relation_group.relations {
            // Apply endpoint-type and include/exclude filters.
            if !compiled_rules.endpoint_allowed(&relation.origin.occupant)
                || !compiled_rules.endpoint_allowed(&relation.destination.occupant)
            {
                continue;
            }

            // Find the exact targets for this specific origin placement.
            let Some(exacts_for_origin) =
                self.exacts.iter().find(|e| e.placement == relation.origin)
            else {
                continue;
            };

            for exact in &exacts_for_origin.exact {
                use SignDegreeAspectMatchKind::*;

                let kind = exact.degree_aspect_kind.clone();

                // If rules provided an explicit aspect list, treat it as an allow-list.
                if let Some(allow_list) = &aspect_allow_list {
                    if !allow_list.contains(&kind) {
                        continue;
                    }
                }

                let Some(dest_sd) = relation.destination.sign_degree() else {
                    continue;
                };

                // Compare destination degree to each exact target.
                match &exact.sign_degree_aspect_match_kind {
                    Single(target) => {
                        let diff = target
                            .forward_distance(&dest_sd)
                            .min(target.backward_distance(&dest_sd));

                        let base_orb = orb_deg_legacy(&kind);
                        let pair_orb_deg = compiled_rules.orb_deg_for_pair(
                            &kind,
                            base_orb,
                            &relation.origin.occupant,
                            &relation.destination.occupant,
                        );

                        if diff <= pair_orb_deg {
                            matches.push(PlacementAspectMatch {
                                kind,
                                origin: relation.origin,
                                destination: relation.destination,
                                diff,
                            });
                        }
                    }
                    Double(targets) => {
                        let base_orb = orb_deg_legacy(&kind);
                        let pair_orb_deg = rules.orb_deg_for_pair(
                            &kind,
                            base_orb,
                            &relation.origin.occupant,
                            &relation.destination.occupant,
                        );

                        for target in targets {
                            let diff = target
                                .forward_distance(&dest_sd)
                                .min(target.backward_distance(&dest_sd));

                            if diff <= pair_orb_deg {
                                matches.push(PlacementAspectMatch {
                                    kind: kind.clone(),
                                    origin: relation.origin,
                                    destination: relation.destination,
                                    diff,
                                });
                            }
                        }
                    }
                }
            }
        }

        matches
    }
}

/// Legacy orb baselines for the older placement-aspect engine.
///
/// This uses the crate's build-time constants from `config/orb_degrees.toml`.
fn orb_deg_legacy(kind: &DegreeAspectKind) -> f64 {
    use crate::constants::aspect_constants::*;
    use DegreeAspectKind::*;

    match kind {
        Conjunction => CONJUNCTION_ORB_DEGREES_WITH_SUN_OR_MOON,
        Opposition => OPPOSITION_ORB_DEGREES,
        Sextile => SEXTILE_ORB_DEGREES,
        Square => SQUARE_ORB_DEGREES,
        Trine => TRINE_ORB_DEGREES,
        Semisextile => SEMISEXTILE_ORB_DEGREES,
        Quincunx => QUINCUNX_ORB_DEGREES,
        Septile => SEPTILE_ORB_DEGREES,
        SemiSquare => OCTILE_ORB_DEGREES,
        Sesquiquadrate => SESQUIQUADRATE_ORB_DEGREES,
        Novile => NOVILE_ORB_DEGREES,
        Undecile => UNDECILE_ORB_DEGREES,
        _ => DEFAULT_ORB_DEGREES,
    }
}

#[derive(Debug, Clone)]
pub struct PlacementMotionAspectMatch {
    pub kind: DegreeAspectKind,
    pub origin: PlacementMotion,
    pub destination: PlacementMotion,
    pub diff: f64,
}

/// Primary consumer-friendly API: compute aspects between `PlacementMotion` values.
///
/// Placements without a `SignDegree` coordinate are skipped.
pub fn aspect_matches_between_placement_motions(
    placement_motions: Vec<PlacementMotion>,
) -> Vec<PlacementMotionAspectMatch> {
    let placements = placement_motions
        .iter()
        .map(|pm| pm.placement)
        .collect::<Vec<_>>();

    let engine = PlacementRelationGroupAspectMatch::from_placements(placements);
    let placement_matches = engine.aspect_matches();

    // Reattach motion context to each endpoint.
    placement_matches
        .into_iter()
        .filter_map(|m| {
            let origin_pm = placement_motions
                .iter()
                .find(|pm| pm.placement == m.origin)
                .copied()?;
            let destination_pm = placement_motions
                .iter()
                .find(|pm| pm.placement == m.destination)
                .copied()?;

            Some(PlacementMotionAspectMatch {
                kind: m.kind,
                origin: origin_pm,
                destination: destination_pm,
                diff: m.diff,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thema_mundi_placement_motion_aspects_smoke() {
        let placement_motions = THEMA_MUNDI_BODY_SIGN_DEGREES
            .iter()
            .map(|bsd| {
                let placement = Placement::new(
                    Coordinate::SignDegree(bsd.sign_degree),
                    Occupant::Body(bsd.body),
                );
                PlacementMotion::new(placement, Motion::Direct)
            })
            .collect::<Vec<_>>();

        let matches = aspect_matches_between_placement_motions(placement_motions);
        assert!(!matches.is_empty());
    }
}
