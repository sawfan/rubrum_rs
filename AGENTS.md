# AGENTS

This document is for humans and automated agents working on the **rubrum** project.

The goal is to describe what the project currently does, what primitives it exposes, and how the codebase is organized so future work (including AI-assisted changes) can be done safely.

---

## 1. Project summary

- **Name:** `rubrum`
- **Type:** Rust library crate (`src/lib.rs`, no `main.rs`)
- **Domain:** Astrology / horoscope engine
- **Purpose:** Provide strongly typed **astrological primitives** (signs, planets, houses, aspects, qualities, placements, etc.) and their relationships, to be used by higher-level applications that generate charts and horoscopes.

At this stage, the project focuses on **describing** and **structuring** the core astrological concepts, not on orchestrating end‑user workflows, building a UI, or shipping interpretation/keyword content.

---

## 2. Tooling and dependencies

From `Cargo.toml`:

- Rust edition: **2021**
- Key dependencies:
  - `strum` and `strum_macros` (0.27): enum utilities and derive macros for things like `EnumIter`, `AsRefStr`, etc.
  - `serde` (with `derive`): serialization/deserialization of core types for storage, APIs, or configuration.
  - `num_enum`: numeric <-> enum conversions.
  - `itertools`: richer iterator utilities.

There are no direct ephemeris-provider dependencies. Ephemeris integration should remain adapter-based and outside this core crate so `rubrum` stays provider-agnostic and license-neutral.

---

## 3. Public API surface (high level)

`src/lib.rs` re-exports most modules and their contents, so consumers can typically use the crate as:

```rust
use rubrum::*;
```

The following modules are public and re-exported:

- `constants` – shared astrological constants and tables
- `types` – shared types and aliases
- `util` – helper functions/utilities
- `sign` / `sign_emoji` – zodiac signs and their emoji representations
- `house` – astrological houses
- `body` – physical celestial bodies and their classifications
- `body_sign` – relationships between bodies and signs
- `body_sign_degree` – body placements with sign and degree
- `body_sign_degree_relation` – relationships derived from placements
- `body_sign_degree_relation_group` – grouped relations
- `body_sign_degree_relation_group_aspect_match` – aspect matching within groups of placements
- `orb` – aspect orbs and related logic
- `motion` – apparent motion state (`Motion`: Direct/Retrograde)
- `ephemeris` – optional ephemeris “extras” (latitude/distance/speeds) that can be attached to placements, plus station helpers:
  - `StationKind` (`StationDirect` / `StationRetrograde`) for classifying stations from multiple samples
  - `EphemerisExtras::{motion_from_lon_speed, is_stationary_lon, station_kind_from_samples}`
  - `PlacementEphemeris` (wraps `PlacementMotion` + `EphemerisExtras`)
- `chart_point` – computed ephemeris points (e.g. nodes, apogees)
- `angle` – chart angles/geometry points (Asc/MC/Vertex, etc.)
- `lot` – lots / Arabic parts (e.g. Part of Fortune)

- `aspect` – aspect kinds, configurable rules, compiled rules, and aspect computation helpers
- `degree30` – degree handling within a sign (0–29 degrees)
- `sign_degree` – degrees within the full zodiac mapped to signs
- `quality` – classical qualities (temperature, humidity, modality, triplicity, etc.)
- `placement` – combinations/placements of bodies in signs/houses/etc.
- `constellation` – constellations and related identifiers (IAU, NASA, Ptolemaic, etc.)
- `decan` – decans or related sub-sign structures
- `element_kind` – elemental classification (fire, earth, air, water)
- `coordinate` – chart coordinates combining signs, houses, and degrees
- `occupant` – occupancy of a coordinate (bodies or empty)

- `chart` – chart container types (ChartInfo, Chart) built from placements and optional cusp data
- `moment_seed` – serde seed/config model for moment, occurrence, and interval seed files

`rulership` exposes `RulershipKind` plus traditional and modern domicile rulership helpers.

---
- `placement_relation` / `placement_relation_group` – generic placement-to-placement angular relations
- `placement_relation_group_aspect_match` – primary aspect matching engine over `Placement` (with public helper for `PlacementMotion`)
- `endpoint_key` – stable, typed canonical keys for endpoints (Body/Angle/ChartPoint/Lot)
- `endpoint_filter` – user-facing endpoint display/selection filter (allow-list / deny-list + body group expansion)
- `body_group` – reusable body classifier groups (classical, modern, big 3, big 6, solar-system, etc.)
- `representation` – lightweight presentation helpers such as generated symbol lookups, without interpretive keyword content


