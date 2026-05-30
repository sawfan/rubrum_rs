use super::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Coordinate {
    Sign(Sign),
    SignDegree(SignDegree),
    House(House),
    SignHouse(SignHouse),
    HouseSignDegree(HouseSignDegree),
}

impl Coordinate {
    pub fn rotated_by_degrees(&self, offset_degrees: f64) -> Self {
        match self {
            Coordinate::Sign(sign) => Coordinate::Sign(*sign),
            Coordinate::SignDegree(sign_degree) => {
                Coordinate::SignDegree(sign_degree.rotated_by_degrees(offset_degrees))
            }
            Coordinate::House(house) => Coordinate::House(*house),
            Coordinate::SignHouse(sign_house) => Coordinate::SignHouse(*sign_house),
            Coordinate::HouseSignDegree(house_sign_degree) => {
                Coordinate::HouseSignDegree(HouseSignDegree::new(
                    house_sign_degree.house,
                    house_sign_degree
                        .sign_degree
                        .rotated_by_degrees(offset_degrees),
                ))
            }
        }
    }

    pub fn sign(&self) -> Option<Sign> {
        match self {
            Coordinate::Sign(sign) => Some(*sign),
            Coordinate::SignDegree(sign_degree) => {
                let (sign, _) = sign_degree.sign_and_degree();
                Some(sign)
            }
            Coordinate::House(_) => None,
            Coordinate::SignHouse(sign_house) => Some(sign_house.sign),
            Coordinate::HouseSignDegree(house_sign_degree) => {
                let (sign, _) = house_sign_degree.sign_degree.sign_and_degree();
                Some(sign)
            }
        }
    }

    pub fn house(&self) -> Option<House> {
        match self {
            Coordinate::Sign(_) => None,
            Coordinate::SignDegree(_) => None,
            Coordinate::House(house) => Some(*house),
            Coordinate::SignHouse(sign_house) => Some(sign_house.house),
            Coordinate::HouseSignDegree(house_sign_degree) => Some(house_sign_degree.house),
        }
    }

    pub fn degree(&self) -> Option<Degree30> {
        match self {
            Coordinate::Sign(_) => None,
            Coordinate::SignDegree(sign_degree) => {
                let (_, degree30) = sign_degree.sign_and_degree();
                Some(degree30)
            }
            Coordinate::House(_) => None,
            Coordinate::SignHouse(_) => None,
            Coordinate::HouseSignDegree(house_sign_degree) => {
                let (_, degree30) = house_sign_degree.sign_degree.sign_and_degree();
                Some(degree30)
            }
        }
    }

    pub fn sign_degree(&self) -> Option<SignDegree> {
        match self {
            Coordinate::Sign(_) => None,
            Coordinate::SignDegree(sign_degree) => Some(*sign_degree),
            Coordinate::House(_) => None,
            Coordinate::SignHouse(_) => None,
            Coordinate::HouseSignDegree(house_sign_degree) => Some(house_sign_degree.sign_degree),
        }
    }
}
