use serde::{Deserialize, Serialize};

use crate::{Motion, Placement, PlacementMotion};

/// Station classification derived from multiple ephemeris samples.
///
/// Note: distinguishing station-direct vs station-retrograde generally requires
/// speeds sampled on either side of the station event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StationKind {
    /// The object is turning from retrograde to direct.
    StationDirect,
    /// The object is turning from direct to retrograde.
    StationRetrograde,
}

/// Optional ephemeris fields typically available from astronomy/astrology engines.
///
/// This struct is intentionally flexible:
/// - Callers can supply only the values they have.
/// - Missing fields are treated as unknown (`None`).
/// - Serialization omits absent fields to avoid noisy output.
///
/// Units are explicit in field names to reduce ambiguity.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct EphemerisExtras {
    /// Ecliptic latitude (degrees).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latitude_deg: Option<f64>,

    /// Distance from Earth/Sun depending on ephemeris (AU).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distance_au: Option<f64>,

    /// Longitude speed (degrees/day).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speed_lon_deg_per_day: Option<f64>,

    /// Latitude speed (degrees/day).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speed_lat_deg_per_day: Option<f64>,

    /// Distance speed (AU/day).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speed_dist_au_per_day: Option<f64>,
}

impl EphemerisExtras {
    /// Convenience constructor for the common "speed lon" ephemeris field.
    pub fn with_lon_speed(speed_lon_deg_per_day: f64) -> Self {
        Self {
            speed_lon_deg_per_day: Some(speed_lon_deg_per_day),
            ..Default::default()
        }
    }

    /// Convenience constructor for common ecliptic ephemeris fields.
    ///
    /// This is intended for adapters that have a full Ephemeris-style tuple of values.
    pub fn from_ecliptic(
        latitude_deg: f64,
        distance_au: f64,
        speed_lon_deg_per_day: f64,
        speed_lat_deg_per_day: f64,
        speed_dist_au_per_day: f64,
    ) -> Self {
        Self {
            latitude_deg: Some(latitude_deg),
            distance_au: Some(distance_au),
            speed_lon_deg_per_day: Some(speed_lon_deg_per_day),
            speed_lat_deg_per_day: Some(speed_lat_deg_per_day),
            speed_dist_au_per_day: Some(speed_dist_au_per_day),
        }
    }

    /// Derive `Motion` from the longitude speed when available.
    ///
    /// - Negative speed => retrograde.
    /// - Zero or positive => direct.
    /// - Missing speed => `None`.
    pub fn motion_from_lon_speed(&self) -> Option<Motion> {
        self.speed_lon_deg_per_day.map(|s| {
            if s < 0.0 {
                Motion::Retrograde
            } else {
                Motion::Direct
            }
        })
    }

    /// Returns true if the longitude speed is within `threshold` of 0.
    ///
    /// If longitude speed is missing, this returns false.
    pub fn is_stationary_lon(&self, threshold: f64) -> bool {
        let threshold = threshold.abs();
        self.speed_lon_deg_per_day
            .is_some_and(|s| s.abs() <= threshold)
    }

    /// Infer station direction using two longitude speed samples around a suspected station.
    ///
    /// The logic is intentionally conservative to avoid classifying noise as a station:
    /// - `StationDirect` if `prev_speed < -threshold` and `next_speed > threshold`
    /// - `StationRetrograde` if `prev_speed > threshold` and `next_speed < -threshold`
    /// - Otherwise `None`
    pub fn station_kind_from_samples(
        prev_speed_lon_deg_per_day: f64,
        next_speed_lon_deg_per_day: f64,
        threshold: f64,
    ) -> Option<StationKind> {
        let threshold = threshold.abs();

        if prev_speed_lon_deg_per_day < -threshold && next_speed_lon_deg_per_day > threshold {
            Some(StationKind::StationDirect)
        } else if prev_speed_lon_deg_per_day > threshold && next_speed_lon_deg_per_day < -threshold
        {
            Some(StationKind::StationRetrograde)
        } else {
            None
        }
    }
}

/// A `PlacementMotion` plus optional ephemeris extras.
///
/// This keeps `Placement` / `PlacementMotion` lightweight while allowing richer datasets
/// to carry additional values like latitude, distance, and their speeds.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PlacementEphemeris {
    pub placement_motion: PlacementMotion,

    #[serde(default)]
    pub extras: EphemerisExtras,
}