## 4. Domain model and primitives

The crate is centered on **typed representations** of astrological concepts. Important categories include:

### 4.1 Signs and degrees

- `sign` and `sign_degree` model the 12 zodiac signs and the division of the zodiac into degrees.
- `degree30` is a helper for degrees within a single sign (0–29), useful for decans and intra-sign calculations.

### 4.2 Bodies and placements

- `body` defines celestial bodies and various groupings (e.g., classical vs modern planets, big 3/big 6, etc.).
- `body_sign` and `body_sign_degree` represent the placement of a body in a sign and specific degree.
- `placement` builds on these to represent more structured placements, using `Coordinate` (where in the chart) and `Occupant` (what, if anything, is there).

### 4.3 Houses and coordinates

- `house` and related constants link houses to bodies and signs.
- `coordinate` introduces:
  - `SignHouse` – a combined sign/house region.
  - `HouseSignDegree` – a house associated with an exact zodiac degree.
  - `Coordinate` enum – variants for `Sign`, `House`, `SignHouse`, and `HouseSignDegree`.
- `constants/house_body_sign_constants.rs` (via `constants`) encodes mappings like which bodies correlate with which houses in particular systems.
- `moment_seed` – serde model for seed files (`SeedFile` with `MomentSeed`/`OccurrenceSeed`/`IntervalSeed`) intended to be stored as TOML/JSON. `OccurrenceSeed.moment_id` is optional for compact single-moment seed files.


This allows reasoning both about concrete placements and about "empty" regions such as a sign/house combination without any occupying body.

### 4.4 Aspects and orbs

- `aspect` defines aspect kinds and concrete aspect types between degrees/signs/bodies.
- `orb` defines and manages allowable orbs for aspects.
- `body_sign_degree_relation_group_aspect_match` contains logic to match aspects within groups of placements.

### 4.5 Qualities and elements

Located under `quality` and `element_kind`:

- Classical qualities like **temperature** (hot/cold), **humidity**, **humor**, **metal**, **modality**, and **triplicity**.
- Elemental classification for signs/placements (fire, earth, air, water).

### 4.6 Interpretation boundary

Keyword vocabularies and narrative interpretation are intentionally outside this crate. `rubrum` provides the typed primitives, endpoints, placements, chart containers, aspect/rulership logic, and serialization-friendly domain structures that downstream interpretation crates or applications can consume.

### 4.7 Constellations

The `constellation` module models sky constellations and their identifiers.

### 4.8 Constants and traditional systems

The `constants` module exposes tables such as:

- Aspect constants
- Body-sign relationships
- Exaltations
- Thema Mundi
- House–body–sign mappings

### 4.9 Consumer-facing stability surfaces

The most important downstream contracts are:

- Crate-root re-exports (`use rubrum::*;`) for common types.
- Canonical string keys for endpoints and major classifiers (`EndpointKey`, `Body`, `Angle`, `ChartPoint`, `Lot`, `BodyGroup`).
- Serde representations for public data/config types.
- Aspect configuration via `AspectRules` and compiled aspect rules.
- Interpretation-free chart/placement structures that can cross crate, API, and storage boundaries.

Agents should treat these as stability-sensitive when changing public APIs.

---

## 5. Current state of the project

Based on the current code layout and metadata, the project is at a stage where:

- **Core primitives exist** for:
  - Signs, degrees, elements
  - Planets and other bodies, with several classification schemes
  - Houses
  - Aspects and orbs (at least at the type and constant level)
  - Placements of bodies in signs/degrees/houses
  - Classical qualities and triplicities
  - Constellations and identification schemes
  - Chart coordinates (`Coordinate`) and occupancy (`Occupant`) abstractions
- **Public re-exports** make the crate relatively easy to consume (`use rubrum::*;`).

The project is well-suited as a **foundation layer** for:

- Horoscope generation
- Chart interpretation engines
- Educational tools about astrology
- Higher-level applications that need typed, serializable representations of astrological concepts

At this time, the library does not appear to:

- Compute real-time ephemeris data; provider-specific integrations belong in separate adapter crates/applications.
- Handle time and location (birth data) end-to-end.
- Provide complete narrative interpretation text or keyword vocabularies; interpretation belongs in downstream crates/applications.

---

## 6. Design and coding style notes

