use crate::matcher::pattern::CompiledMatcher;

pub fn is_match(matcher: &CompiledMatcher, relative_path: &str, is_dir: bool) -> bool {
    for pattern in &matcher.patterns {
        if pattern.directory_only && !is_dir {
            continue;
        }

        if pattern.regex.is_match(relative_path) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::matcher::compiler::compile_matcher;

    use super::*;

    #[test]
    fn first_match_wins_by_returning_true_on_first_hit() {
        let matcher = compile_matcher(&[
            "dist/".to_string(),
            "src/".to_string(),
        ])
        .unwrap();
        
        assert!(is_match(&matcher, "dist", true));
    }

    #[test]
    fn directory_only_pattern_does_not_match_file() {
        let matcher = compile_matcher(&["dist/".to_string()]).unwrap();

        assert!(!is_match(&matcher, "dist", false));
        assert!(is_match(&matcher, "dist", true));
    }

    #[test]
    fn wildcard_matches_nested_file() {
        let matcher = compile_matcher(&["*.log".to_string()]).unwrap();

        assert!(is_match(&matcher, "app.log", false));
        assert!(is_match(&matcher, "logs/app.log", false));
        assert!(!is_match(&matcher, "logs/app.txt", false));
    }

    #[test]
    fn double_star_matches_multiple_levels() {
        let matcher = compile_matcher(&["src/**/mod.rs".to_string()]).unwrap();

        assert!(is_match(&matcher, "src/a/mod.rs", false));
        assert!(is_match(&matcher, "src/a/b/mod.rs", false));
    }

    #[test]
    fn literal_pattern_matches_anywhere_in_tree() {
        let matcher = compile_matcher(&["target/".to_string()]).unwrap();

        assert!(is_match(&matcher, "target", true));
        assert!(is_match(&matcher, "backend/target", true));
    }

    #[test]
    fn returns_false_when_no_patterns_match() {
        let matcher = compile_matcher(&["dist/".to_string()]).unwrap();

        assert!(!is_match(&matcher, "src/main.rs", false));
    }

    #[test]
    fn root_anchored_pattern_matches_only_at_root() {
        let matcher = compile_matcher(&["/target".to_string()]).unwrap();

        assert!(is_match(&matcher, "target", true));
        assert!(!is_match(&matcher, "backend/target", true));
    }

    #[test]
    fn root_anchored_directory_pattern_matches_only_root_directory() {
        let matcher = compile_matcher(&["/target/".to_string()]).unwrap();

        assert!(is_match(&matcher, "target", true));
        assert!(!is_match(&matcher, "backend/target", true));
    }
}