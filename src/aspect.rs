pub mod sign_aspect_kind;
pub use sign_aspect_kind::*;

pub mod sign_aspect;
pub use sign_aspect::*;

pub mod degree_aspect_kind;
pub use degree_aspect_kind::*;

pub mod sign_body_degree_aspect;
pub use sign_body_degree_aspect::*;

pub mod degree_aspect_match_kind;
pub use degree_aspect_match_kind::*;

pub mod body_sign_degree_exact_aspect_matches;
pub use body_sign_degree_exact_aspect_matches::*;

pub mod exact_degree_aspect;
pub use exact_degree_aspect::*;

pub mod placement_exact_aspect_matches;
pub use placement_exact_aspect_matches::*;

pub mod aspect_rules;
pub use aspect_rules::*;

pub mod aspect_set;
pub use aspect_set::*;

pub mod compute;
pub use compute::*;

pub mod rules_hash;
pub use rules_hash::*;

pub mod aspect_rules_compiled;
pub use aspect_rules_compiled::*;

use super::*;

//use std::ops::{Add, Sub};
//impl Add for SignDegree {
//    type Output = Self;
//
//    fn add(self, other: Self) -> Self {
//        let self.degrees + other.degrees
//        //Self {x: self.x + other.x, y: self.y + other.y}
//    }
//}
//
//
//    if degrees1 > degrees2 {
//        higher_degree = degrees1;
//        lower_degree = degrees2;
//    } else {
//        higher_degree = degrees2;
//        lower_degree = degrees1;
//    }

pub fn backward_distance_sign_degree(
    from_sign_degree: &SignDegree,
    to_sign_degree: &SignDegree,
) -> f64 {
    backward_distance_degrees(from_sign_degree.degrees, to_sign_degree.degrees)
}

pub fn backward_distance_degrees(from_degree: f64, to_degree: f64) -> f64 {
    if from_degree >= to_degree {
        from_degree - to_degree // Will create positive number zero to less than 360
    } else {
        let degrees_from_end = CIRCLE_DEGREES - to_degree;
        from_degree + degrees_from_end
    }
}

pub fn forward_distance_sign_degree(
    from_sign_degree: &SignDegree,
    to_sign_degree: &SignDegree,
) -> f64 {
    forward_distance_degrees(from_sign_degree.degrees, to_sign_degree.degrees)
}

pub fn forward_distance_degrees(from_degree: f64, to_degree: f64) -> f64 {
    if from_degree <= to_degree {
        to_degree - from_degree // Will create positive number zero to less than 360
    } else {
        // from_degree < to_degree
        let degrees_to_end = CIRCLE_DEGREES - from_degree;
        degrees_to_end + to_degree
    }
}

//pub fn forward_distance(sign_degree1: &SignDegree, sign_degree2: &SignDegree) -> SignDegree {
//    sign_degree1.degrees - sign_degree2.degrees
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let from_start_to_mid_behind = backward_distance_degrees(0.0, 180.0);
        let from_mid_to_start_behind = backward_distance_degrees(180.0, 0.0);
        let from_start_to_mid_ahead = forward_distance_degrees(0.0, 180.0);
        let from_mid_to_end_ahead = forward_distance_degrees(180.0, 0.0);

        assert_eq!(from_start_to_mid_behind, 180.0);
        assert_eq!(from_mid_to_start_behind, 180.0);
        assert_eq!(from_start_to_mid_ahead, 180.0);
        assert_eq!(from_mid_to_end_ahead, 180.0);

        let forward = forward_distance_degrees(0.0, 45.0);
        assert_eq!(forward, 45.0);
        let backward = backward_distance_degrees(45.0, 0.0);
        assert_eq!(backward, 45.0);

        // same
        let forward = forward_distance_degrees(90.0, 90.0);
        assert_eq!(forward, 0.0);

        let backward = backward_distance_degrees(90.0, 90.0);
        assert_eq!(backward, 0.0);

        // loop around end
        let forward = forward_distance_degrees(359.0, 358.0);
        assert_eq!(forward, 359.0);

        //loop around beginning
        let backward = backward_distance_degrees(1.0, 2.0);
        assert_eq!(backward, 359.0);
    }
}
