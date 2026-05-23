use super::*;

#[derive(Debug, Clone, Copy)]
pub struct BodySignDegree {
    pub body: Body,
    pub sign_degree: SignDegree,
}

impl BodySignDegree {
    pub fn forward_distance(&self, other: &BodySignDegree) -> f64 {
        forward_distance_sign_degree(&self.sign_degree, &other.sign_degree)
    }
    pub fn backward_distance(&self, other: &BodySignDegree) -> f64 {
        backward_distance_sign_degree(&self.sign_degree, &other.sign_degree)
    }

    pub const fn new_const(body: Body, sign_degree: SignDegree) -> Self {
        Self { body, sign_degree }
    }

    pub fn new(body: Body, sign_degree: SignDegree) -> Self {
        Self::new_const(body, sign_degree)
    }
}

use std::convert::From;

/// Canonical ergonomic conversion for test/data entry.
///
/// Order is `(Sign, Body, (deg, min, sec))`.
impl From<(Sign, Body, (i32, i32, i32))> for BodySignDegree {
    fn from(body_sign_degree: (Sign, Body, (i32, i32, i32))) -> Self {
        let sign = body_sign_degree.0;
        let body = body_sign_degree.1;
        let degree30: Degree30 = body_sign_degree.2.into();
        let sign_degree = SignDegree::from_sign_and_degree30_f64(sign, degree30.to_f64());
        BodySignDegree::new(body, sign_degree)
    }
}

//impl From<(SignBody, (i32, i32, i32))> for BodySignDegree {
//    fn from(signbody_degree: (SignBody, (i32, i32, i32))) -> Self {
//        let sign = signbody_degree.0.sign;
//        let body = signbody_degree.0.body;
//        //let signbody = signbody_degree.0;
//        let degree: Degree30 = signbody_degree.1.into();
//        BodySignDegree::new(body, sign_degree)
//    }
//}

//use std::fmt;
//impl fmt::Display for BodySignDegree {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        write!(f, "{}{} {}", self.body, self.degree, self.sign.symbol_text())
//    }
//}
//
//pub struct BodySignDegreeSymbolText<'a>(pub &'a BodySignDegree);
//impl <'a> fmt::Display for BodySignDegreeSymbolText<'a> {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        write!(f, "{} {} {}", self.0.body, self.0.degree, self.0.sign.symbol_text())
//    }
//}
//
//
