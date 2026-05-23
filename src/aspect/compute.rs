use crate::{
    aspect::AspectRules, AspectEdge, AspectEndpointId, DegreeAspectKind, EndpointKey, Occupant,
    PlacementMotion,
};

/// Compute a consolidated undirected aspect edge list for a single dataset (natal).
///
/// Notes:
/// - Only placements with `SignDegree` coordinates participate.
/// - Applying/separating is intentionally out of scope for this first pass.
pub fn compute_aspects_natal(
    placement_motions: &[PlacementMotion],
    rules: &AspectRules,
) -> Vec<AspectEdge> {
    // If the caller provided an empty `aspects` list, treat that as "use default aspects",
    // but keep any other rule fields (filters, overrides, combine policy).
    let mut effective_rules = rules.clone();
    if effective_rules.aspects.is_empty() {
        effective_rules.aspects = AspectRules::default().aspects;
    }

    // Compile once to avoid rebuilding lookup tables in hot loops.
    let compiled_rules = effective_rules.compile();

    let enabled = effective_rules
        .enabled_aspects()
        .map(|r| (r.kind.clone(), r.orb_deg))
        .collect::<Vec<(DegreeAspectKind, f64)>>();

    let mut edges = Vec::new();

    for (i, a) in placement_motions.iter().copied().enumerate() {
        let a_occ = a.placement.occupant;
        if !compiled_rules.endpoint_allowed(&a_occ) {
            continue;
        }

        let Some(a_sd) = a.sign_degree() else {
            continue;
        };

        for b in placement_motions.iter().copied().skip(i + 1) {
            let b_occ = b.placement.occupant;
            if !compiled_rules.endpoint_allowed(&b_occ) {
                continue;
            }

            let Some(b_sd) = b.sign_degree() else {
                continue;
            };

            // Smallest separation angle between points on the circle (0..180).
            let sep = a_sd
                .forward_distance(&b_sd)
                .min(a_sd.backward_distance(&b_sd));

            // Find the best (closest) enabled aspect kind for this pair.
            let mut best: Option<(DegreeAspectKind, f64, f64)> = None;
            for (kind, base_orb_deg) in &enabled {
                let exact = kind.aspect_kind_degree_f64();
                let diff = (sep - exact).abs();

                let pair_orb_deg =
                    compiled_rules.orb_deg_for_pair(kind, *base_orb_deg, &a_occ, &b_occ);

                if diff <= pair_orb_deg {
                    match &best {
                        None => best = Some((kind.clone(), diff, sep - exact)),
                        Some((_, best_diff, _)) if diff < *best_diff => {
                            best = Some((kind.clone(), diff, sep - exact))
                        }
                        _ => {}
                    }
                }
            }

            if let Some((kind, orb, exact_delta)) = best {
                let a_id = AspectEndpointId::from_occupant(a.placement.occupant);
                let b_id = AspectEndpointId::from_occupant(b.placement.occupant);

                edges.push(AspectEdge::new(a_id, b_id, kind, orb, Some(exact_delta)));
            }
        }
    }

    // Sort for stable output.
    edges.sort_by(|lhs, rhs| {
        (&lhs.a, &lhs.b, &lhs.kind.to_string()).cmp(&(&rhs.a, &rhs.b, &rhs.kind.to_string()))
    });

    edges
}

/// Compute a consolidated undirected aspect edge list between two unnamed datasets.
///
/// This is a compatibility wrapper that assigns synthetic dataset ids "a" and "b".
/// For transit charts you likely want [`compute_aspects_cross_datasets`] so the endpoint ids are
/// stable and dataset-scoped (e.g. "transit:sun" vs "natal:sun").
pub fn compute_aspects_cross(
    a_placement_motions: &[PlacementMotion],
    b_placement_motions: &[PlacementMotion],
    rules: &AspectRules,
) -> Vec<AspectEdge> {
    compute_aspects_cross_datasets("a", a_placement_motions, "b", b_placement_motions, rules)
}

