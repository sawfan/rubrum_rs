// These constants are generated at build time from:
// - `config/aspect_degrees.toml`
// - `config/orb_degrees.toml`
//
// The generated file lives in Cargo's OUT_DIR. We keep this module path stable so
// the rest of the crate can continue to `use crate::constants::aspect_constants::*;`.
include!(concat!(env!("OUT_DIR"), "/aspect_constants_generated.rs"));
