use rubrum::{
    aspect::compute_aspects_natal, AspectRule, AspectRules, Body, Coordinate, DegreeAspectKind,
    EndpointKey, Motion, Occupant, Placement, PlacementMotion,
};

fn pm(body: Body, degree360: f64) -> PlacementMotion {
    let sd = rubrum::SignDegree::new(degree360);
    let placement = Placement::new(Coordinate::SignDegree(sd), Occupant::Body(body));
    PlacementMotion::new(placement, Motion::Direct)
}

fn main() {
    // Example scenario: compute natal aspects repeatedly (e.g. in a batch job), and compile rules
    // once per run to avoid rebuilding lookup tables.
    //
    // Demonstrate finer-grain orb control:
    // - Conjunction base orb is 10°.
    // - Pluto conjunctions are tight (5°).
    // - Using `Min` means the tighter endpoint controls the pairwise orb.
    //   - Sun–anything conjunction: min(10, 10) => 10°
    //   - Pluto–anything conjunction: min(5, 10) => 5°
    let rules = AspectRules {
        aspects: vec![
            AspectRule::enabled(DegreeAspectKind::Conjunction, 10.0),
            AspectRule::enabled(DegreeAspectKind::Opposition, 8.0),
        ],
        orb_combine: rubrum::aspect::OrbCombine::Min,
        orb_overrides: vec![rubrum::aspect::OrbOverride {
            endpoint: EndpointKey::Body(Body::Pluto),
            kind: Some(DegreeAspectKind::Conjunction),
            orb_deg: 5.0,
        }],
        include_bodies: true,
        include_angles: false,
        include_chart_points: false,
        include_lots: false,
        ..Default::default()
    };

    // Compile once.
    let compiled = rules.compile();

    // Imagine these are the placements for many charts; we reuse the same compiled rules.
    let placements = vec![
        pm(Body::Sun, 0.0),
        pm(Body::Moon, 180.0),
        pm(Body::Mars, 3.0),
    ];

    // Hot-loop endpoint checks use `CompiledAspectRules`.
    let allowed: Vec<_> = placements
        .iter()
        .filter(|pm| compiled.endpoint_allowed(&pm.occupant()))
        .collect();

    println!("allowed endpoints: {}", allowed.len());

    // `compute_aspects_natal` already compiles internally for its own hot loops, so you can call it
    // directly for a single chart.
    let edges = compute_aspects_natal(&placements, &rules);

    for e in edges {
        println!("{} {} {} orb={:.2}", e.a.0, e.kind, e.b.0, e.orb);
    }
}
