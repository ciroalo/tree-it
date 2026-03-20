

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigError {
    Io(String),
    InvalidSyntax(String),
    InvalidProfileName(String),
    UnexpectedVariable(String),
    UnterminatedList(String),
}