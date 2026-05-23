use serde::{Deserialize, Serialize};
use std::fmt;

use crate::{
    AngleFormat, BodyFormat, ChartPointFormat, DegreeAspectKind, DegreeAspectKindFormat,
    EndpointKey, LotFormat, Occupant,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AspectEndpointDisplayFormat {
    /// Stable storage/API key such as `"sun"` or `"transit:sun"`.
    Key,

    /// Human-readable endpoint name such as `"Sun"`.
    Name,

    /// Symbol/glyph-oriented endpoint display such as `"☉"`.
    Symbol,

    /// Rust enum variant name where known, falling back to the raw endpoint id.
    Debug,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AspectEdgeDisplayOptions {
    pub endpoint_format: AspectEndpointDisplayFormat,
    pub aspect_format: DegreeAspectKindFormat,

    #[serde(default = "default_true")]
    pub include_orb: bool,

    #[serde(default)]
    pub include_exact_delta: bool,
}

fn default_true() -> bool {
    true
}

impl Default for AspectEdgeDisplayOptions {
    fn default() -> Self {
        Self {
            endpoint_format: AspectEndpointDisplayFormat::Key,
            aspect_format: DegreeAspectKindFormat::Symbol,
            include_orb: true,
            include_exact_delta: false,
        }
    }
}

impl AspectEndpointDisplayFormat {
    fn format_endpoint_key(self, key: EndpointKey) -> String {
        match key {
            EndpointKey::Body(body) => match self {
                AspectEndpointDisplayFormat::Key => body.format_body(BodyFormat::Key),
                AspectEndpointDisplayFormat::Name => body.format_body(BodyFormat::Name),
                AspectEndpointDisplayFormat::Symbol => body.format_body(BodyFormat::Symbol),
                AspectEndpointDisplayFormat::Debug => body.format_body(BodyFormat::Debug),
            },
            EndpointKey::Angle(angle) => match self {
                AspectEndpointDisplayFormat::Key => angle.format_angle(AngleFormat::Key),
                AspectEndpointDisplayFormat::Name => angle.format_angle(AngleFormat::Name),
                AspectEndpointDisplayFormat::Symbol => angle.format_angle(AngleFormat::Symbol),
                AspectEndpointDisplayFormat::Debug => angle.format_angle(AngleFormat::Debug),
            },
            EndpointKey::ChartPoint(point) => match self {
                AspectEndpointDisplayFormat::Key => point.format_chart_point(ChartPointFormat::Key),
                AspectEndpointDisplayFormat::Name => {
                    point.format_chart_point(ChartPointFormat::Name)
                }
                AspectEndpointDisplayFormat::Symbol => {
                    point.format_chart_point(ChartPointFormat::Symbol)
                }
                AspectEndpointDisplayFormat::Debug => {
                    point.format_chart_point(ChartPointFormat::Debug)
                }
            },
            EndpointKey::Lot(lot) => match self {
                AspectEndpointDisplayFormat::Key => lot.format_lot(LotFormat::Key),
                AspectEndpointDisplayFormat::Name => lot.format_lot(LotFormat::Name),
                AspectEndpointDisplayFormat::Symbol => lot.format_lot(LotFormat::Symbol),
                AspectEndpointDisplayFormat::Debug => lot.format_lot(LotFormat::Debug),
            },
        }
    }
}

fn format_endpoint_id(id: &AspectEndpointId, format: AspectEndpointDisplayFormat) -> String {
    let raw = id.0.as_str();
    let (dataset, key) = raw
        .split_once(':')
        .map_or((None, raw), |(dataset, key)| (Some(dataset), key));

    let formatted = EndpointKey::parse(key)
        .map(|endpoint| format.format_endpoint_key(endpoint))
        .unwrap_or_else(|_| key.to_owned());

    if let Some(dataset) = dataset {
        format!("{dataset}:{formatted}")
    } else {
        formatted
    }
}

fn format_degrees(value: f64) -> String {
    if value.fract().abs() < f64::EPSILON {
        format!("{}°", value as i64)
    } else {
        format!("{value:.2}°")
    }
}

/// Stable endpoint id for an aspect edge.
///
/// For now, this uses the occupant canonical key (e.g. "sun", "ascendant").
///
/// Endpoint id for an aspect edge.
///
/// This is a stable string token. By default we use the occupant canonical key (e.g. "sun",
/// "ascendant").
///
/// For cross-dataset aspects (transit ↔ natal), use [`AspectEndpointId::from_dataset_occupant`]
/// so the same occupant in different datasets remains distinguishable.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct AspectEndpointId(pub String);

impl AspectEndpointId {
    pub fn from_occupant(occupant: Occupant) -> Self {
        Self(occupant.canonical_key().to_owned())
    }

    pub fn from_dataset_occupant(dataset_id: &str, occupant: Occupant) -> Self {
        Self(format!("{}:{}", dataset_id, occupant.canonical_key()))
    }
}

/// A single consolidated aspect between two endpoints.
///
/// This is an undirected edge: endpoint ordering is canonicalized so the same pair is never
/// represented twice.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AspectEdge {
    pub a: AspectEndpointId,
    pub b: AspectEndpointId,
    pub kind: DegreeAspectKind,

    /// Smallest angular distance (in degrees) from exactness.
    pub orb: f64,

    /// Exact signed delta (in degrees) from the exact aspect angle.
    ///
    /// This is optional because most rendering needs only the absolute orb.
    #[serde(default)]
    pub exact_delta: Option<f64>,
}

impl AspectEdge {
    pub fn new(
        mut a: AspectEndpointId,
        mut b: AspectEndpointId,
        kind: DegreeAspectKind,
        orb: f64,
        exact_delta: Option<f64>,
    ) -> Self {
        if b < a {
            std::mem::swap(&mut a, &mut b);
        }

        Self {
            a,
            b,
            kind,
            orb,
            exact_delta,
        }
    }
    pub fn format_with_options(&self, options: &AspectEdgeDisplayOptions) -> String {
        let a = format_endpoint_id(&self.a, options.endpoint_format);
        let b = format_endpoint_id(&self.b, options.endpoint_format);
        let aspect = self.kind.format_degree_aspect_kind(options.aspect_format);

        let mut rendered = format!("{a} {aspect} {b}");

        if options.include_orb {
            rendered.push_str(&format!(" (orb {})", format_degrees(self.orb)));
        }

        if options.include_exact_delta {
            if let Some(delta) = self.exact_delta {
                rendered.push_str(&format!(" Δ{}", format_degrees(delta)));
            }
        }

        rendered
    }
}

impl fmt::Display for AspectEdge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.format_with_options(&AspectEdgeDisplayOptions::default())
        )
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AspectSet {
    pub edges: Vec<AspectEdge>,

    /// Optional hash of the rules/config used to compute this set.
    ///
    /// This is intended for cache invalidation.
    #[serde(default)]
    pub rules_hash: Option<String>,

    /// Optional human-readable version string.
    #[serde(default)]
    pub version: Option<String>,
}

impl AspectSet {
    pub fn new(edges: Vec<AspectEdge>) -> Self {
        Self {
            edges,
            rules_hash: None,
            version: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_edge_with_options_supports_endpoint_and_aspect_formats() {
        let edge = AspectEdge::new(
            AspectEndpointId::from_occupant(Occupant::Body(crate::Body::Sun)),
            AspectEndpointId::from_occupant(Occupant::Body(crate::Body::Moon)),
            DegreeAspectKind::Trine,
            1.25,
            Some(-1.25),
        );

        let options = AspectEdgeDisplayOptions {
            endpoint_format: AspectEndpointDisplayFormat::Symbol,
            aspect_format: DegreeAspectKindFormat::Symbol,
            include_orb: true,
            include_exact_delta: true,
        };

        assert_eq!(
            edge.format_with_options(&options),
            "☽ △ ☉ (orb 1.25°) Δ-1.25°"
        );
    }
}
