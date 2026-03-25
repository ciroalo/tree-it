use std::fmt;
use std::path::Path;

#[derive(Debug)]
pub enum PathError {
    NotRelativeToRoot,
    InvalidUniCode,
}

impl fmt::Display for PathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PathError::NotRelativeToRoot => write!(f, "Path is not relative to the analyzed root"),
            PathError::InvalidUniCode => write!(f, "Path contains invalid unicode"),
        }
    }
}

impl std::error::Error for PathError {}


pub fn normalize_relative_path(root: &Path, path: &Path) -> Result<String, PathError> {
    let relative = path
        .strip_prefix(root)
        .map_err(|_| PathError::NotRelativeToRoot)?;

    if relative.as_os_str().is_empty() {
        return Ok(".".to_string());
    }

    let relative_str = relative.to_str().ok_or(PathError::InvalidUniCode)?;

    Ok(relative_str.replace('\\', "/"))
}

pub fn display_name(path: &Path) -> Option<String> {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name: &str| name.to_string())
}

pub fn is_hidden(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.starts_with('.'))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn normalize_root_as_dot() {
        let root = PathBuf::from("/project");
        let path = PathBuf::from("/project");

        let result = normalize_relative_path(&root, &path).unwrap();

        assert_eq!(result, ".");
    }

    #[test]
    fn normalize_nested_path() {
        let root = PathBuf::from("/project");
        let path = PathBuf::from("/project/src/main.rs");

        let result = normalize_relative_path(&root, &path).unwrap();

        assert_eq!(result, "src/main.rs");
    }

    #[test]
    fn display_name_returns_last_component() {
        let path = PathBuf::from("/projects/src/main.rs");

        let result = display_name(&path);

        assert_eq!(result, Some("main.rs".to_string()));
    }

    #[test]
    fn hidden_file_is_detected() {
        let path = PathBuf::from(".gitignore");

        assert!(is_hidden(&path));
    }

    #[test]
    fn normal_file_is_not_hidden() {
        let path = PathBuf::from("main.rs");

        assert!(!is_hidden(&path))
    }
}