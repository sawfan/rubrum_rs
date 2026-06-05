use serde::{Deserialize, Serialize};

/// Astrological house division system.
///
/// This is a backend-independent chart description type. Ephemeris adapters
/// (for example Swiss Ephemeris) are responsible for mapping these variants to
/// backend-specific codes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HouseSystem {
    #[default]
    Placidus,
    /// Do not compute or render houses/angles. Useful when birth time is unknown.
    None,
    Koch,
    Porphyry,
    Regiomontanus,
    Campanus,
    Equal,
    WholeSign,
    Meridian,
    Alcabitius,
    Morinus,
    Topocentric,
}

impl HouseSystem {
    pub fn label(self) -> &'static str {
        match self {
            Self::Placidus => "Placidus",
            Self::None => "No houses",
            Self::Koch => "Koch",
            Self::Porphyry => "Porphyry",
            Self::Regiomontanus => "Regiomontanus",
            Self::Campanus => "Campanus",
            Self::Equal => "Equal",
            Self::WholeSign => "Whole Sign",
            Self::Meridian => "Meridian",
            Self::Alcabitius => "Alcabitius",
            Self::Morinus => "Morinus",
            Self::Topocentric => "Topocentric",
        }
    }

    pub fn as_slug(self) -> &'static str {
        match self {
            Self::Placidus => "placidus",
            Self::None => "none",
            Self::Koch => "koch",
            Self::Porphyry => "porphyry",
            Self::Regiomontanus => "regiomontanus",
            Self::Campanus => "campanus",
            Self::Equal => "equal",
            Self::WholeSign => "whole-sign",
            Self::Meridian => "meridian",
            Self::Alcabitius => "alcabitius",
            Self::Morinus => "morinus",
            Self::Topocentric => "topocentric",
        }
    }

    pub fn parse_slug(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "placidus" | "p" => Some(Self::Placidus),
            "none" | "no-houses" | "no_houses" | "nohouses" | "off" => Some(Self::None),
            "koch" | "k" => Some(Self::Koch),
            "porphyry" | "porphyrius" | "o" => Some(Self::Porphyry),
            "regiomontanus" | "regio" | "r" => Some(Self::Regiomontanus),
            "campanus" | "c" => Some(Self::Campanus),
            "equal" | "equal-house" | "a" | "e" => Some(Self::Equal),
            "whole-sign" | "wholesign" | "whole_sign" | "whole" | "w" => Some(Self::WholeSign),
            "meridian" | "axial" | "x" => Some(Self::Meridian),
            "alcabitius" | "alcabitus" | "b" => Some(Self::Alcabitius),
            "morinus" | "m" => Some(Self::Morinus),
            "topocentric" | "polich-page" | "t" => Some(Self::Topocentric),
            _ => None,
        }
    }

    pub fn all() -> &'static [Self] {
        &[
            Self::Placidus,
            Self::None,
            Self::Koch,
            Self::Porphyry,
            Self::Regiomontanus,
            Self::Campanus,
            Self::Equal,
            Self::WholeSign,
            Self::Meridian,
            Self::Alcabitius,
            Self::Morinus,
            Self::Topocentric,
        ]
    }
}
