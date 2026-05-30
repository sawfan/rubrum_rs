#![cfg_attr(
    debug_assertions,
    allow(
        dead_code,
        unused_imports,
        non_snake_case,
        unused_variables,
        unused_mut,
        non_camel_case_types,
        confusable_idents,
        uncommon_codepoints,
        mixed_script_confusables,
    )
)]
//! Strongly typed astrological primitives for chart and horoscope applications.
//!
//! `rubrum` models signs, houses, bodies, degrees, placements, aspects,
//! endpoint filters, rulerships, and related tables. The crate is intended as a
//! foundation layer for applications that compute or interpret charts, rather
//! than as an end-to-end ephemeris or UI engine.
//!
//! ## Consumer API contracts
//!
//! Stability-sensitive surfaces include crate-root re-exports (`use rubrum::*;`),
//! canonical string keys such as `Body::canonical_key()` and `EndpointKey`, serde
//! forms for public config/storage types, and aspect configuration via
//! `AspectRules` and compiled aspect rules.
//!
//! Canonical keys are storage/API identifiers. Presentation helpers such as
//! `symbol_text()` are user-facing and should not be treated as durable storage
//! keys unless explicitly documented.

pub mod parse;
pub use parse::*;

pub mod constants;
pub use constants::*;

pub mod representation;
pub use representation::*;

pub mod types;
pub use types::*;

pub mod util;

pub mod motion;
pub use motion::*;

pub mod ephemeris;
pub use ephemeris::*;

pub use util::*;

pub mod sign;
pub use sign::*;
pub mod sign_emoji;

pub mod house;
pub use house::*;

pub mod house_system;
pub use house_system::*;

pub mod zodiac_system;
pub use zodiac_system::*;

pub mod body;
pub use body::*;

pub mod body_group;
pub use body_group::*;

pub mod chart_point;
pub use chart_point::*;

pub mod angle;
pub use angle::*;

pub mod lot;
pub use lot::*;

pub mod body_sign;
pub use body_sign::*;

pub mod body_sign_degree;
pub use body_sign_degree::*;

pub mod body_sign_degree_relation;
pub use body_sign_degree_relation::*;

pub mod body_sign_degree_relation_group;
pub use body_sign_degree_relation_group::*;

pub mod orb;
pub use orb::*;

pub mod aspect;
pub use aspect::*;

pub mod degree30;
pub use degree30::*;

pub mod degree360;
pub use degree360::*;

pub mod sign_degree;
pub use sign_degree::*;

pub mod quality;
pub use quality::*;

pub mod placement;
pub use placement::*;

pub mod constellation;

pub mod chart;
pub use chart::*;

pub use constellation::*;

pub mod decan;
pub use decan::*;

pub mod element_kind;
pub use element_kind::*;

pub mod coordinate;
pub use coordinate::*;

pub mod occupant;
pub use occupant::*;

pub mod endpoint_key;
pub use endpoint_key::*;

pub mod endpoint_filter;
pub use endpoint_filter::*;

pub mod placement_relation;
pub use placement_relation::*;

pub mod moment_seed;
pub use moment_seed::*;

pub mod placement_relation_group;
pub use placement_relation_group::*;
pub mod placement_relation_group_aspect_match;
pub use placement_relation_group_aspect_match::*;

pub mod house_sign_degree;
pub use house_sign_degree::*;

pub mod sign_house;
pub use sign_house::*;

pub mod rulership;
pub use rulership::*;

#[cfg(test)]
mod tests {
    use super::*;
    use aspect::DegreeAspectKind::*;
    use body::Body::*;
    use house::House::*;
    use sign::Sign::*;

    #[test]
    fn it_works() {}
}
