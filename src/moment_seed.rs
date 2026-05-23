//! Seed file format (TOML/JSON) for "moments" and their associated occurrences/intervals.
//!
//! A "moment" is the astrological base unit: time + place.
//! - Multiple occurrences ("things that happened") can refer to the same moment.
//! - Intervals (wars/terms/school-years) refer to two moments (start/end).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct LocationSeed {
    #[serde(default)]
    pub name: Option<String>,

    #[serde(default)]
    pub lat: Option<f64>,

    #[serde(default)]
    pub lon: Option<f64>,
}

/// A time + place.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MomentSeed {
    /// Optional stable ID. If omitted, importers may derive it from filename.
    #[serde(default)]
    pub id: Option<String>,

    /// Human-friendly local datetime string.
    ///
    /// Example: "1969-07-20 20:17:40"
    ///
    /// Importers treat this as the canonical representation of what the user entered.
    pub local_datetime: String,

    /// IANA timezone name.
    ///
    /// Examples: "UTC", "America/New_York"
    #[serde(default)]
    pub timezone: Option<String>,

    /// Optional explicit UTC datetime string (ISO-8601).
    ///
    /// Example: "1969-07-20T20:17:40Z"
    ///
    /// This is optional because some datasets only have local time.
    #[serde(default)]
    pub utc_datetime: Option<String>,

    #[serde(default)]
    pub location: LocationSeed,

    /// Optional tags that apply to the moment itself.
    #[serde(default)]
    pub tags: Vec<String>,
}

/// A "thing that happened" at a moment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OccurrenceSeed {
    #[serde(default)]
    pub id: Option<String>,

    /// Foreign key to a moment (by id).
    ///
    /// This is optional to allow compact single-moment seed files. Importers may
    /// derive it when exactly one moment exists in the seed file.
    #[serde(default)]
    pub moment_id: Option<String>,

    pub title: String,

    #[serde(default)]
    pub summary: Option<String>,

    #[serde(default)]
    pub notes: Option<String>,

    #[serde(default)]
    pub source: Option<String>,

    /// Optional tags that apply to the occurrence.
    #[serde(default)]
    pub tags: Vec<String>,
}

/// A duration modeled as two moments.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntervalSeed {
    #[serde(default)]
    pub id: Option<String>,

    pub title: String,

    #[serde(default)]
    pub summary: Option<String>,

    #[serde(default)]
    pub notes: Option<String>,

    /// Optional entity/grouping id (e.g. "war:ww2", "person:obama", "school:harvard").
    #[serde(default)]
    pub entity_id: Option<String>,

    /// Start moment.
    pub start_moment_id: String,

    /// End moment (optional for open-ended intervals).
    #[serde(default)]
    pub end_moment_id: Option<String>,

    #[serde(default)]
    pub source: Option<String>,

    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SeedFile {
    #[serde(default)]
    pub moments: Vec<MomentSeed>,

    #[serde(default)]
    pub occurrences: Vec<OccurrenceSeed>,

    #[serde(default)]
    pub intervals: Vec<IntervalSeed>,
}

impl OccurrenceSeed {
    pub fn summary_or_empty(&self) -> &str {
        self.summary.as_deref().unwrap_or("")
    }
}

impl IntervalSeed {
    pub fn summary_or_empty(&self) -> &str {
        self.summary.as_deref().unwrap_or("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_from_toml() {
        let s = r#"
[[moments]]
id = "m1"
local_datetime = "1969-07-20 20:17:40"
timezone = "UTC"
utc_datetime = "1969-07-20T20:17:40Z"

[moments.location]
name = "Somewhere"
lat = 1.0
lon = 2.0

[[occurrences]]
id = "o1"
moment_id = "m1"
title = "Apollo 11 landing"
tags = ["space"]
"#;

        let seed: SeedFile = toml::from_str(s).unwrap();
        assert_eq!(seed.moments.len(), 1);
        assert_eq!(seed.occurrences.len(), 1);
        assert_eq!(seed.occurrences[0].moment_id.as_deref(), Some("m1"));
    }

    #[test]
    fn parses_occurrence_without_moment_id() {
        let s = r#"
[[moments]]
id = "m1"
local_datetime = "1969-07-20 20:17:40"
timezone = "UTC"

[[occurrences]]
id = "o1"
title = "Apollo 11 landing"
"#;

        let seed: SeedFile = toml::from_str(s).unwrap();
        assert_eq!(seed.moments.len(), 1);
        assert_eq!(seed.occurrences.len(), 1);
        assert_eq!(seed.occurrences[0].moment_id, None);
    }
}
