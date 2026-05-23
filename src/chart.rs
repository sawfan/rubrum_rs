use super::*;

use serde::{Deserialize, Serialize};

/// Non-interpretive metadata describing how a chart was produced.
///
/// This intentionally avoids committing to a specific datetime/location library.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ChartInfo {
    /// Optional human-friendly label (e.g. "Natal chart", "Event chart").
    pub name: Option<String>,

    /// Optional notes about the chart.
    pub notes: Option<String>,

    /// Optional UTC timestamp representation.
    pub datetime_utc: Option<String>,

    /// Optional latitude in decimal degrees.
    pub latitude: Option<f64>,

    /// Optional longitude in decimal degrees.
    pub longitude: Option<f64>,
}

/// A computed chart: placements plus optional house cusp data.
///
/// This type is deliberately interpretation-free; it is meant to be used as input
/// to later "horoscope" or interpretation layers.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Chart {
    pub info: Option<ChartInfo>,

    /// All computed placements in the chart.
    ///
    /// Multiple occupants may share the same coordinate (e.g. conjunctions).
    pub placements: Vec<Placement>,

    /// House cusp longitudes, if computed (typically 12 entries).
    pub house_cusps: Vec<HouseSignDegree>,
}

impl Chart {
    pub fn new(placements: Vec<Placement>) -> Self {
        Self {
            info: None,
            placements,
            house_cusps: vec![],
        }
    }

    /// Returns placements whose occupants pass the provided endpoint filter.
    ///
    /// Note: this filter is based on endpoint identity (e.g. Body/Angle/ChartPoint/Lot).
    /// It does not filter on coordinate/sign/house.
    pub fn placements_filtered<'a>(
        &'a self,
        filter: &'a CompiledEndpointFilter,
    ) -> impl Iterator<Item = &'a Placement> + 'a {
        self.placements
            .iter()
            .filter(|p| filter.endpoint_allowed(p.occupant))
    }

    /// Returns occupants for placements that pass the provided endpoint filter.
    pub fn occupants_filtered<'a>(
        &'a self,
        filter: &'a CompiledEndpointFilter,
    ) -> impl Iterator<Item = Occupant> + 'a {
        self.placements_filtered(filter).map(|p| p.occupant)
    }

    pub fn with_info(mut self, info: ChartInfo) -> Self {
        self.info = Some(info);
        self
    }

    pub fn with_house_cusps(mut self, house_cusps: Vec<HouseSignDegree>) -> Self {
        self.house_cusps = house_cusps;
        self
    }

    pub fn placements_of(&self, occupant: Occupant) -> Vec<&Placement> {
        self.placements
            .iter()
            .filter(|p| p.occupant == occupant)
            .collect()
    }

    pub fn placements_in_sign(&self, sign: Sign) -> Vec<&Placement> {
        self.placements
            .iter()
            .filter(|p| p.sign() == Some(sign))
            .collect()
    }

    pub fn placements_in_house(&self, house: House) -> Vec<&Placement> {
        self.placements
            .iter()
            .filter(|p| p.house() == Some(house))
            .collect()
    }

    /// Returns the cusp longitude for a given house, if cusp data is present.
    pub fn house_cusp(&self, house: House) -> Option<SignDegree> {
        self.house_cusps
            .iter()
            .find(|hsd| hsd.house == house)
            .map(|hsd| hsd.sign_degree)
    }

    /// Returns occupants grouped by house for all 12 houses.
    ///
    /// Houses with no occupants will have an empty vec.
    pub fn occupants_by_house(&self) -> Vec<(House, Vec<Occupant>)> {
        House::default_order()
            .into_iter()
            .map(|house| {
                let occupants = self
                    .placements_in_house(house)
                    .into_iter()
                    .map(|p| p.occupant)
                    .collect::<Vec<Occupant>>();
                (house, occupants)
            })
            .collect()
    }

    /// Returns occupants grouped by sign, starting from the provided sign.
    ///
    /// Signs with no occupants will have an empty vec.
    pub fn occupants_by_sign_from(&self, start: Sign) -> Vec<(Sign, Vec<Occupant>)> {
        start
            .native_order()
            .into_iter()
            .map(|sign| {
                let occupants = self
                    .placements_in_sign(sign)
                    .into_iter()
                    .map(|p| p.occupant)
                    .collect::<Vec<Occupant>>();
                (sign, occupants)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use body::Body::*;
    use sign::Sign::*;

    #[test]
    fn chart_basics() {
        let placements = vec![
            (House::Seventh, Pisces, Mercury, (6, 14, 0)).into(),
            (House::Seventh, Pisces, Sun, (7, 12, 0)).into(),
        ];

        let chart = Chart::new(placements);

        assert_eq!(chart.placements_in_sign(Pisces).len(), 2);
        assert_eq!(chart.placements_in_sign(Leo).len(), 0);
        assert_eq!(chart.placements_of(Occupant::Body(Mercury)).len(), 1);
    }
}
