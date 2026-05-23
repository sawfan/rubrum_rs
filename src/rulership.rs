use crate::{Body, Sign};
use serde::{Deserialize, Serialize};

/// One or more signs associated with a ruling body.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RulershipKind {
    Single(Sign),
    Dual([Sign; 2]),
}

impl RulershipKind {
    /// Returns the ruled signs as a borrowed slice.
    pub fn signs(&self) -> &[Sign] {
        match self {
            Self::Single(sign) => std::slice::from_ref(sign),
            Self::Dual(signs) => signs,
        }
    }

    /// Returns true when this rulership includes `sign`.
    pub fn contains(self, sign: Sign) -> bool {
        self.signs().contains(&sign)
    }
}

/// Returns the traditional domicile signs ruled by a body.
///
/// The luminaries rule one sign each; the five traditional non-luminary planets
/// rule two signs each. Modern outer planets are intentionally omitted here.
pub const fn traditional_rulership(body: Body) -> Option<RulershipKind> {
    match body {
        Body::Sun => Some(RulershipKind::Single(Sign::Leo)),
        Body::Moon => Some(RulershipKind::Single(Sign::Cancer)),
        Body::Mercury => Some(RulershipKind::Dual([Sign::Gemini, Sign::Virgo])),
        Body::Venus => Some(RulershipKind::Dual([Sign::Taurus, Sign::Libra])),
        Body::Mars => Some(RulershipKind::Dual([Sign::Aries, Sign::Scorpio])),
        Body::Jupiter => Some(RulershipKind::Dual([Sign::Sagittarius, Sign::Pisces])),
        Body::Saturn => Some(RulershipKind::Dual([Sign::Capricorn, Sign::Aquarius])),
        _ => None,
    }
}

/// Returns a modern rulership mapping for bodies with commonly used modern rulerships.
///
/// This keeps traditional rulerships for the visible planets and assigns the
/// outer planets to their common modern signs.
pub const fn modern_rulership(body: Body) -> Option<RulershipKind> {
    match body {
        Body::Uranus => Some(RulershipKind::Single(Sign::Aquarius)),
        Body::Neptune => Some(RulershipKind::Single(Sign::Pisces)),
        Body::Pluto => Some(RulershipKind::Single(Sign::Scorpio)),
        _ => traditional_rulership(body),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn traditional_rulerships_cover_classical_bodies() {
        assert_eq!(
            traditional_rulership(Body::Sun),
            Some(RulershipKind::Single(Sign::Leo))
        );
        assert_eq!(
            traditional_rulership(Body::Moon),
            Some(RulershipKind::Single(Sign::Cancer))
        );
        assert_eq!(
            traditional_rulership(Body::Mercury),
            Some(RulershipKind::Dual([Sign::Gemini, Sign::Virgo]))
        );
        assert!(traditional_rulership(Body::Uranus).is_none());
    }

    #[test]
    fn modern_rulerships_include_outer_planets() {
        assert_eq!(
            modern_rulership(Body::Uranus),
            Some(RulershipKind::Single(Sign::Aquarius))
        );
        assert_eq!(
            modern_rulership(Body::Neptune),
            Some(RulershipKind::Single(Sign::Pisces))
        );
        assert_eq!(
            modern_rulership(Body::Pluto),
            Some(RulershipKind::Single(Sign::Scorpio))
        );
    }

    #[test]
    fn rulership_kind_membership_works() {
        let rulership = RulershipKind::Dual([Sign::Gemini, Sign::Virgo]);
        assert!(rulership.contains(Sign::Gemini));
        assert!(!rulership.contains(Sign::Leo));
    }
}
