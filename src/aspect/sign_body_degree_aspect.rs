use super::*;

#[derive(Debug, Clone)]
pub struct BodyDegreeSignAspect {
    pub kind: DegreeAspectKind,
    pub body_position1: BodySignDegree,
    pub body_position2: BodySignDegree,
}

impl BodyDegreeSignAspect {
    pub fn new_const(
        kind: DegreeAspectKind,
        body_position1: BodySignDegree,
        body_position2: BodySignDegree,
    ) -> Self {
        Self {
            kind,
            body_position1,
            body_position2,
        }
    }

    pub fn new(
        kind: DegreeAspectKind,
        body_position1: BodySignDegree,
        body_position2: BodySignDegree,
    ) -> Self {
        Self::new_const(kind, body_position1, body_position2)
        //    Self {
        //      kind, body_position1, body_position2
        //    }
    }
}

use std::convert::From;

impl From<SignBodyDegreeAspectTuple2> for BodyDegreeSignAspect {
    fn from(sign_body_degree_aspect: SignBodyDegreeAspectTuple2) -> Self {
        let kind = sign_body_degree_aspect.0;
        let body1 = sign_body_degree_aspect.1;
        let body2 = sign_body_degree_aspect.2;
        BodyDegreeSignAspect::new(kind, body1, body2)
    }
}

//impl From<BodyDegreeSignAspectTuple> for BodyDegreeSignAspect {
//  fn from(sign_body_degree_aspect: BodyDegreeSignAspectTuple) -> Self {
//    let kind = sign_body_degree_aspect.0;
//    let body1: BodySignDegree = sign_body_degree_aspect.1.into();
//    let body2: BodySignDegree = sign_body_degree_aspect.2.into();
//    BodyDegreeSignAspect::new(kind, body1.into(), body2.into())
//  }
//}

//use std::fmt;
//impl fmt::Display for BodyDegreeSignAspect {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        //write!(f, "{} {} {}", self.body_position1, self.kind, self.body_position2)
//        //write!(f, "{} {} {}", self.body_position1, self.kind, self.body_position2)
//        write!(f, "{}", BodySignDegreeAspectSymbolText(&self))
//    }
//}

//use body_sign_degree::BodySignDegreeSymbolText;
//pub struct BodySignDegreeAspectSymbolText<'a>(pub &'a BodyDegreeSignAspect);
//impl <'a> fmt::Display for BodySignDegreeAspectSymbolText<'a> {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//    let body_position1 = BodySignDegreeSymbolText(&self.0.body_position1);
//    let body_position2 = BodySignDegreeSymbolText(&self.0.body_position2);
//    let kind = DegreeAspectKindSymbolText(&self.0.kind);
//        write!(f, "{} {} {}", body_position1, kind, body_position2)
//    }
//}
