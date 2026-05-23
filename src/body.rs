pub mod body_symbol_text;
pub use body_symbol_text::*;

use super::*;
use crate::ParseKeyError;

use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

// TODO: Make a body format() method and enum type

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Serialize, Deserialize, EnumIter)]
pub enum BodyFormat {
    /// Stable storage/API key such as `"saturn"`.
    Key,

    /// Human-readable English name such as `"Saturn"`.
    Name,

    /// Astrological symbol/glyph such as `"♄"`, falling back to `Name` when unavailable.
    Symbol,

    /// Rust enum variant name such as `"Saturn"`; useful for debugging/config migration.
    Debug,
}

/// Physical bodies that can be treated as ephemeris-resolvable objects.
///
/// Non-physical points (nodes, apogees, angles, lots, etc.) should be modeled
/// via `ChartPoint`, `Angle`, and `Lot` and placed in a chart via `Occupant`.
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Serialize, Deserialize, EnumIter)]
pub enum Body {
    // Major bodies / planets
    Sun,
    Moon,
    Mercury,
    Venus,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune,
    Pluto,

    // Physical objects used in some systems
    Earth,

    // Minor bodies
    Chiron,
    Pholus,

    // Asteroids
    Ceres,
    Pallas,
    Juno,
    Vesta,

    // Additional asteroid identifiers may be supported by downstream ephemeris adapters.
    Astraea,
    Hebe,
    Iris,
    Flora,
    Metis,
    Hygiea,
    Urania,
    IsisAstroid,
    Hilda,
    Philosophia,
    Sophia,
    Aletheia,
    Sapientia,
    Thule,
    Ursula,
    Eros,
    CupidoAstroid,
    Hidalgo,
    Amor,
    Kama,
    Aphrodite,
    Apollo,
    Damocles,
    Cruithne,
    PoseidonAstroid,
    Vulcano,
    ZeusAstroid,
    Nessus,
    // TODO:
}

