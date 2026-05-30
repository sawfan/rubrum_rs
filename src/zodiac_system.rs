use serde::{Deserialize, Serialize};

/// Zodiac reference frame used for chart positions.
///
/// This is a backend-independent chart description type. Ephemeris adapters
/// (for example Swiss Ephemeris) are responsible for mapping these variants to
/// backend-specific flags/modes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ZodiacSystem {
    #[default]
    Tropical,
    Sidereal,
    Draconic,
}

impl ZodiacSystem {
    pub fn label(self) -> &'static str {
        match self {
            Self::Tropical => "Tropical",
            Self::Sidereal => "Sidereal",
            Self::Draconic => "Draconic",
        }
    }

    pub fn as_slug(self) -> &'static str {
        match self {
            Self::Tropical => "tropical",
            Self::Sidereal => "sidereal",
            Self::Draconic => "draconic",
        }
    }

    pub fn parse_slug(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "tropical" | "tropic" => Some(Self::Tropical),
            "sidereal" | "sidereal-zodiac" => Some(Self::Sidereal),
            "draconic" | "dragon" => Some(Self::Draconic),
            _ => None,
        }
    }

    pub fn all() -> &'static [Self] {
        &[Self::Tropical, Self::Sidereal, Self::Draconic]
    }
}

/// Sidereal ayanamsa/offset selection.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Ayanamsa {
    #[default]
    Lahiri,
    FaganBradley,
    Raman,
    Krishnamurti,
    Yukteswar,
    TrueChitra,
    TrueRevati,
}

impl Ayanamsa {
    pub fn label(self) -> &'static str {
        match self {
            Self::Lahiri => "Lahiri / Chitrapaksha",
            Self::FaganBradley => "Fagan-Bradley",
            Self::Raman => "Raman",
            Self::Krishnamurti => "Krishnamurti",
            Self::Yukteswar => "Yukteswar",
            Self::TrueChitra => "True Chitra",
            Self::TrueRevati => "True Revati",
        }
    }

    pub fn short_label(self) -> &'static str {
        match self {
            Self::Lahiri => "Lahiri",
            Self::FaganBradley => "Fagan-Bradley",
            Self::Raman => "Raman",
            Self::Krishnamurti => "Krishnamurti",
            Self::Yukteswar => "Yukteswar",
            Self::TrueChitra => "True Chitra",
            Self::TrueRevati => "True Revati",
        }
    }

    pub fn as_slug(self) -> &'static str {
        match self {
            Self::Lahiri => "lahiri",
            Self::FaganBradley => "fagan-bradley",
            Self::Raman => "raman",
            Self::Krishnamurti => "krishnamurti",
            Self::Yukteswar => "yukteswar",
            Self::TrueChitra => "true-chitra",
            Self::TrueRevati => "true-revati",
        }
    }

    pub fn parse_slug(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "lahiri" | "chitrapaksha" | "lahiri-chitrapaksha" => Some(Self::Lahiri),
            "fagan-bradley" | "fagan" | "bradley" => Some(Self::FaganBradley),
            "raman" => Some(Self::Raman),
            "krishnamurti" | "kp" => Some(Self::Krishnamurti),
            "yukteswar" | "yukteshwar" | "sri-yukteswar" => Some(Self::Yukteswar),
            "true-chitra" | "true_chitra" | "chitra" => Some(Self::TrueChitra),
            "true-revati" | "true_revati" | "revati" => Some(Self::TrueRevati),
            _ => None,
        }
    }

    pub fn all() -> &'static [Self] {
        &[
            Self::Lahiri,
            Self::FaganBradley,
            Self::Raman,
            Self::Krishnamurti,
            Self::Yukteswar,
            Self::TrueChitra,
            Self::TrueRevati,
        ]
    }
}
