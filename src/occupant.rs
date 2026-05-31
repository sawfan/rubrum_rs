use super::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Serialize, Deserialize)]
pub enum OccupantFormat {
    /// Stable storage/API key such as `"mars"` or `"ascendant"`.
    Key,

    /// Human-readable English name such as `"Mars"` or `"Ascendant"`.
    Name,

    /// Astrological glyph/short label such as `"♂"`, falling back to `Name` when unavailable.
    Symbol,

    /// Rust enum variant name such as `"Body(Mars)"`; useful for debugging/config migration.
    Debug,
}

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

    /// Returns a human-readable name for this occupant.
    ///
    /// This intentionally differs from `Display`, which remains symbol-oriented
    /// for historical compatibility with glyph-heavy chart rendering.
    pub fn name(&self) -> &'static str {
        match self {
            Occupant::Empty => "",
            Occupant::Body(body) => body.name(),
            Occupant::ChartPoint(chart_point) => chart_point.name(),
            Occupant::Angle(angle) => angle.name(),
            Occupant::Lot(lot) => lot.name(),
        }
    }

    /// Returns the configured astrological glyph/short-label representation.
    pub fn symbol_text(&self) -> String {
        match self {
            Occupant::Empty => String::new(),
            Occupant::Body(body) => body.symbol_text(),
            Occupant::ChartPoint(chart_point) => chart_point.symbol_text(),
            Occupant::Angle(angle) => angle.symbol_text(),
            Occupant::Lot(lot) => lot.symbol_text(),
        }
    }

    /// Formats this occupant for a requested presentation surface.
    pub fn format_occupant(&self, fmt: OccupantFormat) -> String {
        match fmt {
            OccupantFormat::Key => self.canonical_key().to_owned(),
            OccupantFormat::Name => self.name().to_owned(),
            OccupantFormat::Symbol => self.symbol_text(),
            OccupantFormat::Debug => format!("{:?}", self),
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

    #[test]
    fn format_occupant_supports_names_and_symbols() {
        let mars = Occupant::Body(Body::Mars);
        assert_eq!(mars.format_occupant(OccupantFormat::Key), "mars");
        assert_eq!(mars.format_occupant(OccupantFormat::Name), "Mars");
        assert_eq!(mars.format_occupant(OccupantFormat::Symbol), "♂");
        assert_eq!(mars.format_occupant(OccupantFormat::Debug), "Body(Mars)");

        let asc = Occupant::Angle(Angle::Ascendant);
        assert_eq!(asc.format_occupant(OccupantFormat::Name), "Ascendant");
        assert_eq!(asc.format_occupant(OccupantFormat::Symbol), "Asc");
    }
}
