use rubrum::{
    Body, BodyGroup, BodyGroupKey, Chart, CompiledEndpointFilter, Coordinate, EndpointFilter,
    EndpointKey, Occupant, Placement, SignDegree,
};

fn make_chart() -> Chart {
    // Small chart with a few bodies + an angle + a lot.
    let placements = vec![
        Placement::new(
            Coordinate::SignDegree(SignDegree::new(0.0)),
            Occupant::Body(Body::Sun),
        ),
        Placement::new(
            Coordinate::SignDegree(SignDegree::new(3.0)),
            Occupant::Body(Body::Mercury),
        ),
        Placement::new(
            Coordinate::SignDegree(SignDegree::new(12.0)),
            Occupant::Body(Body::Pluto),
        ),
        // Angle / lot coordinates are just placeholders for this example.
        Placement::new(
            Coordinate::SignDegree(SignDegree::new(90.0)),
            Occupant::Angle(rubrum::Angle::Ascendant),
        ),
        Placement::new(
            Coordinate::SignDegree(SignDegree::new(180.0)),
            Occupant::Lot(rubrum::Lot::Fortune),
        ),
    ];

    Chart::new(placements)
}

fn print_filtered(chart: &Chart, compiled: &CompiledEndpointFilter, label: &str) {
    let keys: Vec<_> = chart
        .occupants_filtered(compiled)
        .filter_map(EndpointKey::from_occupant)
        .collect();

    println!("{label}: {keys:?}");
}

fn main() {
    let chart = make_chart();

    // 1) Default filter: allow everything (except `Empty`).
    let compiled = EndpointFilter::default().compile();
    print_filtered(&chart, &compiled, "default");

    // 2) Allow-list mode: include only a group.
    // Since Big3 is defined as (Sun, Moon) in this crate, only Sun passes here.
    let filter = EndpointFilter {
        include_body_groups: vec![BodyGroupKey(BodyGroup::Big3)],
        ..Default::default()
    };
    let compiled = filter.compile();
    print_filtered(&chart, &compiled, "include big3");

    // 3) Allow-list mode with explicit includes + excludes.
    // Include Big3 + Mercury, but explicitly exclude Sun.
    let filter = EndpointFilter {
        include_body_groups: vec![BodyGroupKey(BodyGroup::Big3)],
        include_endpoints: vec![EndpointKey::Body(Body::Mercury)],
        exclude_endpoints: vec![EndpointKey::Body(Body::Sun)],
        ..Default::default()
    };
    let compiled = filter.compile();
    print_filtered(&chart, &compiled, "include big3+mercury, exclude sun");

    // 4) Kind toggles apply only when no allow-list is present.
    // Here we turn off bodies and leave angles/lots on.
    let filter = EndpointFilter {
        allow_bodies: false,
        allow_angles: true,
        allow_chart_points: true,
        allow_lots: true,
        ..Default::default()
    };
    let compiled = filter.compile();
    print_filtered(&chart, &compiled, "kind toggles (no bodies)");
}
