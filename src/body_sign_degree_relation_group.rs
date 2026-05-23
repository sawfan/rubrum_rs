use super::*;
use std::collections::HashMap;

pub type BodyDistanceHash = HashMap<Body, HashMap<Body, BodySignDegreeRelation>>;
#[derive(Debug, Clone)]
pub struct BodySignDegreeRelationGroup {
    pub body_sign_degrees: Vec<BodySignDegree>,
    pub combos: Vec<BodySignDegreeRelation>,
    //hashmap: HashMap<Body, Vec<BodySignDegreeDistance>>,
    pub hashmap: BodyDistanceHash,
}

impl BodySignDegreeRelationGroup {
    pub fn new(body_sign_degrees: Vec<BodySignDegree>) -> Self {
        use itertools::Itertools;
        let combos = body_sign_degrees
            .clone()
            .into_iter()
            //.combinations(2)
            .permutations(2)
            .map(|x| BodySignDegreeRelation::new(x[0], x[1]))
            .collect::<Vec<BodySignDegreeRelation>>();
        //let mut hashmap: HashMap<Body, Vec<BodySignDegreeDistance>> = HashMap::new();
        let mut hashmap: BodyDistanceHash = HashMap::new();
        for combo in &combos {
            let origin_body = combo.origin.body;
            let destination_body = combo.destination.body;
            hashmap
                .entry(origin_body)
                .and_modify(|c| {
                    c.insert(destination_body, combo.clone());
                })
                .or_insert(HashMap::from([(destination_body, combo.clone())]));
        }

        Self {
            body_sign_degrees,
            combos,
            hashmap,
        }
    }

    pub fn body_sign_degree(&self, body: Body) -> BodySignDegree {
        *self
            .body_sign_degrees
            .iter()
            .find(|x| x.body == body)
            .unwrap()
    }

    pub fn forward_distance(&self, body_kind_1: Body, body_kind_2: Body) -> f64 {
        let body_1_relations = self.hashmap.get(&body_kind_1).unwrap();
        body_1_relations.get(&body_kind_2).unwrap().forward
    }

    pub fn backward_distance(&self, body_kind_1: Body, body_kind_2: Body) -> f64 {
        let body_1_relations = self.hashmap.get(&body_kind_1).unwrap();
        body_1_relations.get(&body_kind_2).unwrap().backward
    }
}

//        let combos = ClassicalPlanetKind::body_combinations_2();
//        let placements = BodyPlacments::new(THEMA_MUNDI_BODY_SIGN_DEGREES.to_vec());
//        let r = combos
//            .iter()
//            .map(|p| {
//                let p1 = p[0];
//                //let p1 = placements
//                let p2 = p[1];
//                let forward = placements.forward_distance(p1, p2);
//                let backward = placements.backward_distance(p1, p2);
//                (p1, p2, forward, backward)
//            })
//            .collect::<Vec<(Body, Body, f64, f64)>>();
