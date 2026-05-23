use super::*;

#[derive(Debug, Clone)]
pub struct PlacementRelationGroup {
    pub placements: Vec<Placement>,
    pub relations: Vec<PlacementRelation>,
}

impl PlacementRelationGroup {
    pub fn new(placements: Vec<Placement>) -> Self {
        use itertools::Itertools;

        // Only keep placements that can actually participate in aspects.
        let placements = placements
            .into_iter()
            .filter(|p| p.sign_degree().is_some())
            .collect::<Vec<_>>();

        let relations = placements
            .clone()
            .into_iter()
            .permutations(2)
            .filter_map(|x| PlacementRelation::new(x[0], x[1]))
            .collect::<Vec<PlacementRelation>>();

        Self {
            placements,
            relations,
        }
    }
}
