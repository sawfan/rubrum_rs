use crate::Body;

use Body::*;

/// Sun + Moon. (Ascendant is an `Angle` in this crate, not a `Body`.)
pub const BIG_3_BODIES: [Body; 2] = [Sun, Moon];

/// Sun, Moon, Mercury, Venus, Mars, Jupiter.
pub const BIG_6_BODIES: [Body; 6] = [Sun, Moon, Mercury, Venus, Mars, Jupiter];

/// Traditional visible planets + luminaries.
pub const CLASSICAL_BODIES: [Body; 7] = [Sun, Moon, Mercury, Venus, Mars, Jupiter, Saturn];

/// Commonly used modern outer planets.
pub const MODERN_BODIES: [Body; 3] = [Uranus, Neptune, Pluto];

/// Major solar system bodies modeled as `Body`.
///
/// Note: This is intentionally limited to the “major planets” represented in the
/// `Body` enum (plus `Earth`). It does not include asteroids, centaurs, etc.
pub const SOLAR_SYSTEM_BODIES: [Body; 11] = [
    Sun, Moon, Mercury, Venus, Earth, Mars, Jupiter, Saturn, Uranus, Neptune, Pluto,
];
