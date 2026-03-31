use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RenderError {
    EmptyTree,
}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RenderError::EmptyTree => write!(f, "Cannot render an empty or invalid tree"),
        }
    }
}

impl std::error::Error for RenderError {}
