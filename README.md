# rubrum

`rubrum` is a Rust library crate for strongly typed astrological primitives. It models signs, degrees, bodies, houses, aspects, placements, endpoint filters, chart containers, and related domain primitives so higher-level applications can build chart, horoscope, education, or research tools on top of a typed foundation.

The crate currently focuses on domain structure and relationship logic. It does **not** compute live ephemerides or full birth charts from time/place data end-to-end. It also intentionally does **not** ship interpretation keywords or narrative text; those belong in downstream interpretation crates or applications.

## Project boundaries

`rubrum` is intended to be the stable foundation layer for astrological software:

- **In scope:** typed signs, degrees, bodies, chart points, angles, lots, placements, charts, aspect rules/computation, endpoint keys/filters, rulership helpers, serialization-friendly seed/config models, and objective lookup tables.
- **Out of scope:** subjective keyword vocabularies, generated interpretation prose, UI workflows, and live ephemeris/time-location chart calculation.

Downstream projects can use `rubrum` as the typed input/output layer for chart computation, storage, APIs, or interpretation engines.

## Highlights

- Typed zodiac signs, houses, bodies, chart points, angles, lots, elements, and qualities.
- Typed sign-degree and degree-within-sign helpers.
- Body groups such as classical, modern, big 3, big 6, and solar-system bodies.
- Placement and chart container types.
- Aspect kinds, configurable aspect rules, compiled aspect rules, and natal aspect computation helpers.
- Stable endpoint keys for `Body`, `Angle`, `ChartPoint`, and `Lot` values.
- Endpoint filters with allow-list / deny-list behavior and body group expansion.
- Rulership helpers for traditional and modern sign rulerships.
- Serde support across core public types.
- Build-time generated aspect/orb constants from TOML configuration.
- Interpretation/keyword data intentionally lives outside this crate.

## Installation

Add the crate from this repository path or Git URL while it is unpublished:

````toml
[dependencies]
rubrum = { git = "https://github.com/sawfan/rubrum_rs" }
````

For local development:

````toml
[dependencies]
rubrum = { path = "../rubrum_rs" }
````

## Basic usage

Most common types are re-exported from the crate root:

````rust
use rubrum::*;

let sun = Body::Sun;
let sign = Sign::Leo;
let longitude = SignDegree::from_sign_and_degree30_f64(sign, 15.0);

let placement = Placement::new(
    Coordinate::SignDegree(longitude),
    Occupant::Body(sun),
);

assert_eq!(placement.sign_degree(), Some(longitude));
````

## Display and formatting

Use canonical keys for storage/API identifiers and explicit format helpers for screen output:

````rust
use rubrum::*;

assert_eq!(Body::Saturn.canonical_key(), "saturn");
assert_eq!(Body::Saturn.format_body(BodyFormat::Key), "saturn");
assert_eq!(Body::Saturn.format_body(BodyFormat::Name), "Saturn");
assert_eq!(Body::Saturn.format_body(BodyFormat::Symbol), "♄");

assert_eq!(Sign::Aries.format_sign(SignFormat::Name), "Aries");
assert_eq!(Sign::Aries.format_sign(SignFormat::Symbol), "♈");
assert_eq!(House::First.format_house(HouseFormat::Number), "1");
assert_eq!(Angle::Ascendant.format_angle(AngleFormat::Symbol), "Asc");
assert_eq!(ChartPoint::MeanNode.format_chart_point(ChartPointFormat::Symbol), "☊");
assert_eq!(Lot::Fortune.format_lot(LotFormat::Name), "Part of Fortune");
assert_eq!(ElementKind::Fire.format_element(ElementFormat::AlchemySymbol), "🜂");
assert_eq!(Motion::Retrograde.format_motion(MotionFormat::Short), "R");
assert_eq!(DegreeAspectKind::Trine.format_degree_aspect_kind(DegreeAspectKindFormat::Name), "Trine");
assert_eq!(DegreeAspectKind::Trine.format_degree_aspect_kind(DegreeAspectKindFormat::Symbol), "△");
assert_eq!(DegreeAspectKind::Trine.format_degree_aspect_kind(DegreeAspectKindFormat::Degrees), "120°");
````

The available format enums are:

- `BodyFormat`
- `SignFormat`
- `HouseFormat`
- `AngleFormat`
- `ChartPointFormat`
- `LotFormat`
- `ElementFormat`
- `MotionFormat`
- `BodyGroupFormat`
- `DegreeAspectKindFormat`

`symbol_text()` and `*Format::Symbol` are presentation-oriented helpers. Prefer `canonical_key()` for durable storage, APIs, and config files.

## Degrees

Use `SignDegree::new` when you already have absolute zodiac longitude in `0..360` degrees:

````rust
use rubrum::*;

let aries_15 = SignDegree::new(15.0);
let aquarius_10 = SignDegree::new(310.0);
````

Use `SignDegree::from_sign_and_degree30_f64` when you have a sign plus a degree inside that sign:

````rust
use rubrum::*;

let leo_3 = SignDegree::from_sign_and_degree30_f64(Sign::Leo, 3.0);
assert_eq!(leo_3.degrees, 123.0);
````

`Degree30` is available for degree/minute/second input within a single sign.

## Charts and placements

`Chart` stores placements and optional chart metadata / house cusps:

````rust
use rubrum::*;

