use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlacementRelation {
    pub origin: Placement,
    pub destination: Placement,
    pub forward: f64,
    pub backward: f64,
}

impl PlacementRelation {
    pub fn new(origin: Placement, destination: Placement) -> Option<Self> {
        let origin_sd = origin.sign_degree()?;
        let dest_sd = destination.sign_degree()?;

        let forward = origin_sd.forward_distance(&dest_sd);
        let backward = origin_sd.backward_distance(&dest_sd);

        Some(Self {
            origin,
            destination,
            forward,
            backward,
        })
    }
}
