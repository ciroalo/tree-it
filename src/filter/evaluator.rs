use std::path::Path;

use crate::matcher::evaluator::is_match;
use crate::matcher::pattern::CompiledMatcher;
use crate::shared::path::is_hidden;

pub fn should_include(
    path: &Path,
    relative_path: &str,
    is_dir: bool,
    matcher: &CompiledMatcher,
) -> bool {
    if is_hidden(path) {
        return false;
    }

    if is_match(matcher, relative_path, is_dir) {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::matcher::compiler::compile_matcher;

    use super::*;

    #[test]
    fn excludes_hidden_file() {
        let matcher = compile_matcher(&[]).unwrap();
        let path = PathBuf::from(".env");

        assert!(!should_include(&path, ".env", false, &matcher));
    }

    #[test]
    fn excludes_hidden_directory() {
        let matcher = compile_matcher(&[]).unwrap();
        let path = PathBuf::from(".git");

        assert!(!should_include(&path, ".git", true, &matcher));
    }

    #[test]
    fn excludes_matcher_hit() {
        let matcher = compile_matcher(&["target/".to_string()]).unwrap();
        let path = PathBuf::from("target");

        assert!(!should_include(&path, "target", true, &matcher));
    }

    #[test]
    fn includes_visible_non_matching_file() {
        let matcher = compile_matcher(&["target/".to_string()]).unwrap();
        let path = PathBuf::from("src/main.rs");

        assert!(should_include(&path, "src/main.rs", false, &matcher));
    }
}
