use super::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Occupant {
    Empty,
    Body(Body),
    ChartPoint(ChartPoint),
    Angle(Angle),
    Lot(Lot),
}

impl Occupant {
    /// Returns a stable, storage-friendly key for this occupant.
    ///
    /// Keys are intended to be a long-term compatibility contract for consumers.
    /// Most keys are "leaf" keys (e.g. `sun`, `asc`). A small number of ambiguous
    /// concepts may return a qualified key (e.g. `chart_point:lilith_mean_apog`).
    pub fn canonical_key(&self) -> &'static str {
        match self {
            Occupant::Empty => "empty",
            Occupant::Body(body) => body.canonical_key(),
            Occupant::ChartPoint(chart_point) => chart_point.canonical_key(),
            Occupant::Angle(angle) => angle.canonical_key(),
            Occupant::Lot(lot) => lot.canonical_key(),
        }
    }

    pub fn body(&self) -> Option<Body> {
        match self {
            Occupant::Body(body) => Some(*body),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn occupant_canonical_key_basics() {
        assert_eq!(Occupant::Empty.canonical_key(), "empty");
        assert_eq!(Occupant::Body(Body::Sun).canonical_key(), "sun");
        assert_eq!(
            Occupant::Angle(Angle::Ascendant).canonical_key(),
            "ascendant"
        );
        assert_eq!(
            Occupant::ChartPoint(ChartPoint::MeanApog).canonical_key(),
            "mean_apog"
        );
    }
}
