use super::*;

pub type DegreeBodySignAspect = BodyDegreeSignAspect;
pub type DegreeSignBodyAspect = BodyDegreeSignAspect;
pub type SignBodyDegreeAspect = BodyDegreeSignAspect;
pub type SignDegreeBodyAspect = BodyDegreeSignAspect;
pub type BodySignDegreeAspect = BodyDegreeSignAspect;

pub type SignBodyDegreeAspectTuple2 = (DegreeAspectKind, BodySignDegree, BodySignDegree);

pub type BodyDegreeSignAspectTuple = (
    DegreeAspectKind,
    (Sign, Body, (i32, i32, i32)),
    (Sign, Body, (i32, i32, i32)),
);

pub type TruncFractArray = [f64; 2];
