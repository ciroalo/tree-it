use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigError {
    Io(String),
    InvalidSyntax(String),
    InvalidProfileName(String),
    UnexpectedVariable(String),
    UnterminatedList(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Io(message) => write!(f, "Configuration I/O error: {message}"),
            ConfigError::InvalidSyntax(line) => {
                write!(f, "Invalid configuration syntax near: {line}")
            }
            ConfigError::InvalidProfileName(name) => write!(f, "Invalid profile name: {name}"),
            ConfigError::UnexpectedVariable(name) => {
                write!(f, "Unexpected configuration variable: {name}")
            }
            ConfigError::UnterminatedList(name) => {
                write!(f, "Unterminated list for variable {name}")
            }
        }
    }
}

impl std::error::Error for ConfigError {}
