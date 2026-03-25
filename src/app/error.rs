use core::fmt;

use crate::config::error::ConfigError;
use crate::ignore::planner::PlanningError;
use crate::matcher::error::MatcherError;
use crate::render::error::RenderError;
use crate::shared::path::PathError;
use crate::fs::walker::WalkerError;

#[derive(Debug)]
pub enum AppError {
    InvalidArguments(String),
    InvalidTargetPath(String),
    Io(std::io::Error),
    Config(ConfigError),
    Planning(PlanningError),
    Matcher(MatcherError),
    Walker(WalkerError),
    Render(RenderError),
    Path(PathError),
}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<ConfigError> for AppError {
    fn from(value: ConfigError) -> Self {
        Self::Config(value)
    }
}

impl From<PlanningError> for AppError {
    fn from(value: PlanningError) -> Self {
        Self::Planning(value)
    }
}

impl From<MatcherError> for AppError {
    fn from(value: MatcherError) -> Self {
        Self::Matcher(value)
    }
}

impl From<WalkerError> for AppError {
    fn from(value: WalkerError) -> Self {
        Self::Walker(value)
    }
}

impl From<RenderError> for AppError {
    fn from(value: RenderError) -> Self {
        Self::Render(value)
    }
}

impl From<PathError> for AppError {
    fn from(value: PathError) -> Self {
        Self::Path(value)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::InvalidArguments(message) => write!(f, "Invalid arguments: {message}"),
            AppError::InvalidTargetPath(message) => write!(f, "{message}"),
            AppError::Io(error) => write!(f, "I/O error: {error}"),
            AppError::Config(error) => write!(f, "{error}"),
            AppError::Planning(error) => write!(f, "{error}"),
            AppError::Matcher(error) => write!(f, "{error}"),
            AppError::Walker(error) => write!(f, "{error}"),
            AppError::Render(error) => write!(f, "{error}"),
            AppError::Path(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for AppError {}