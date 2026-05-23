use super::*;

#[derive(Debug, Clone)]
pub struct SignBody {
    pub sign: Sign,
    pub body: Body,
}

impl SignBody {
    pub const fn new_const(sign: Sign, body: Body) -> Self {
        Self { sign, body }
    }

    pub fn new(sign: Sign, body: Body) -> Self {
        Self { sign, body }
    }
}

use std::convert::From;
impl From<(Sign, Body)> for SignBody {
    fn from(body_sign: (Sign, Body)) -> Self {
        let sign = body_sign.0;
        let body = body_sign.1;
        SignBody::new(sign, body)
    }
}

//pub enum BodySign2 {
//}
//pub enum _BodySign2 {
//  _MajorSolarSystemBody,
//  _MinorSolarSystemBody,
//  _Other,
//}
