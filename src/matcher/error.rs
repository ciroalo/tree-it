use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatcherError {
    EmptyPattern,
    InvalidPattern(String),
    RegexBuildFailed(String),
}

impl fmt::Display for MatcherError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MatcherError::EmptyPattern => write!(f, "Encountered an empty ignored pattern"),
            MatcherError::InvalidPattern(pattern) => {
                write!(f, "Invalid ignored pattern: {pattern}")
            }
            MatcherError::RegexBuildFailed(pattern) => {
                write!(f, "Failed to compile ignore pattern: {pattern}")
            }
        }
    }
}

impl std::error::Error for MatcherError {}