let chart = Chart::new(vec![
    Placement::new(
        Coordinate::SignDegree(SignDegree::from_sign_and_degree30_f64(Sign::Cancer, 10.0)),
        Occupant::Body(Body::Sun),
    ),
    Placement::new(
        Coordinate::SignDegree(SignDegree::from_sign_and_degree30_f64(Sign::Libra, 12.0)),
        Occupant::Angle(Angle::Ascendant),
    ),
]);

assert_eq!(chart.placements_of(Occupant::Body(Body::Sun)).len(), 1);
````

## Endpoint keys and filters

`EndpointKey` provides a stable typed key for objects that can appear as chart endpoints:

````rust
use rubrum::*;

let key = EndpointKey::Body(Body::Mars);
assert_eq!(key.canonical_key(), "mars");
assert_eq!(EndpointKey::parse("mars"), Some(key));
````

`EndpointFilter` lets callers include/exclude endpoint kinds, explicit endpoints, and body groups:

````rust
use rubrum::*;

let filter = EndpointFilter {
    include_body_groups: vec![BodyGroupKey(BodyGroup::Modern)],
    include_endpoints: vec![EndpointKey::Angle(Angle::Ascendant)],
    ..Default::default()
};

let compiled = filter.compile();
assert!(compiled.allows(EndpointKey::Body(Body::Uranus)));
assert!(compiled.allows(EndpointKey::Angle(Angle::Ascendant)));
````

## Aspects

The `aspect` module contains aspect kinds, aspect rules, compiled rules, and computation helpers. The newer aspect APIs are designed around compiling `AspectRules` once and reusing the compiled form for repeated matching/filtering.

Aspect rules can be built from presets:

````rust
use rubrum::*;

let rules = AspectRules::major();
rules.validate().unwrap();

let broader_rules = AspectRules::major_and_minor();
broader_rules.validate().unwrap();
````

Aspect rule TOML should use canonical aspect keys such as `"trine"` and endpoint keys such as `"sun"`:

````toml
orb_combine = "max"
include_bodies = true
include_angles = true

[[aspects]]
kind = "conjunction"
orb_deg = 8.0

[[aspects]]
kind = "trine"
orb_deg = 6.0

[[orb_overrides]]
endpoint = "sun"
orb_deg = 10.0
````

Computed aspect edges can be formatted for display:

````rust
use rubrum::*;

let edge = AspectEdge::new(
    AspectEndpointId::from_occupant(Occupant::Body(Body::Sun)),
    AspectEndpointId::from_occupant(Occupant::Body(Body::Moon)),
    DegreeAspectKind::Trine,
    1.25,
    Some(-1.25),
);

let display = edge.format_with_options(&AspectEdgeDisplayOptions {
    endpoint_format: AspectEndpointDisplayFormat::Symbol,
    aspect_format: DegreeAspectKindFormat::Symbol,
    include_orb: true,
    include_exact_delta: true,
});

assert_eq!(display, "☽ △ ☉ (orb 1.25°) Δ-1.25°");
````

See runnable examples:

````sh
cargo run --example compiled_rules
cargo run --example endpoint_key
cargo run --example endpoint_filter
cargo run --example natal_aspects_with_filters
cargo run --example aspect_rules_from_toml
````

## Rulership

Traditional and modern domicile rulership helpers live in `rulership` and are re-exported from the crate root:

````rust
use rubrum::*;

assert_eq!(traditional_rulership(Body::Sun), Some(RulershipKind::Single(Sign::Leo)));
assert_eq!(modern_rulership(Body::Uranus), Some(RulershipKind::Single(Sign::Aquarius)));
````

## API stability and consumer contracts

The most consumer-facing APIs are:

- Crate-root re-exports: `use rubrum::*;`
- Stable canonical keys: `EndpointKey`, `Body::canonical_key`, `Angle::canonical_key`, `ChartPoint::canonical_key`, `Lot::canonical_key`, and `BodyGroup::canonical_key`.
- Serde support for public domain types that are useful in storage, APIs, or config files.
- Aspect APIs centered on `AspectRules`, compiled rules, endpoint filtering, and `compute_aspects_natal` / `compute_aspects_cross_datasets`.

When adding new public types, prefer stable string keys and serde behavior over exposing display text as an API contract. Human-facing language should live downstream.

## Constants and configuration

Some numeric aspect/orb constants are generated at build time from TOML files:

- `config/aspect_degrees.toml`
- `config/orb_degrees.toml`

The generated Rust code is included through `src/constants/aspect_constants.rs`, preserving a stable import path for crate users.

## Current limitations

- No built-in live ephemeris provider.
- No end-to-end chart calculation from birth/event datetime and geographic location.
- Interpretation and keyword content is intentionally kept out of this crate; downstream crates/applications own vocabulary and narrative generation.
- Some old relation/aspect modules remain for compatibility while newer compiled aspect-rule APIs evolve.

## Roadmap ideas for better downstream usability

Good next improvements for consumers:

- Add `FromStr`/`Display` implementations and typed parse errors for key public enums where only custom `parse` helpers exist today.
- Avoid panics in public lookup helpers; prefer `Option`, `Result`, or documented fallback behavior.
- Mark older compatibility APIs as deprecated once newer APIs cover the same use cases.
- Add more end-to-end examples for natal charts, transit-to-natal aspects, endpoint filtering from TOML, and chart serialization.
- Add schema/stability tests for canonical keys and serde forms so downstream data remains compatible.
- Expand crate-level and module-level rustdoc for docs.rs-style browsing.

## Development

Useful checks:

````sh
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features
````

When extending the crate, prefer explicit typed enums/structs over ad-hoc strings, keep data-heavy tables in `constants` or generated config, and add tests for non-trivial logic.
