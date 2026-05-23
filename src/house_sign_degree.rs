use super::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct HouseSignDegree {
    pub house: House,
    pub sign_degree: SignDegree,
}

impl HouseSignDegree {
    pub const fn new_const(house: House, sign_degree: SignDegree) -> Self {
        Self { house, sign_degree }
    }

    pub fn new(house: House, sign_degree: SignDegree) -> Self {
        Self::new_const(house, sign_degree)
    }
}
