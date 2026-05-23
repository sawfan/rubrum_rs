use crate::CIRCLE_DEGREES;

/// A validated zodiac/circle longitude in the half-open range `0.0..360.0`.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Degree360(f64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Degree360Error {
    NotFinite(f64),
    OutOfRange(f64),
}

impl std::fmt::Display for Degree360Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Degree360Error::NotFinite(value) => write!(f, "degree must be finite, got {value}"),
            Degree360Error::OutOfRange(value) => {
                write!(f, "degree must be in 0..360, got {value}")
            }
        }
    }
}

impl std::error::Error for Degree360Error {}

impl Degree360 {
    pub fn new(degree: f64) -> Result<Self, Degree360Error> {
        if !degree.is_finite() {
            return Err(Degree360Error::NotFinite(degree));
        }

        if !(0.0..CIRCLE_DEGREES).contains(&degree) {
            return Err(Degree360Error::OutOfRange(degree));
        }

        Ok(Self(degree))
    }

    pub const fn get(self) -> f64 {
        self.0
    }
}

#[deprecated(note = "use Degree360 instead")]
pub type f64_360 = Degree360;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_range_without_panicking() {
        assert_eq!(Degree360::new(0.0).unwrap().get(), 0.0);
        assert_eq!(Degree360::new(359.999).unwrap().get(), 359.999);
        assert!(matches!(
            Degree360::new(360.0),
            Err(Degree360Error::OutOfRange(360.0))
        ));
        assert!(matches!(
            Degree360::new(-1.0),
            Err(Degree360Error::OutOfRange(-1.0))
        ));
    }
}