/// Compute a consolidated undirected aspect edge list between two datasets (e.g. transit ↔ natal).
///
/// Notes:
/// - Only placements with `SignDegree` coordinates participate.
/// - The edge list is still undirected (`AspectEdge` stores endpoints as ids), but pairs are
///   formed strictly as (a in `a_placement_motions`) × (b in `b_placement_motions`).
/// - Endpoints are dataset-scoped via `"{dataset}:{occupant}"` so the same occupant in two
///   datasets can appear as distinct endpoints.
pub fn compute_aspects_cross_datasets(
    a_dataset_id: &str,
    a_placement_motions: &[PlacementMotion],
    b_dataset_id: &str,
    b_placement_motions: &[PlacementMotion],
    rules: &AspectRules,
) -> Vec<AspectEdge> {
    let mut effective_rules = rules.clone();
    if effective_rules.aspects.is_empty() {
        effective_rules.aspects = AspectRules::default().aspects;
    }

    let compiled_rules = effective_rules.compile();

    let enabled = effective_rules
        .enabled_aspects()
        .map(|r| (r.kind.clone(), r.orb_deg))
        .collect::<Vec<(DegreeAspectKind, f64)>>();

    let mut edges = Vec::new();

    for a in a_placement_motions.iter().copied() {
        let a_occ = a.placement.occupant;
        if !compiled_rules.endpoint_allowed(&a_occ) {
            continue;
        }

        let Some(a_sd) = a.sign_degree() else {
            continue;
        };

        for b in b_placement_motions.iter().copied() {
            let b_occ = b.placement.occupant;
            if !compiled_rules.endpoint_allowed(&b_occ) {
                continue;
            }

            let Some(b_sd) = b.sign_degree() else {
                continue;
            };

            let sep = a_sd
                .forward_distance(&b_sd)
                .min(a_sd.backward_distance(&b_sd));

            let mut best: Option<(DegreeAspectKind, f64, f64)> = None;
            for (kind, base_orb_deg) in &enabled {
                let exact = kind.aspect_kind_degree_f64();
                let diff = (sep - exact).abs();

                let pair_orb_deg =
                    compiled_rules.orb_deg_for_pair(kind, *base_orb_deg, &a_occ, &b_occ);

                if diff <= pair_orb_deg {
                    match &best {
                        None => best = Some((kind.clone(), diff, sep - exact)),
                        Some((_, best_diff, _)) if diff < *best_diff => {
                            best = Some((kind.clone(), diff, sep - exact))
                        }
                        _ => {}
                    }
                }
            }

            if let Some((kind, orb, exact_delta)) = best {
                let a_id = AspectEndpointId::from_dataset_occupant(a_dataset_id, a_occ);
                let b_id = AspectEndpointId::from_dataset_occupant(b_dataset_id, b_occ);

                edges.push(AspectEdge::new(a_id, b_id, kind, orb, Some(exact_delta)));
            }
        }
    }

    edges.sort_by(|lhs, rhs| {
        (&lhs.a, &lhs.b, &lhs.kind.to_string()).cmp(&(&rhs.a, &rhs.b, &rhs.kind.to_string()))
    });

    edges
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Body, Coordinate, Motion, Occupant, Placement, PlacementMotion, SignDegree};

    fn pm(body: Body, degree360: f64) -> PlacementMotion {
        let sd = SignDegree::new(degree360);
        let placement = Placement::new(Coordinate::SignDegree(sd), Occupant::Body(body));
        PlacementMotion::new(placement, Motion::Direct)
    }

    #[test]
    fn per_endpoint_orb_override_allows_sun_pluto_conjunction() {
        // Sun at 0°, Pluto at 3° => conjunction orb = 3°.
        let placements = vec![pm(Body::Sun, 0.0), pm(Body::Pluto, 3.0)];

        let rules = AspectRules {
            // Only consider conjunctions and set a tight base orb.
            aspects: vec![crate::aspect::AspectRule::enabled(
                DegreeAspectKind::Conjunction,
                2.5,
            )],
            // Widen Sun, tighten Pluto.
            orb_overrides: vec![
                crate::aspect::OrbOverride {
                    endpoint: EndpointKey::Body(Body::Sun),
                    kind: None,
                    orb_deg: 10.0,
                },
                crate::aspect::OrbOverride {
                    endpoint: EndpointKey::Body(Body::Pluto),
                    kind: None,
                    orb_deg: 2.0,
                },
            ],
            // "Max" means the Sun widens the pair orb to 10°.
            orb_combine: crate::aspect::OrbCombine::Max,
            ..Default::default()
        };

        let edges = compute_aspects_natal(&placements, &rules);
        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0].kind, DegreeAspectKind::Conjunction);
    }

    #[test]
    fn per_endpoint_orb_override_can_be_strict_with_min_policy() {
        // Sun at 0°, Pluto at 3° => conjunction orb = 3°.
        let placements = vec![pm(Body::Sun, 0.0), pm(Body::Pluto, 3.0)];

        let rules = AspectRules {
            aspects: vec![crate::aspect::AspectRule::enabled(
                DegreeAspectKind::Conjunction,
                8.0,
            )],
            orb_overrides: vec![
                crate::aspect::OrbOverride {
                    endpoint: EndpointKey::Body(Body::Sun),
                    kind: None,
                    orb_deg: 10.0,
                },
                crate::aspect::OrbOverride {
                    endpoint: EndpointKey::Body(Body::Pluto),
                    kind: None,
                    orb_deg: 2.0,
                },
            ],
            // "Min" means the Pluto override tightens the pair orb to 2°.
            orb_combine: crate::aspect::OrbCombine::Min,
            ..Default::default()
        };

        let edges = compute_aspects_natal(&placements, &rules);
        assert!(edges.is_empty());
    }

    #[test]
    fn include_endpoints_filter_works() {
        let placements = vec![pm(Body::Sun, 0.0), pm(Body::Moon, 180.0)];

        let rules = AspectRules {
            aspects: vec![crate::aspect::AspectRule::enabled(
                DegreeAspectKind::Opposition,
                8.0,
            )],
            include_endpoints: vec![EndpointKey::Body(Body::Sun)],
            ..Default::default()
        };

        // Moon is excluded by include_endpoints allow-list => no edges.
        let edges = compute_aspects_natal(&placements, &rules);
        assert!(edges.is_empty());
    }

    #[test]
    fn compute_aspects_cross_detects_cross_conjunction() {
        let a = vec![pm(Body::Sun, 0.0)];
        let b = vec![pm(Body::Pluto, 3.0)];

        let rules = AspectRules {
            aspects: vec![crate::aspect::AspectRule::enabled(
                DegreeAspectKind::Conjunction,
                8.0,
            )],
            ..Default::default()
        };

        let edges = compute_aspects_cross(&a, &b, &rules);
        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0].kind, DegreeAspectKind::Conjunction);
    }
}
