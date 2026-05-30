use super::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Placement {
    pub coordinate: Coordinate,
    pub occupant: Occupant,
}

impl Placement {
    pub fn new(coordinate: Coordinate, occupant: Occupant) -> Self {
        Self {
            coordinate,
            occupant,
        }
    }

    pub fn rotated_by_degrees(&self, offset_degrees: f64) -> Self {
        Self {
            coordinate: self.coordinate.rotated_by_degrees(offset_degrees),
            occupant: self.occupant,
        }
    }

    pub fn house(&self) -> Option<House> {
        self.coordinate.house()
    }

    pub fn sign(&self) -> Option<Sign> {
        self.coordinate.sign()
    }

    pub fn sign_degree(&self) -> Option<SignDegree> {
        self.coordinate.sign_degree()
    }
}

/// A `Placement` plus its apparent motion state.
///
/// This allows representing retrograde objects without changing the core `Placement` type.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PlacementMotion {
    pub placement: Placement,
    pub motion: Motion,
}

impl PlacementMotion {
    pub fn new(placement: Placement, motion: Motion) -> Self {
        Self { placement, motion }
    }

    pub fn rotated_by_degrees(&self, offset_degrees: f64) -> Self {
        Self {
            placement: self.placement.rotated_by_degrees(offset_degrees),
            motion: self.motion,
        }
    }

    pub fn is_retrograde(&self) -> bool {
        self.motion == Motion::Retrograde
    }

    pub fn coordinate(&self) -> Coordinate {
        self.placement.coordinate
    }

    pub fn occupant(&self) -> Occupant {
        self.placement.occupant
    }

    pub fn house(&self) -> Option<House> {
        self.placement.house()
    }

    pub fn sign(&self) -> Option<Sign> {
        self.placement.sign()
    }

    pub fn sign_degree(&self) -> Option<SignDegree> {
        self.placement.sign_degree()
    }
}

impl From<Placement> for PlacementMotion {
    fn from(placement: Placement) -> Self {
        Self {
            placement,
            motion: Motion::Direct,
        }
    }
}

impl From<(Placement, Motion)> for PlacementMotion {
    fn from(value: (Placement, Motion)) -> Self {
        Self {
            placement: value.0,
            motion: value.1,
        }
    }
}

impl From<(House, Sign, Body, (i32, i32, i32))> for Placement {
    fn from(value: (House, Sign, Body, (i32, i32, i32))) -> Self {
        // Interpret the degree tuple as (deg, min, sec) within the sign.
        let (house, sign, body, (d, m, s)) = value;

        let degree30 = Degree30::from_i32(d, m, s);
        let sign_degree = SignDegree::from_sign_and_degree30_f64(sign, degree30.to_f64());

        let coordinate = Coordinate::HouseSignDegree(HouseSignDegree::new(house, sign_degree));

        Placement {
            coordinate,
            occupant: Occupant::Body(body),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let placement: Placement = (House::Fourth, Sign::Scorpio, Body::Mercury, (4, 14, 0)).into();
        assert_eq!(placement.house(), Some(House::Fourth));
        assert_eq!(placement.sign(), Some(Sign::Scorpio));
        assert_eq!(placement.occupant, Occupant::Body(Body::Mercury));
    }

    #[test]
    fn placement_motion_basics() {
        let placement: Placement = (House::Fourth, Sign::Scorpio, Body::Mercury, (4, 14, 0)).into();
        let pm = PlacementMotion::new(placement, Motion::Retrograde);
        assert!(pm.is_retrograde());
        assert_eq!(pm.house(), Some(House::Fourth));
        assert_eq!(pm.sign(), Some(Sign::Scorpio));
    }
}
