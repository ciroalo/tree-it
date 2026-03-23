#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatcherError {
    EmptyPattern,
    InvalidPattern(String),
    RegexBuildFailed(String),
}

