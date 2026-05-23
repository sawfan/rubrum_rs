// Demonstrates computing natal aspects with endpoint filtering.
//
// Run with:
//   cargo run --example natal_aspects_with_filters
//
// This example shows:
// - `EndpointFilter` allow/deny list behavior
// - `AspectRules` configuration + orb overrides
// - computing aspects via `aspect::compute_aspects_natal`

use rubrum::{
    aspect::{compute_aspects_natal, OrbCombine},
    AspectRule, AspectRules, Body, BodyGroup, BodyGroupKey, Coordinate, DegreeAspectKind,
    EndpointFilter, EndpointKey, Motion, Occupant, Placement, PlacementMotion, SignDegree,
};

fn pm(body: Body, degree360: f64) -> PlacementMotion {
    let sd = SignDegree::new(degree360);
    let placement = Placement::new(Coordinate::SignDegree(sd), Occupant::Body(body));
    PlacementMotion::new(placement, Motion::Direct)
}

fn main() {
    // A toy set of placements: Sun and Mars near conjunction, Moon opposite.
    let placements = vec![
        pm(Body::Sun, 0.0),
        pm(Body::Mars, 3.0),
        pm(Body::Moon, 180.0),
    ];

    // 1) Define which endpoints are allowed.
    // Only allow Big3 + Mars, but explicitly exclude Moon.
    //
    // NOTE: `AspectRules` has its own include/exclude lists (EndpointKey-level) but does not
    // understand body groups. We use `EndpointFilter` here just to expand Big3 -> bodies.
    let endpoint_filter = EndpointFilter {
        include_body_groups: vec![BodyGroupKey(BodyGroup::Big3)],
        include_endpoints: vec![EndpointKey::Body(Body::Mars)],
        exclude_endpoints: vec![EndpointKey::Body(Body::Moon)],
        ..Default::default()
    };
    let compiled = endpoint_filter.compile();

    let mut include_endpoints: Vec<EndpointKey> = vec![];
    for pm in &placements {
        if compiled.endpoint_allowed(pm.occupant()) {
            if let Some(key) = EndpointKey::from_occupant(pm.occupant()) {
                include_endpoints.push(key);
            }
        }
    }

    // 2) Define aspect rules.
    // Conjunction base orb is 10°, but Mars conjunction is tighter (6°).
    let rules = AspectRules {
        include_endpoints,
        exclude_endpoints: vec![EndpointKey::Body(Body::Moon)],
        aspects: vec![
            AspectRule::enabled(DegreeAspectKind::Conjunction, 10.0),
            AspectRule::enabled(DegreeAspectKind::Opposition, 8.0),
        ],
        orb_combine: OrbCombine::Min,
        orb_overrides: vec![rubrum::aspect::OrbOverride {
            endpoint: EndpointKey::Body(Body::Mars),
            kind: Some(DegreeAspectKind::Conjunction),
            orb_deg: 6.0,
        }],
        ..Default::default()
    };

    let edges = compute_aspects_natal(&placements, &rules);

    println!("computed aspects: {}", edges.len());
    for e in edges {
        println!("{} {} {} orb={:.2}", e.a.0, e.kind, e.b.0, e.orb);
    }
}
