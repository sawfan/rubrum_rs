use crate::aspect::degree_aspect_kind::DegreeAspectKind;
use crate::aspect::degree_aspect_match_kind::find_exact_sign_degree_for_aspect_kind;
use crate::body_sign_degree::BodySignDegree;

use strum::IntoEnumIterator;

#[derive(Debug, Clone)]
pub struct BodySignDegreeExactAspectMatches {
    pub body_sign_degree: BodySignDegree,
    pub exact: Vec<crate::aspect::exact_degree_aspect::ExactDegreeAspect>,
}

impl BodySignDegreeExactAspectMatches {
    pub fn new(
        body_sign_degree: BodySignDegree,
        exact: Vec<crate::aspect::exact_degree_aspect::ExactDegreeAspect>,
    ) -> Self {
        Self {
            body_sign_degree,
            exact,
        }
    }

    pub fn from_multiple(body_sign_degrees: Vec<BodySignDegree>) -> Vec<Self> {
        body_sign_degrees
            .iter()
            .map(|body_sign_degree| {
                let sign_degree = body_sign_degree.sign_degree;
                let aspect_matches = DegreeAspectKind::iter()
                    .map(|aspect_kind| {
                        let aspect_degree =
                            find_exact_sign_degree_for_aspect_kind(&aspect_kind, &sign_degree);

                        crate::aspect::exact_degree_aspect::ExactDegreeAspect::new(
                            aspect_kind,
                            aspect_degree,
                        )
                    })
                    .collect::<Vec<crate::aspect::exact_degree_aspect::ExactDegreeAspect>>();

                BodySignDegreeExactAspectMatches::new(*body_sign_degree, aspect_matches)
            })
            .collect::<Vec<BodySignDegreeExactAspectMatches>>()
    }
}