impl PlacementEphemeris {
    pub fn new(placement_motion: PlacementMotion, extras: EphemerisExtras) -> Self {
        Self {
            placement_motion,
            extras,
        }
    }

    pub fn placement(&self) -> Placement {
        self.placement_motion.placement
    }

    pub fn motion(&self) -> Motion {
        self.placement_motion.motion
    }

    /// If longitude speed is present, update `motion` using it.
    ///
    /// This is useful when ephemeris data is considered canonical and `motion` is derived.
    pub fn with_motion_derived_from_speed(mut self) -> Self {
        if let Some(motion) = self.extras.motion_from_lon_speed() {
            self.placement_motion.motion = motion;
        }
        self
    }
}

impl From<PlacementMotion> for PlacementEphemeris {
    fn from(placement_motion: PlacementMotion) -> Self {
        Self {
            placement_motion,
            extras: EphemerisExtras::default(),
        }
    }
}

impl From<(PlacementMotion, EphemerisExtras)> for PlacementEphemeris {
    fn from(value: (PlacementMotion, EphemerisExtras)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl From<Placement> for PlacementEphemeris {
    fn from(placement: Placement) -> Self {
        PlacementMotion::from(placement).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extras_motion_from_speed() {
        assert_eq!(
            EphemerisExtras::with_lon_speed(-0.01).motion_from_lon_speed(),
            Some(Motion::Retrograde)
        );
        assert_eq!(
            EphemerisExtras::with_lon_speed(0.0).motion_from_lon_speed(),
            Some(Motion::Direct)
        );
        assert_eq!(EphemerisExtras::default().motion_from_lon_speed(), None);
    }

    #[test]
    fn extras_is_stationary_lon() {
        let threshold = 0.01;
        assert!(EphemerisExtras::with_lon_speed(0.0).is_stationary_lon(threshold));
        assert!(EphemerisExtras::with_lon_speed(-0.009).is_stationary_lon(threshold));
        assert!(!EphemerisExtras::with_lon_speed(-0.02).is_stationary_lon(threshold));
        assert!(!EphemerisExtras::default().is_stationary_lon(threshold));
    }

    #[test]
    fn station_kind_from_samples_basics() {
        let threshold = 0.01;

        assert_eq!(
            EphemerisExtras::station_kind_from_samples(-0.1, 0.1, threshold),
            Some(StationKind::StationDirect)
        );
        assert_eq!(
            EphemerisExtras::station_kind_from_samples(0.1, -0.1, threshold),
            Some(StationKind::StationRetrograde)
        );

        // No sign change.
        assert_eq!(
            EphemerisExtras::station_kind_from_samples(0.1, 0.05, threshold),
            None
        );

        // Within threshold (too close to call).
        assert_eq!(
            EphemerisExtras::station_kind_from_samples(0.005, -0.005, threshold),
            None
        );
    }

    #[test]
    fn extras_from_ecliptic_basics() {
        let extras = EphemerisExtras::from_ecliptic(1.25, 0.75, 0.02, -0.001, 0.0001);
        assert_eq!(extras.latitude_deg, Some(1.25));
        assert_eq!(extras.distance_au, Some(0.75));
        assert_eq!(extras.speed_lon_deg_per_day, Some(0.02));
        assert_eq!(extras.motion_from_lon_speed(), Some(Motion::Direct));
    }

    #[test]
    fn extras_serde_defaulting() {
        // Missing fields should default to None.
        let s = "{}";
        let parsed: EphemerisExtras = serde_json::from_str(s).unwrap();
        assert_eq!(parsed, EphemerisExtras::default());
    }

    #[test]
    fn placement_ephemeris_serde_round_trip() {
        // `toml` does not support serializing a top-level struct enum without wrapper in all cases.
        // Use JSON for a straightforward round-trip test.
        let pm = PlacementMotion {
            placement: Placement {
                coordinate: crate::Coordinate::Sign(crate::Sign::Aries),
                occupant: crate::Occupant::Body(crate::Body::Sun),
            },
            motion: Motion::Direct,
        };

        let pe = PlacementEphemeris::new(pm, EphemerisExtras::with_lon_speed(-0.5))
            .with_motion_derived_from_speed();

        assert_eq!(pe.motion(), Motion::Retrograde);

        let s = serde_json::to_string(&pe).unwrap();
        let parsed: PlacementEphemeris = serde_json::from_str(&s).unwrap();
        assert_eq!(parsed, pe);
    }
}
