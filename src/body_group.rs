use super::*;
use crate::ParseKeyError;

use serde::{Deserialize, Serialize};

use strum_macros::{Display, EnumIter, IntoStaticStr};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    Display,
    EnumIter,
    IntoStaticStr,
)]
pub enum BodyGroupFormat {
    /// Stable storage/API key such as `"classical"`.
    Key,

    /// Human-readable English name such as `"Classical Bodies"`.
    Name,

    /// Rust enum variant name such as `"Classical"`.
    Debug,
}

/// A coarse classifier that groups multiple `Body` values.
///
/// Notes on definitions:
/// - Group semantics are project conventions and may not match every astrology school.
/// - Groups are intentionally overlapping; a `Body` can be in multiple groups.
/// - `Big3` is typically (Sun, Moon, Ascendant). Since `Ascendant` is an `Angle`
///   in this crate (not a `Body`), `Big3` here includes only (Sun, Moon).
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    Display,
    EnumIter,
    IntoStaticStr,
)]
pub enum BodyGroup {
    /// Sun + Moon (Ascendant is an `Angle`, not a `Body`).
    Big3,

    /// Sun, Moon, Mercury, Venus, Mars, Jupiter.
    Big6,

    /// The traditional visible planets + luminaries.
    Classical,

    /// Commonly used modern outer planets.
    Modern,

    /// Major solar system bodies modeled as `Body` (includes `Earth`).
    SolarSystem,
}

impl BodyGroup {
    /// Returns a stable, storage-friendly key for this group.
    #[inline]
    pub const fn canonical_key(self) -> &'static str {
        match self {
            BodyGroup::Big3 => "big3",
            BodyGroup::Big6 => "big6",
            BodyGroup::Classical => "classical",
            BodyGroup::Modern => "modern",
            BodyGroup::SolarSystem => "solarsystem",
        }
    }

    pub fn from_canonical_key(s: &str) -> Option<Self> {
        match s {
            "big3" => Some(BodyGroup::Big3),
            "big6" => Some(BodyGroup::Big6),
            "classical" => Some(BodyGroup::Classical),
            "modern" => Some(BodyGroup::Modern),
            "solarsystem" => Some(BodyGroup::SolarSystem),
            _ => None,
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            BodyGroup::Big3 => "Big 3",
            BodyGroup::Big6 => "Big 6",
            BodyGroup::Classical => "Classical Bodies",
            BodyGroup::Modern => "Modern Bodies",
            BodyGroup::SolarSystem => "Solar System Bodies",
        }
    }

    pub fn format_body_group(self, fmt: BodyGroupFormat) -> String {
        match fmt {
            BodyGroupFormat::Key => self.canonical_key().to_owned(),
            BodyGroupFormat::Name => self.name().to_owned(),
            BodyGroupFormat::Debug => format!("{:?}", self),
        }
    }

    /// Returns the bodies in this group.
    #[inline]
    pub const fn bodies(self) -> &'static [Body] {
        use crate::constants::*;
        match self {
            BodyGroup::Big3 => &BIG_3_BODIES,
            BodyGroup::Big6 => &BIG_6_BODIES,
            BodyGroup::Classical => &CLASSICAL_BODIES,
            BodyGroup::Modern => &MODERN_BODIES,
            BodyGroup::SolarSystem => &SOLAR_SYSTEM_BODIES,
        }
    }

    /// Convenience helper for membership checks.
    #[inline]
    pub fn contains(self, body: Body) -> bool {
        self.bodies().contains(&body)
    }
}

impl std::str::FromStr for BodyGroup {
    type Err = ParseKeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_canonical_key(s).ok_or_else(|| ParseKeyError::new("BodyGroup", s))
    }
}

impl TryFrom<&str> for BodyGroup {
    type Error = ParseKeyError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_key_is_stable() {
        assert_eq!(BodyGroup::Big3.canonical_key(), "big3");
        assert_eq!(BodyGroup::Big6.canonical_key(), "big6");
        assert_eq!(BodyGroup::Classical.canonical_key(), "classical");
        assert_eq!(BodyGroup::Modern.canonical_key(), "modern");
        assert_eq!(BodyGroup::SolarSystem.canonical_key(), "solarsystem");
    }

    #[test]
    fn bodies_are_reasonable() {
        assert!(BodyGroup::Big3.contains(Body::Sun));
        assert!(BodyGroup::Big3.contains(Body::Moon));
        assert!(!BodyGroup::Big3.contains(Body::Mercury));

        assert_eq!(BodyGroup::Big6.bodies().len(), 6);
        assert!(BodyGroup::Big6.contains(Body::Jupiter));

        assert_eq!(BodyGroup::Classical.bodies().len(), 7);
        assert!(BodyGroup::Classical.contains(Body::Saturn));

        assert_eq!(BodyGroup::Modern.bodies().len(), 3);
        assert!(BodyGroup::Modern.contains(Body::Uranus));

        assert!(BodyGroup::SolarSystem.contains(Body::Earth));
    }

    #[test]
    fn canonical_key_round_trips_with_from_str() {
        use strum::IntoEnumIterator;

        for group in BodyGroup::iter() {
            assert_eq!(
                BodyGroup::from_canonical_key(group.canonical_key()),
                Some(group)
            );
            assert_eq!(group.canonical_key().parse::<BodyGroup>(), Ok(group));
        }

        let err = "not_a_group".parse::<BodyGroup>().unwrap_err();
        assert_eq!(err, ParseKeyError::new("BodyGroup", "not_a_group"));
    }

    #[test]
    fn format_body_group_supports_multiple_screen_representations() {
        assert_eq!(
            BodyGroup::Classical.format_body_group(BodyGroupFormat::Key),
            "classical"
        );
        assert_eq!(
            BodyGroup::Classical.format_body_group(BodyGroupFormat::Name),
            "Classical Bodies"
        );
        assert_eq!(
            BodyGroup::Classical.format_body_group(BodyGroupFormat::Debug),
            "Classical"
        );
    }
}
