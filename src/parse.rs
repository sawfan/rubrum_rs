use std::fmt;

/// Error returned when parsing a public enum from a canonical key fails.
///
/// Canonical keys are stable, storage/API-friendly identifiers such as `"sun"`,
/// `"ascendant"`, or `"aries"`. They are distinct from presentation-oriented
/// symbols or localized display text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseKeyError {
    type_name: &'static str,
    key: String,
}

impl ParseKeyError {
    pub fn new(type_name: &'static str, key: impl Into<String>) -> Self {
        Self {
            type_name,
            key: key.into(),
        }
    }

    pub const fn type_name(&self) -> &'static str {
        self.type_name
    }

    pub fn key(&self) -> &str {
        &self.key
    }
}

impl fmt::Display for ParseKeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown {} key: {}", self.type_name, self.key)
    }
}

impl std::error::Error for ParseKeyError {}