impl Body {
    #[inline]
    pub const fn canonical_key(self) -> &'static str {
        match self {
            Body::Sun => "sun",
            Body::Moon => "moon",
            Body::Mercury => "mercury",
            Body::Venus => "venus",
            Body::Mars => "mars",
            Body::Jupiter => "jupiter",
            Body::Saturn => "saturn",
            Body::Uranus => "uranus",
            Body::Neptune => "neptune",
            Body::Pluto => "pluto",
            Body::Earth => "earth",
            Body::Chiron => "chiron",
            Body::Pholus => "pholus",
            Body::Ceres => "ceres",
            Body::Pallas => "pallas",
            Body::Juno => "juno",
            Body::Vesta => "vesta",
            Body::Astraea => "astraea",
            Body::Hebe => "hebe",
            Body::Iris => "iris",
            Body::Flora => "flora",
            Body::Metis => "metis",
            Body::Hygiea => "hygiea",
            Body::Urania => "urania",
            Body::IsisAstroid => "isis_asteroid",
            Body::Hilda => "hilda",
            Body::Philosophia => "philosophia",
            Body::Sophia => "sophia",
            Body::Aletheia => "aletheia",
            Body::Sapientia => "sapientia",
            Body::Thule => "thule",
            Body::Ursula => "ursula",
            Body::Eros => "eros",
            Body::CupidoAstroid => "cupido_asteroid",
            Body::Hidalgo => "hidalgo",
            Body::Amor => "amor",
            Body::Kama => "kama",
            Body::Aphrodite => "aphrodite",
            Body::Apollo => "apollo",
            Body::Damocles => "damocles",
            Body::Cruithne => "cruithne",
            Body::PoseidonAstroid => "poseidon_asteroid",
            Body::Vulcano => "vulcano",
            Body::ZeusAstroid => "zeus_asteroid",
            Body::Nessus => "nessus",
        }
    }

    pub fn from_canonical_key(s: &str) -> Option<Self> {
        match s {
            "sun" => Some(Body::Sun),
            "moon" => Some(Body::Moon),
            "mercury" => Some(Body::Mercury),
            "venus" => Some(Body::Venus),
            "mars" => Some(Body::Mars),
            "jupiter" => Some(Body::Jupiter),
            "saturn" => Some(Body::Saturn),
            "uranus" => Some(Body::Uranus),
            "neptune" => Some(Body::Neptune),
            "pluto" => Some(Body::Pluto),
            "earth" => Some(Body::Earth),
            "chiron" => Some(Body::Chiron),
            "pholus" => Some(Body::Pholus),
            "ceres" => Some(Body::Ceres),
            "pallas" => Some(Body::Pallas),
            "juno" => Some(Body::Juno),
            "vesta" => Some(Body::Vesta),
            "astraea" => Some(Body::Astraea),
            "hebe" => Some(Body::Hebe),
            "iris" => Some(Body::Iris),
            "flora" => Some(Body::Flora),
            "metis" => Some(Body::Metis),
            "hygiea" => Some(Body::Hygiea),
            "urania" => Some(Body::Urania),
            "isis_asteroid" => Some(Body::IsisAstroid),
            "hilda" => Some(Body::Hilda),
            "philosophia" => Some(Body::Philosophia),
            "sophia" => Some(Body::Sophia),
            "aletheia" => Some(Body::Aletheia),
            "sapientia" => Some(Body::Sapientia),
            "thule" => Some(Body::Thule),
            "ursula" => Some(Body::Ursula),
            "eros" => Some(Body::Eros),
            "cupido_asteroid" => Some(Body::CupidoAstroid),
            "hidalgo" => Some(Body::Hidalgo),
            "amor" => Some(Body::Amor),
            "kama" => Some(Body::Kama),
            "aphrodite" => Some(Body::Aphrodite),
            "apollo" => Some(Body::Apollo),
            "damocles" => Some(Body::Damocles),
            "cruithne" => Some(Body::Cruithne),
            "poseidon_asteroid" => Some(Body::PoseidonAstroid),
            "vulcano" => Some(Body::Vulcano),
            "zeus_asteroid" => Some(Body::ZeusAstroid),
            "nessus" => Some(Body::Nessus),
            _ => None,
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Body::Sun => "Sun",
            Body::Moon => "Moon",
            Body::Mercury => "Mercury",
            Body::Venus => "Venus",
            Body::Mars => "Mars",
            Body::Jupiter => "Jupiter",
            Body::Saturn => "Saturn",
            Body::Uranus => "Uranus",
            Body::Neptune => "Neptune",
            Body::Pluto => "Pluto",
            Body::Earth => "Earth",
            Body::Chiron => "Chiron",
            Body::Pholus => "Pholus",
            Body::Ceres => "Ceres",
            Body::Pallas => "Pallas",
            Body::Juno => "Juno",
            Body::Vesta => "Vesta",
            Body::Astraea => "Astraea",
            Body::Hebe => "Hebe",
            Body::Iris => "Iris",
            Body::Flora => "Flora",
            Body::Metis => "Metis",
            Body::Hygiea => "Hygiea",
            Body::Urania => "Urania",
            Body::IsisAstroid => "Isis Asteroid",
            Body::Hilda => "Hilda",
            Body::Philosophia => "Philosophia",
            Body::Sophia => "Sophia",
            Body::Aletheia => "Aletheia",
            Body::Sapientia => "Sapientia",
            Body::Thule => "Thule",
            Body::Ursula => "Ursula",
            Body::Eros => "Eros",
            Body::CupidoAstroid => "Cupido Asteroid",
            Body::Hidalgo => "Hidalgo",
            Body::Amor => "Amor",
            Body::Kama => "Kama",
            Body::Aphrodite => "Aphrodite",
            Body::Apollo => "Apollo",
            Body::Damocles => "Damocles",
            Body::Cruithne => "Cruithne",
            Body::PoseidonAstroid => "Poseidon Asteroid",
            Body::Vulcano => "Vulcano",
            Body::ZeusAstroid => "Zeus Asteroid",
            Body::Nessus => "Nessus",
        }
    }

    pub fn format_body(self, fmt: BodyFormat) -> String {
        match fmt {
            BodyFormat::Key => self.canonical_key().to_owned(),
            BodyFormat::Name => self.name().to_owned(),
            BodyFormat::Symbol => self.symbol_text(),
            BodyFormat::Debug => format!("{:?}", self),
        }
    }

    /// Returns true if this body is a member of the given `BodyGroup`.
    #[inline]
    pub fn is_in(self, group: BodyGroup) -> bool {
        group.contains(self)
    }

    pub fn symbol_text(&self) -> String {
        body_symbol_text(self)
    }
}

impl std::str::FromStr for Body {
    type Err = ParseKeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_canonical_key(s).ok_or_else(|| ParseKeyError::new("Body", s))
    }
}

impl TryFrom<&str> for Body {
    type Error = ParseKeyError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

use std::fmt;
impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", BodySymbolText(self))
    }
}

pub struct BodySymbolText<'a>(&'a Body);
impl<'a> fmt::Display for BodySymbolText<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.symbol_text())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn body_is_in_group() {
        assert!(Body::Sun.is_in(BodyGroup::Big3));
        assert!(Body::Moon.is_in(BodyGroup::Big3));
        assert!(!Body::Mercury.is_in(BodyGroup::Big3));

        assert!(Body::Saturn.is_in(BodyGroup::Classical));
        assert!(!Body::Saturn.is_in(BodyGroup::Modern));

        assert!(Body::Earth.is_in(BodyGroup::SolarSystem));
    }

    #[test]
    fn canonical_key_round_trips_with_from_str() {
        use strum::IntoEnumIterator;

        for body in Body::iter() {
            assert_eq!(Body::from_canonical_key(body.canonical_key()), Some(body));
            assert_eq!(body.canonical_key().parse::<Body>(), Ok(body));
        }

        let err = "definitely_not_real".parse::<Body>().unwrap_err();
        assert_eq!(err, ParseKeyError::new("Body", "definitely_not_real"));
    }

    #[test]
    fn format_body_supports_multiple_screen_representations() {
        assert_eq!(Body::Saturn.format_body(BodyFormat::Key), "saturn");
        assert_eq!(Body::Saturn.format_body(BodyFormat::Name), "Saturn");
        assert_eq!(Body::Saturn.format_body(BodyFormat::Symbol), "♄");
        assert_eq!(Body::Saturn.format_body(BodyFormat::Debug), "Saturn");
    }
}
