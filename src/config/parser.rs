use crate::config::error::ConfigError;
use crate::config::model::{ParsedConfig, ProfileConfig};

pub fn parse_gitignore(contents: &str) -> Result<ParsedConfig, ConfigError> {
    let mut config = ParsedConfig::default();

    for raw_line in contents.lines() {
        let line = raw_line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        config.global_excludes.push(line.to_string());
    }

    Ok(config)
}

pub fn parse_treeignore(contents: &str) -> Result<ParsedConfig, ConfigError> {
    let mut config = ParsedConfig::default();
    let lines: Vec<&str> = contents.lines().collect();
    let mut index = 0;

    while index < lines.len() {
        let line = lines[index].trim();

        if line.is_empty() || line.starts_with('#') {
            index += 1;
            continue;
        }

        // variable assignment start
        if line.contains('=') {
            let (name, values, next_index) = parse_list_assignment(&lines, index)?;

            let normalized_name = name.to_lowercase();

            if normalized_name == "tags" {
                config.global_tags = values;
            } else if normalized_name.starts_with("tree") {
                if normalized_name.ends_with("_tags") {
                    let profile_name = normalized_name.trim_end_matches("_tags").to_string();

                    let profile = config.profiles.iter_mut().find(|p| p.name == profile_name);

                    match profile {
                        Some(existing) => existing.tags = values,
                        None => {
                            config.profiles.push(ProfileConfig {
                                name: profile_name,
                                excludes: Vec::new(),
                                tags: values,
                            });
                        }
                    }
                } else {
                    let profile = config
                        .profiles
                        .iter_mut()
                        .find(|p| p.name == normalized_name);

                    match profile {
                        Some(existing) => existing.excludes = values,
                        None => {
                            config.profiles.push(ProfileConfig {
                                name: normalized_name,
                                excludes: values,
                                tags: Vec::new(),
                            });
                        }
                    }
                }
            } else {
                return Err(ConfigError::UnexpectedVariable(name));
            }

            index = next_index;
            continue;
        }

        // otherwise treat as global exclusion
        config.global_excludes.push(line.to_string());
        index += 1;
    }

    Ok(config)
}

fn parse_list_assignment(
    lines: &[&str],
    start_index: usize,
) -> Result<(String, Vec<String>, usize), ConfigError> {
    let first_line = lines[start_index].trim();

    let (raw_name, raw_value_start) = first_line
        .split_once('=')
        .ok_or_else(|| ConfigError::InvalidSyntax(first_line.to_string()))?;

    let name = raw_name.trim().to_string();
    let value_start = raw_value_start.trim();

    if value_start != "[" {
        return Err(ConfigError::InvalidSyntax(first_line.to_string()));
    }

    let mut values = Vec::new();
    let mut index = start_index + 1;

    while index < lines.len() {
        let line = lines[index].trim();

        if line.is_empty() || line.starts_with('#') {
            index += 1;
            continue;
        }

        if line == "]" {
            return Ok((name, values, index + 1));
        }

        let parsed_value = parse_list_value(line)?;
        values.push(parsed_value);

        index += 1;
    }

    Err(ConfigError::UnterminatedList(name))
}

fn parse_list_value(line: &str) -> Result<String, ConfigError> {
    let trimmed = line.trim_end_matches(',');

    if !(trimmed.starts_with('"') && trimmed.ends_with('"')) {
        return Err(ConfigError::InvalidSyntax(line.to_string()));
    }

    let inner = &trimmed[1..trimmed.len() - 1];
    Ok(inner.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_gitignore_skips_comments_and_empty_lines() {
        let input = r#"
# comment
node_modules/

dist/
"#;
        let config = parse_gitignore(input).unwrap();

        assert_eq!(
            config.global_excludes,
            vec!["node_modules/".to_string(), "dist/".to_string()]
        );

        assert!(config.profiles.is_empty());
        assert!(config.global_tags.is_empty());
    }

    #[test]
    fn parse_tree_ignore_global_excludes() {
        let input = r#"
node_modules/
dist/
"#;

        let config = parse_treeignore(input).unwrap();

        assert_eq!(
            config.global_excludes,
            vec!["node_modules/".to_string(), "dist/".to_string()]
        );
        assert!(config.profiles.is_empty());
    }

    #[test]
    fn parse_treeignore_profile_excludes() {
        let input = r#"
node_modules/

tree_docs = [
    "tests/",
    ".github/"
]
"#;

        let config = parse_treeignore(input).unwrap();

        assert_eq!(config.global_excludes, vec!["node_modules/".to_string()]);
        assert_eq!(config.profiles.len(), 1);
        assert_eq!(config.profiles[0].name, "tree_docs");
        assert_eq!(
            config.profiles[0].excludes,
            vec!["tests/".to_string(), ".github/".to_string()]
        );
    }

    #[test]
    fn parse_treeignore_normalizes_profile_names_to_lowercase() {
        let input = r#"
TREE_DOCS = [
    "tests/"
]
"#;

        let config = parse_treeignore(input).unwrap();

        assert_eq!(config.profiles.len(), 1);
        assert_eq!(config.profiles[0].name, "tree_docs");
    }

    #[test]
    fn parse_tree_ignore_global_tags() {
        let input = r#"
tags = [
    "--level=2",
    "--ascii"
]
"#;

        let config = parse_treeignore(input).unwrap();

        assert_eq!(
            config.global_tags,
            vec!["--level=2".to_string(), "--ascii".to_string()]
        );
    }

    #[test]
    fn parse_treeignore_profile_tags() {
        let input = r#"
tree_docs = [
    "tests/"
]

tree_docs_tags = [
    "--level=2"
]
"#;

        let config = parse_treeignore(input).unwrap();

        assert_eq!(config.profiles.len(), 1);
        assert_eq!(config.profiles[0].name, "tree_docs");
        assert_eq!(config.profiles[0].excludes, vec!["tests/".to_string()]);
        assert_eq!(config.profiles[0].tags, vec!["--level=2".to_string()]);
    }

    #[test]
    fn parse_treeignore_rejects_unknown_variable() {
        let input = r#"
vscode = [
    ".vscode/"
]
"#;

        let result = parse_treeignore(input);

        assert!(matches!(result, Err(ConfigError::UnexpectedVariable(_))));
    }

    #[test]
    fn parse_treeignore_rejects_unterminated_list() {
        let input = r#"
tree_docs = [
    "tests/"
"#;

        let result = parse_treeignore(input);

        assert!(matches!(result, Err(ConfigError::UnterminatedList(_))));
    }
}
