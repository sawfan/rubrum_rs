use super::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SignHouse {
    pub sign: Sign,
    pub house: House,
}

impl SignHouse {
    pub const fn new_const(sign: Sign, house: House) -> Self {
        Self { sign, house }
    }

    pub fn new(sign: Sign, house: House) -> Self {
        Self::new_const(sign, house)
    }
}
