use super::*;
use DegreeAspectKind::*;

// TODO:
// * Look up exact degree2 given degree1 and aspect
// * Look up true/false if degree1 fits degree2, given tolerance
//
#[derive(Debug, Clone)]
pub enum Degree360AspectMatchKind {
    Single(f64),
    Double([f64; 2]),
}

#[derive(Debug, Clone)]
pub enum SignDegreeAspectMatchKind {
    Single(SignDegree),
    Double([SignDegree; 2]),
}

use std::fmt;
impl fmt::Display for SignDegreeAspectMatchKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Single(sign_degree) => {
                write!(f, "{}", sign_degree)
            }
            Self::Double(sign_degrees) => {
                write!(f, "{} {}", sign_degrees[0], sign_degrees[1])
            }
        }
    }
}

pub fn find_exact_sign_degree_for_aspect_kind(
    aspect: &DegreeAspectKind,
    degree: &SignDegree,
) -> SignDegreeAspectMatchKind {
    match find_exact_degrees_for_aspect_kind(aspect, degree) {
        Degree360AspectMatchKind::Single(degrees) => {
            SignDegreeAspectMatchKind::Single(SignDegree::new(degrees))
        }
        Degree360AspectMatchKind::Double(degrees) => {
            let l = SignDegree::new(degrees[0]);
            let r = SignDegree::new(degrees[1]);
            SignDegreeAspectMatchKind::Double([l, r])
        }
    }
}

pub fn find_exact_degrees_for_aspect_kind(
    aspect: &DegreeAspectKind,
    degree: &SignDegree,
) -> Degree360AspectMatchKind {
    let aspect_degrees = aspect.aspect_kind_degree_f64();
    let mut r = degree.degrees + aspect_degrees; // Always positive
    r = r.rem_euclid(CIRCLE_DEGREES); // Could be larger than 360

    if *aspect == Conjunction
        || *aspect == Opposition
        || *aspect == LunarEclipse
        || *aspect == Occultation
    {
        return Degree360AspectMatchKind::Single(r);
    }

    let mut l = degree.degrees - aspect_degrees; // Could be negative, but doesn't have to be
    if l < 0.0 {
        l += CIRCLE_DEGREES; // Add a negative number. Same as // l = CIRCLE_DEGREES - l.abs()
    }
    assert!(
        l >= 0.0,
        "Should never be negative, will wrap around to 360 opposite direction"
    );
    assert!(l < 360.0, "Should never be greater, will wrap around to 0");

    Degree360AspectMatchKind::Double([l, r])
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;
    use Sign::*;

    #[test]
    fn it_works() {
        let thema_mundi = THEMA_MUNDI_BODY_SIGN_DEGREES
            .iter()
            .map(|body_sign_degree| {
                let sign_degree = body_sign_degree.sign_degree;
                let aspect_matches = DegreeAspectKind::iter()
                    .map(|aspect_kind| {
                        let aspect_degree =
                            find_exact_sign_degree_for_aspect_kind(&aspect_kind, &sign_degree);

                        (aspect_kind, aspect_degree)
                    })
                    .collect::<Vec<(DegreeAspectKind, SignDegreeAspectMatchKind)>>();

                (*body_sign_degree, aspect_matches)
            })
            .collect::<Vec<(
                BodySignDegree,
                Vec<(DegreeAspectKind, SignDegreeAspectMatchKind)>,
            )>>();

        //panic!("{:#?}", thema_mundi)
    }
}

//    let a = DegreeAspectKind::iter().map(|x| {
//      let sign_degree = SignDegree::from_sign_and_degree30_f64(Scorpio, 15.0);
//      let aspect_degree = find_exact_sign_degree_for_aspect_kind(&x, sign_degree);
//      (x, aspect_degree)
//    }).collect::<Vec<(DegreeAspectKind, SignDegreeAspectMatchKind)>>();

//  (body, DegreeAspectKind::iter().map(|x| {
//    //let sign_degree = SignDegree::from_sign_and_degree30_f64(Scorpio, 15.0);
//    let sign_degree = body;
//    let aspect_degree = find_exact_sign_degree_for_aspect_kind(&x, sign_degree.clone());
//    (x, aspect_degree)
//  }).collect::<Vec<(DegreeAspectKind, SignDegreeAspectMatchKind)>>())
//});

//panic!("{:#?}", thema_mundi);

//    let s = a.iter().map(|(a, d)| {
//      format!("{} {}", a, d)
//    }).collect::<Vec<String>>();
//
//    panic!("{:#?}", s);

//let degree = Degree30::from_f64(180.0);
//let result = find_exact_degrees_for_aspect_kind(&Opposition, degree);