- The library is **type-driven** with many small enums and structs rather than generic stringly typed representations.
- Data-heavy objective tables and associations are pulled into `constants` modules; subjective keyword/interpretation content belongs outside this crate.
- The public API is intentionally broad through `pub use` re-exports, but the underlying modules remain reasonably well-organized by concept.
- `#[cfg(test)]` unit tests live alongside the main code.

Agents modifying the code should:

1. Prefer adding new concepts as explicit enums/structs in the appropriate module rather than adding ad-hoc strings.
2. Keep new tables and associations in the relevant `constants` submodule when possible.
3. Respect the existing module boundaries.
4. Maintain serialization compatibility when changing public types that derive `serde` traits.
5. Add tests near any new non-trivial logic.

6. Keep subjective keyword vocabularies and narrative interpretation out of this crate.
7. Prefer fallible APIs (`Option`/`Result`) over panics for public lookup paths.
8. If replacing an older API with a better one, preserve compatibility where reasonable and consider `#[deprecated]` with migration guidance.

---

## 7. Potential future directions (inferred)

From the current structure and comments, likely future work includes:

- Aspect calculations with configurable orbs for arbitrary sets of placements.
- Decan-level modeling of placements.
- Rulership and dignity calculation helpers.
- Integration with ephemeris libraries to generate real charts from time and location.

---

## 8. Notes for future agents

When extending this project:

- **Locate the right module first.**
- **Use existing enums and types** instead of creating parallel ones.


## 9. Body groups / classifiers

Some common “body classifier” groupings are modeled explicitly to make it easy to filter/iterate bodies consistently across the crate.

- `BodyGroup` enum: `src/body_group.rs` (re-exported from `lib.rs`).
  - `BodyGroup::bodies() -> &'static [Body]`
  - `Body::is_in(BodyGroup) -> bool`
- Membership tables live in constants: `src/constants/body_group_constants.rs` (re-exported from `constants`).
  - `BIG_3_BODIES` (Sun + Moon; Ascendant is an `Angle` in this crate)
  - `BIG_6_BODIES`
  - `CLASSICAL_BODIES`
  - `MODERN_BODIES`
  - `SOLAR_SYSTEM_BODIES`

These sets are intentionally overlapping; a `Body` can belong to multiple groups.

## 10. Build-time generated constants


## 12. Recent agent changes (2026-03-07)

- Aspect computation hot paths now compile `AspectRules` once and reuse `CompiledAspectRules` for endpoint filtering and pairwise orb selection.
  - Updated both `aspect::compute::compute_aspects_natal` and the legacy placement aspect engine in `placement_relation_group_aspect_match`.
- `EndpointKey` now implements a stable ordering (`Ord`/`PartialOrd`) based on its canonical key string, enabling deterministic sorting without requiring ordering traits on `Body`/`Angle`/`ChartPoint`/`Lot`.
- Added schema validation tests for `EndpointKey`:
  - canonical string round-trips via `EndpointKey::parse`
  - serde round-trips and rejects unknown keys
  - TOML parsing rejects unknown keys
- Added `serde_json` as a dev-dependency to support serde round-trip tests.

- Added runnable examples:
  - `cargo run --example compiled_rules`
  - `cargo run --example endpoint_key`


Some numeric constants are generated at build time from TOML to improve maintainability while still producing `pub const` values.

- Source TOMLs:
  - `config/aspect_degrees.toml`
    - `[aspect_degrees]` generates `*_DEGREES` constants
  - `config/orb_degrees.toml`
    - `[orb_degrees]` generates `*_ORB_DEGREES` constants and `DEFAULT_ORB_DEGREES`
- Build script: `build.rs`
  - Parses the TOMLs and writes Rust code to: `$OUT_DIR/aspect_constants_generated.rs`
- Stable import path:
  - `src/constants/aspect_constants.rs` is a thin wrapper that `include!`s the generated file.
  - The rest of the crate continues to use: `use crate::constants::aspect_constants::*;`

When changing aspect/orb numbers, edit the appropriate TOML(s) above and rebuild.

- Keep constants and tables under `constants`.
- Maintain serialization compatibility for public types.
- Add tests for any non-trivial logic.

## 11. Test/data construction gotchas

- `SignDegree::from_sign_and_degree30_f64(sign, degrees)` expects `degrees` to be within a single sign (0..30). If you already have absolute ecliptic degrees (0..360), construct the value with `SignDegree::new(degrees360)` instead.

