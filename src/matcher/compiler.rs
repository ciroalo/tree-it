use regex::Regex;

use crate::matcher::error::MatcherError;
use crate::matcher::pattern::{CompiledMatcher, CompiledPattern};

pub fn compile_matcher(patterns: &[String]) -> Result< CompiledMatcher, MatcherError> {
    let mut compiled_patterns = Vec::new();

    for pattern in patterns {
        let compiled = compile_pattern(pattern)?;
        compiled_patterns.push(compiled);
    }

    Ok(CompiledMatcher {
        patterns: compiled_patterns,
    })
}

fn compile_pattern(pattern: &str) -> Result<CompiledPattern, MatcherError> {
    let trimmed = pattern.trim();

    if trimmed.is_empty() {
        return Err(MatcherError::EmptyPattern);
    }

    if trimmed.starts_with('!') {
        return Err(MatcherError::InvalidPattern(trimmed.to_string()));
    }

    let directory_only = trimmed.ends_with('/');
    let root_anchored = trimmed.starts_with('/');

    let normalized = trimmed
        .trim_start_matches('/')
        .trim_end_matches('/');

    if normalized.is_empty() {
        return Err(MatcherError::InvalidPattern(trimmed.to_string()));
    }

    let regex_string = build_regex_pattern(normalized, root_anchored);
    let regex = Regex::new(&regex_string)
        .map_err(|_| MatcherError::RegexBuildFailed(trimmed.to_string()))?;

    Ok(CompiledPattern {
        raw: trimmed.to_string(),
        directory_only,
        regex,
    })
}

fn build_regex_pattern(pattern: &str, root_anchored: bool) -> String {
    let mut regex = if root_anchored {
        String::from("^")
    } else {
        String::from("(^|.*/)")
    };

    let chars: Vec<char> = pattern.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            '*' => {
                if i + 1 < chars.len() && chars[i + 1] == '*' {
                    regex.push_str(".*");
                    i += 2;
                } else {
                    regex.push_str("[^/]*");
                    i += 1;
                }
            }
            '?' => {
                regex.push_str("[^/]");
                i += 1;
            }
            '.' => {
                regex.push_str("\\.");
                i += 1;
            }
            '+' | '(' | ')' | '|' | '^' | '$' | '{' | '}' | '[' | ']' | '\\' => {
                regex.push('\\');
                regex.push(chars[i]);
                i += 1;
            }
            '/' => {
                regex.push('/');
                i += 1;
            }
            other => {
                regex.push(other);
                i += 1;
            }
        }
    }

    regex.push('$');
    regex
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compiles_literal_pattern() {
        let compiled = compile_pattern("node_modules/").unwrap();

        assert_eq!(compiled.raw, "node_modules/");
        assert!(compiled.directory_only);
    }

    #[test]
    fn rejects_empty_pattern() {
        let result = compile_pattern("");

        assert!(matches!(result, Err(MatcherError::EmptyPattern)));
    }

    #[test]
    fn rejects_negation_pattern() {
        let result = compile_pattern("!Cargo.lock");

        assert!(matches!(result, Err(MatcherError::InvalidPattern(_))));
    }

    #[test]
    fn compiles_star_wildcard() {
        let compiled = compile_pattern("*.log").unwrap();

        assert!(compiled.regex.is_match("app.log"));
        assert!(compiled.regex.is_match("logs/app.log"));
        assert!(!compiled.regex.is_match("logs/app.log.bak"));
    }

    #[test]
    fn compiles_question_mark_wildcard() {
        let compiled = compile_pattern("file?.txt").unwrap();

        assert!(compiled.regex.is_match("file1.txt"));
        assert!(compiled.regex.is_match("dir/filea.txt"));
        assert!(!compiled.regex.is_match("file10.txt"));
    }

    #[test]
    fn compiles_double_star_wildcard() {
        let compiled = compile_pattern("src/**/mod.rs").unwrap();

        assert!(compiled.regex.is_match("src/foo/mod.rs"));
        assert!(compiled.regex.is_match("src/foo/bar/mod.rs"));
    }

    #[test]
    fn compiles_root_anchored_pattern() {
        let compiled = compile_pattern("/target").unwrap();

        assert!(compiled.regex.is_match("target"));
        assert!(!compiled.regex.is_match("backend/target"));
    }

    #[test]
    fn compiles_root_anchored_directory_pattern() {
        let compiled = compile_pattern("/target/").unwrap();

        assert!(compiled.directory_only);
        assert!(compiled.regex.is_match("target"));
        assert!(!compiled.regex.is_match("backend/target"));
    }
}