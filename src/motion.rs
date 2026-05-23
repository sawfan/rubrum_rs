use serde::{Deserialize, Serialize};

use crate::ParseKeyError;

use strum_macros::Display;
use strum_macros::EnumIter;
use strum_macros::IntoStaticStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MotionFormat {
    /// Stable storage/API key such as `"retrograde"`.
    Key,

    /// Human-readable English name such as `"Retrograde"`.
    Name,

    /// Short label such as `"R"`.
    Short,

    /// Rust enum variant name such as `"Retrograde"`.
    Debug,
}

/// Apparent motion state of an occupant at a given chart moment.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Display,
    PartialEq,
    Eq,
    EnumIter,
    IntoStaticStr,
    Hash,
    Serialize,
    Deserialize,
)]
pub enum Motion {
    #[default]
    Direct,
    Retrograde,
}

impl Motion {
    #[inline]
    pub const fn canonical_key(self) -> &'static str {
        match self {
            Motion::Direct => "direct",
            Motion::Retrograde => "retrograde",
        }
    }

    pub fn from_canonical_key(s: &str) -> Option<Self> {
        match s {
            "direct" => Some(Motion::Direct),
            "retrograde" => Some(Motion::Retrograde),
            _ => None,
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Motion::Direct => "Direct",
            Motion::Retrograde => "Retrograde",
        }
    }

    pub const fn short_label(self) -> &'static str {
        match self {
            Motion::Direct => "D",
            Motion::Retrograde => "R",
        }
    }

    pub fn format_motion(self, fmt: MotionFormat) -> String {
        match fmt {
            MotionFormat::Key => self.canonical_key().to_owned(),
            MotionFormat::Name => self.name().to_owned(),
            MotionFormat::Short => self.short_label().to_owned(),
            MotionFormat::Debug => format!("{:?}", self),
        }
    }
}

impl std::str::FromStr for Motion {
    type Err = ParseKeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_canonical_key(s).ok_or_else(|| ParseKeyError::new("Motion", s))
    }
}

impl TryFrom<&str> for Motion {
    type Error = ParseKeyError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_direct() {
        assert_eq!(Motion::default(), Motion::Direct);
    }

    #[test]
    fn serde_round_trip() {
        // `toml` does not support serializing a top-level enum by itself.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        struct Wrapper {
            motion: Motion,
        }

        let v = Wrapper {
            motion: Motion::Retrograde,
        };

        let s = toml::to_string(&v).unwrap();
        let parsed: Wrapper = toml::from_str(&s).unwrap();
        assert_eq!(parsed, v);
    }

    #[test]
    fn canonical_key_round_trips_with_from_str() {
        use strum::IntoEnumIterator;

        for motion in Motion::iter() {
            assert_eq!(
                Motion::from_canonical_key(motion.canonical_key()),
                Some(motion)
            );
            assert_eq!(motion.canonical_key().parse::<Motion>(), Ok(motion));
        }

        let err = "not_motion".parse::<Motion>().unwrap_err();
        assert_eq!(err, ParseKeyError::new("Motion", "not_motion"));
    }

    #[test]
    fn format_motion_supports_multiple_screen_representations() {
        assert_eq!(
            Motion::Retrograde.format_motion(MotionFormat::Key),
            "retrograde"
        );
        assert_eq!(
            Motion::Retrograde.format_motion(MotionFormat::Name),
            "Retrograde"
        );
        assert_eq!(Motion::Retrograde.format_motion(MotionFormat::Short), "R");
        assert_eq!(
            Motion::Retrograde.format_motion(MotionFormat::Debug),
            "Retrograde"
        );
    }
}
