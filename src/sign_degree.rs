use super::*;

use serde::de;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, PartialEq, Copy, Serialize)]
pub struct SignDegree {
    // ecliptical
    pub degrees: f64,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(untagged)]
enum SignDegreeRepr {
    Degrees {
        degrees: f64,
    },
    SignAndDegree {
        sign: Sign,
        degree: f64,
        minutes: Option<f64>,
        seconds: Option<f64>,
    },
}

impl<'de> Deserialize<'de> for SignDegree {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let repr = SignDegreeRepr::deserialize(deserializer)?;

        match repr {
            SignDegreeRepr::Degrees { degrees } => Ok(SignDegree::new(degrees)),
            SignDegreeRepr::SignAndDegree {
                sign,
                degree,
                minutes,
                seconds,
            } => {
                let mut reduced = degree;
                if let Some(m) = minutes {
                    reduced += m / 60.0;
                }
                if let Some(s) = seconds {
                    reduced += s / 3600.0;
                }

                if !(0.0..DEGREES_IN_SIGN).contains(&reduced) {
                    return Err(de::Error::custom(format!(
                        "SignDegree degree must be within [0, {})",
                        DEGREES_IN_SIGN
                    )));
                }

                Ok(SignDegree::from_sign_and_degree30_f64(sign, reduced))
            }
        }
    }
}

impl SignDegree {
    pub fn forward_distance(&self, other: &SignDegree) -> f64 {
        forward_distance_sign_degree(self, other)
    }
    pub fn backward_distance(&self, other: &SignDegree) -> f64 {
        backward_distance_sign_degree(self, other)
    }

    pub const fn new_const(degrees: f64) -> Self {
        assert!(degrees >= 0.0);
        assert!(degrees < 360.0);
        Self { degrees }
    }

    pub fn new(degrees: f64) -> Self {
        Self::new_const(degrees)
    }

    //    pub fn from_f64(d: f64) -> Self {
    //        Self::new(d)
    //    }

    pub const fn from_sign_and_degree30_f64(sign: Sign, degrees: f64) -> Self {
        assert!(degrees < MAX_DEGREE);
        assert!(degrees >= 0.0);
        let sign_start_degree = sign.start_degree_f64();
        let degree360 = sign_start_degree + degrees;
        Self::new_const(degree360)
    }

    //  pub fn from_i32(d: i32, m: i32, s: i32) -> Self {}
    //  pub fn from_i32_tuple(t: (i32, i32, i32)) -> Self {}

    pub fn sign_and_degree(&self) -> (Sign, Degree30) {
        sign_and_degree30(self.degrees)
    }

    pub fn nearest_sign_and_degrees(&self) -> (Sign, f64) {
        let (sign, mut degree30) = self.sign_and_degree();
        let degrees = degree30.degrees_rounded();
        if degrees == DEGREES_IN_SIGN {
            let next_sign = sign.next();
            return (next_sign, 0.0);
        }

        (sign, degrees)
    }

    pub fn nearest_sign_and_degrees_minutes(&self) -> (Sign, f64, f64) {
        let (sign, mut degree30) = self.sign_and_degree();
        let mut degrees = degree30.degrees();
        let mut minutes = degree30.minutes_rounded();
        if minutes == MINUTES_IN_DEGREE {
            minutes = 0.0;
            degrees += 1.0
        }

        if degrees == DEGREES_IN_SIGN {
            let next_sign = sign.next();
            return (next_sign, degrees, minutes);
        }

        (sign, degrees, minutes)
    }

    pub fn nearest_sign_and_degrees_minutes_seconds(&self) -> (Sign, f64, f64, f64) {
        let (mut sign, mut degree30) = self.sign_and_degree();
        let mut degrees = degree30.degrees();
        let mut minutes = degree30.minutes();
        let mut seconds = degree30.seconds_rounded();
        if seconds == SECONDS_IN_MINUTE {
            seconds = 0.0;
            minutes += 1.0
        }

        if minutes == MINUTES_IN_DEGREE {
            minutes = 0.0;
            degrees += 1.0;
        }

        if degrees == DEGREES_IN_SIGN {
            sign = sign.next();
            degrees = 0.0;
        }

        (sign, degrees, minutes, seconds)
    }

    // This will always stay in the same sign and never tick over.
    pub fn nearest_bounded_sign_and_degrees_minutes_seconds(&self) -> (Sign, f64, f64, f64) {
        let (sign, mut degree30) = self.sign_and_degree();
        let (degrees, minutes, seconds) = degree30.nearest_degrees_minutes_seconds();
        (sign, degrees, minutes, seconds)
    }

    pub fn nearest_bounded_sign_and_degrees_minutes_seconds_str(&self) -> String {
        let (sign, degrees, minutes, seconds) =
            self.nearest_bounded_sign_and_degrees_minutes_seconds();

        format!(
            "{} {}",
            format_degrees_minutes_seconds_str(degrees, minutes, seconds),
            sign.symbol_text(),
        )
    }
}

//pub fn degree_f64_from_reduced_degree_and_sign(degree: f64, sign: Sign) -> f64{
//    //let degree_offset =
//
//    todo!()
//}

pub fn sign_number_and_reduced_degree_from_degree_f64(mut degree: f64) -> (i32, f64) {
    let mut i = 0;
    while degree >= SIGN_DEGREES {
        degree -= SIGN_DEGREES;
        i += 1;
    }

    (i, degree)
}

pub fn sign_and_degree30(degrees: f64) -> (Sign, Degree30) {
    let (sign_i, reduced_degree) = sign_number_and_reduced_degree_from_degree_f64(degrees);
    let sign = Sign::try_from(sign_i as u8).unwrap();
    let degree30 = Degree30::new(reduced_degree);
    (sign, degree30)
}

use std::fmt;
impl fmt::Display for SignDegree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.nearest_bounded_sign_and_degrees_minutes_seconds_str()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Sign::*;

    #[test]
    fn test() {
        let test1 = SignDegree::new(69.999999);
        let (sign, degrees, minutes, seconds) = test1.nearest_sign_and_degrees_minutes_seconds();
        assert_eq!(sign, Gemini);
        assert_eq!(degrees, 10.0);
        assert_eq!(minutes, 0.0);
        assert_eq!(seconds, 0.0);
    }

    #[test]
    fn test2() {
        let test1 = SignDegree::new(29.999999);
        let (sign, degrees, minutes, seconds) = test1.nearest_sign_and_degrees_minutes_seconds();
        assert_eq!(sign, Taurus);
        assert_eq!(degrees, 0.0);
        assert_eq!(minutes, 0.0);
        assert_eq!(seconds, 0.0);
    }

    #[test]
    fn test3() {
        let test1 = SignDegree::new(29.9999);
        let (sign, degrees, minutes, seconds) =
            test1.nearest_bounded_sign_and_degrees_minutes_seconds();
        assert_eq!(sign, Aries);
        assert_eq!(degrees, 29.0);
        assert_eq!(minutes, 59.0);
        assert_eq!(seconds, 59.0);
    }
}

//let test1 = TruncFractDegree::from_i32(29, 59, 59);
//let test1 = Degree360::from_i32(29, 59, 59);
//panic!("HERE{} {}", test1.degree30, test1.degree30.degrees_minutes_rounded_str());
//    assert_eq!("29°59′59″", test1.degrees_minutes_seconds_rounded_str());
//    assert_eq!("30°", test1.degrees_rounded_str());
//    assert_eq!("30°0′", test1.degrees_minutes_rounded_str());
