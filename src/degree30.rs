use super::*;

#[derive(Debug, Clone)]
pub struct Degree30 {
    pub degrees: f64,
    pub minutes: f64,
    pub seconds: f64,
    pub thirds: f64,  // 1/60th of second
    pub fourths: f64, // 1/60th of thirds
}

impl Degree30 {
    pub fn new(degrees: f64) -> Self {
        assert!(degrees < 30.0);
        let minutes = degrees.fract() * MINUTES_IN_DEGREE;
        let seconds = minutes.fract() * SECONDS_IN_MINUTE;
        let thirds = seconds.fract() * THIRDS_IN_SECOND;
        let fourths = thirds.fract() * FOURTHS_IN_THIRD;

        Self {
            degrees,
            minutes,
            seconds,
            thirds,
            fourths,
        }
    }

    pub fn from_i32(d: i32, m: i32, s: i32) -> Self {
        assert!(d < DEGREES_IN_SIGN as i32);
        assert!(m < MINUTES_IN_DEGREE as i32);
        assert!(s < SECONDS_IN_MINUTE as i32);
        let minute_fract = (s as f64) / SECONDS_IN_MINUTE;
        let minutes = (m as f64) + minute_fract;
        let degrees_fract = minutes / MINUTES_IN_DEGREE;
        let degrees = (d as f64) + degrees_fract;
        Self::new(degrees)
    }

    pub fn from_i32_tuple(t: (i32, i32, i32)) -> Self {
        Self::from_i32(t.0, t.1, t.2)
    }

    pub fn to_f64(&self) -> f64 {
        self.degrees
    }

    pub fn degrees(&self) -> f64 {
        self.degrees.trunc()
    }
    pub fn degrees_full(&self) -> f64 {
        self.degrees
    }

    pub fn minutes(&self) -> f64 {
        self.minutes.trunc()
    }

    pub fn minutes_full(&self) -> f64 {
        self.minutes
    }

    pub fn seconds(&self) -> f64 {
        self.seconds.trunc()
    }

    pub fn seconds_full(&self) -> f64 {
        self.seconds
    }

    pub fn degrees_rounded(&self) -> f64 {
        self.degrees.round()
    }

    pub fn minutes_rounded(&self) -> f64 {
        self.minutes.round()
    }

    pub fn seconds_rounded(&self) -> f64 {
        self.seconds.round()
    }

    pub fn nearest_degrees_minutes_seconds(&self) -> (f64, f64, f64) {
        let mut degrees = self.degrees();
        let mut minutes = self.minutes();
        let mut seconds = self.seconds_rounded();
        if seconds == SECONDS_IN_MINUTE {
            seconds = 0.0;
            minutes += 1.0
        }

        if minutes == MINUTES_IN_DEGREE {
            minutes = 0.0;
            degrees += 1.0;
        }

        if degrees == DEGREES_IN_SIGN {
            degrees = 29.0;
            minutes = 59.0;
            seconds = 59.0;
        }

        (degrees, minutes, seconds)
    }

    pub fn nearest_degrees_minutes(&self) -> (f64, f64) {
        let mut degrees = self.degrees();
        let mut minutes = self.minutes_rounded();

        if minutes == MINUTES_IN_DEGREE {
            minutes = 0.0;
            degrees += 1.0;
        }

        if degrees == DEGREES_IN_SIGN {
            degrees = 29.0;
            minutes = 59.0;
        }

        (degrees, minutes)
    }

    pub fn nearest_degrees_minutes_str(&self) -> String {
        let (degrees, minutes) = self.nearest_degrees_minutes();
        format_degrees_minutes_str(degrees, minutes)
    }

    pub fn nearest_degrees_minutes_seconds_str(&self) -> String {
        let (degrees, minutes, seconds) = self.nearest_degrees_minutes_seconds();
        format_degrees_minutes_seconds_str(degrees, minutes, seconds)
    }
}

use std::fmt;
impl fmt::Display for Degree30 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.nearest_degrees_minutes_seconds_str())
    }
}

use std::convert::From;
impl From<(i32, i32, i32)> for Degree30 {
    fn from(split_degree: (i32, i32, i32)) -> Self {
        let (degree, minute, second) = split_degree;
        Degree30::from_i32(degree, minute, second)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_i32_at_max_bounds_works() {
        //let test1 = TruncFractDegree::from_i32(29, 59, 59);
        let test1 = Degree30::from_i32(29, 59, 59);
        //    assert_eq!("29°59′59″", test1.degrees_minutes_seconds_rounded_str());
        //    assert_eq!("0°", test1.degrees_rounded_str_with_overflow().0);
        //    assert_eq!("30°0′", test1.degrees_minutes_rounded_str());
    }

    #[test]
    fn it_works2() {
        //let test1 = TruncFractDegree::from_f64(29.5557);
        //    panic!("{}", test1.degrees_minutes_seconds_rounded_str());
        //panic!("{}", test1.degrees_rounded_str())

        //let test1 = TruncFractDegree::from_f64(29.495555555555555);
        //panic!("{}", test1.degrees_rounded_str());
        //panic!("{}", test1.degrees_minutes_rounded_str());
    }
}

//  pub fn degrees_minutes_rounded_str(&self) -> String {
//    let mut degrees = self.degrees();
//    let mut minutes_rounded = self.minutes_rounded();
//
//    if minutes_rounded >= 60.0 {
//      degrees += 1.0;
//      minutes_rounded = 0.0
//    }
//
////    if degrees == 30. {
////      //minutes_rounded = f64::MAX.fract() - f64::MIN_POSITIVE.fract();
////      //minutes_rounded = 29.999999999;;
////      minutes_rounded = 29.0 + (f64::MAX / (f64::MAX - 1.0)).fract();
////
////
////      degrees = 29.;
////    }
//
//    format!("{}°{}′", degrees, minutes_rounded)
//  }
//
//  pub fn degrees_minutes_seconds_rounded_str(&self) -> String {
//    format!("{}°{}′{}″", self.degrees(), self.minutes(), self.seconds_rounded())
//  }
