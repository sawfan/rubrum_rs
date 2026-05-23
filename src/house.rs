use super::*;
use crate::ParseKeyError;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HouseFormat {
    /// Stable storage/API key such as `"first"`.
    Key,

    /// Human-readable name such as `"First House"`.
    Name,

    /// Numeric house label such as `"1"`.
    Number,

    /// Rust enum variant name such as `"First"`.
    Debug,
}

/// Defines the classical 12 astrological houses.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum House {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
    Ninth,
    Tenth,
    Eleventh,
    Twelfth,
}

impl House {
    #[inline]
    pub const fn canonical_key(self) -> &'static str {
        match self {
            House::First => "first",
            House::Second => "second",
            House::Third => "third",
            House::Fourth => "fourth",
            House::Fifth => "fifth",
            House::Sixth => "sixth",
            House::Seventh => "seventh",
            House::Eighth => "eighth",
            House::Ninth => "ninth",
            House::Tenth => "tenth",
            House::Eleventh => "eleventh",
            House::Twelfth => "twelfth",
        }
    }

    pub fn from_canonical_key(s: &str) -> Option<Self> {
        match s {
            "first" | "house_1" | "1" => Some(House::First),
            "second" | "house_2" | "2" => Some(House::Second),
            "third" | "house_3" | "3" => Some(House::Third),
            "fourth" | "house_4" | "4" => Some(House::Fourth),
            "fifth" | "house_5" | "5" => Some(House::Fifth),
            "sixth" | "house_6" | "6" => Some(House::Sixth),
            "seventh" | "house_7" | "7" => Some(House::Seventh),
            "eighth" | "house_8" | "8" => Some(House::Eighth),
            "ninth" | "house_9" | "9" => Some(House::Ninth),
            "tenth" | "house_10" | "10" => Some(House::Tenth),
            "eleventh" | "house_11" | "11" => Some(House::Eleventh),
            "twelfth" | "house_12" | "12" => Some(House::Twelfth),
            _ => None,
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            House::First => "First House",
            House::Second => "Second House",
            House::Third => "Third House",
            House::Fourth => "Fourth House",
            House::Fifth => "Fifth House",
            House::Sixth => "Sixth House",
            House::Seventh => "Seventh House",
            House::Eighth => "Eighth House",
            House::Ninth => "Ninth House",
            House::Tenth => "Tenth House",
            House::Eleventh => "Eleventh House",
            House::Twelfth => "Twelfth House",
        }
    }

    pub fn format_house(self, fmt: HouseFormat) -> String {
        match fmt {
            HouseFormat::Key => self.canonical_key().to_owned(),
            HouseFormat::Name => self.name().to_owned(),
            HouseFormat::Number => self.to_1_based_i32().to_string(),
            HouseFormat::Debug => format!("{:?}", self),
        }
    }

    /// Converts a 1-based house number (1..=12) to a `House`.
    pub fn from_1_based_i32(n: i32) -> Option<Self> {
        match n {
            1 => Some(House::First),
            2 => Some(House::Second),
            3 => Some(House::Third),
            4 => Some(House::Fourth),
            5 => Some(House::Fifth),
            6 => Some(House::Sixth),
            7 => Some(House::Seventh),
            8 => Some(House::Eighth),
            9 => Some(House::Ninth),
            10 => Some(House::Tenth),
            11 => Some(House::Eleventh),
            12 => Some(House::Twelfth),
            _ => None,
        }
    }

    pub fn to_1_based_i32(&self) -> i32 {
        match self {
            House::First => 1,
            House::Second => 2,
            House::Third => 3,
            House::Fourth => 4,
            House::Fifth => 5,
            House::Sixth => 6,
            House::Seventh => 7,
            House::Eighth => 8,
            House::Ninth => 9,
            House::Tenth => 10,
            House::Eleventh => 11,
            House::Twelfth => 12,
        }
    }

    pub fn default_order() -> Vec<House> {
        vec![
            House::First,
            House::Second,
            House::Third,
            House::Fourth,
            House::Fifth,
            House::Sixth,
            House::Seventh,
            House::Eighth,
            House::Ninth,
            House::Tenth,
            House::Eleventh,
            House::Twelfth,
        ]
    }

    pub fn next(&self) -> House {
        match self {
            House::First => House::Second,
            House::Second => House::Third,
            House::Third => House::Fourth,
            House::Fourth => House::Fifth,
            House::Fifth => House::Sixth,
            House::Sixth => House::Seventh,
            House::Seventh => House::Eighth,
            House::Eighth => House::Ninth,
            House::Ninth => House::Tenth,
            House::Tenth => House::Eleventh,
            House::Eleventh => House::Twelfth,
            House::Twelfth => House::First,
        }
    }
}

impl std::str::FromStr for House {
    type Err = ParseKeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_canonical_key(s).ok_or_else(|| ParseKeyError::new("House", s))
    }
}

impl TryFrom<&str> for House {
    type Error = ParseKeyError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_key_round_trips_with_from_str() {
        for house in House::default_order() {
            assert_eq!(
                House::from_canonical_key(house.canonical_key()),
                Some(house)
            );
            assert_eq!(house.canonical_key().parse::<House>(), Ok(house));
        }

        assert_eq!("1".parse::<House>(), Ok(House::First));
        assert_eq!("house_12".parse::<House>(), Ok(House::Twelfth));

        let err = "not_a_house".parse::<House>().unwrap_err();
        assert_eq!(err, ParseKeyError::new("House", "not_a_house"));
    }

    #[test]
    fn format_house_supports_multiple_screen_representations() {
        assert_eq!(House::First.format_house(HouseFormat::Key), "first");
        assert_eq!(House::First.format_house(HouseFormat::Name), "First House");
        assert_eq!(House::First.format_house(HouseFormat::Number), "1");
        assert_eq!(House::First.format_house(HouseFormat::Debug), "First");
    }
}
