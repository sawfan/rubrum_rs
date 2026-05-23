use super::*;

#[derive(Debug, Clone)]
pub struct BodySignDegreeRelation {
    pub origin: BodySignDegree,
    pub destination: BodySignDegree,
    pub forward: f64,
    pub backward: f64,
}

impl BodySignDegreeRelation {
    pub fn new(origin: BodySignDegree, destination: BodySignDegree) -> Self {
        let forward = origin.forward_distance(&destination);
        let backward = origin.backward_distance(&destination);
        Self {
            origin,
            destination,
            forward,
            backward,
        }
    }
}
