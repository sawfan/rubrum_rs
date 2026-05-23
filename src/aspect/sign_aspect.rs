use super::*;

pub struct SignAspect {
    kind: SignAspectKind,
    sign1: Sign,
    sign2: Sign,
}

impl SignAspect {
    pub fn new(kind: SignAspectKind, sign1: Sign, sign2: Sign) -> Self {
        Self { kind, sign1, sign2 }
    }
}

//pub struct SignBodyAspect {
//  kind: DegreeAspectKind,
//  body_position1: SignBody,
//  body_position2: SignBody,
//}
//
//impl SignBodyAspect {
//  pub fn new(
//    kind: DegreeAspectKind,
//    body_position1: SignBody,
//    body_position2: SignBody,
//  ) -> Self {
//    Self {
//      kind, body_position1, body_position2
//    }
//  }
//}
//
