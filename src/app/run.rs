use std::fs;
use std::path::{Path, PathBuf};

use crate::app::error::AppError;
use crate::config::loader::resolve_config_source;
use crate::config::model::ConfigSource;
use crate::config::parser::{parse_gitignore, parse_treeignore};
use crate::ignore::planner::plan_jobs;
use crate::matcher::compiler::compile_matcher;
use crate::render::terminal::render_tree;
use crate::tree::builder::build_tree;
use crate::fs::walker::walk_filtered;

#[derive(Debug, Clone)]
pub struct CliRequest {
    pub target_path: PathBuf,
    pub profile: Option<String>,
}

pub fn run(request: CliRequest) -> Result<String, AppError> {
    validate_target_path(&request.target_path)?;

    let config_source = resolve_config_source(&request.target_path);
    let parsed_config = load_and_parse_config(&config_source)?;
    let has_treeignore = matches!(config_source, ConfigSource::TreeIgnore(_));

    let jobs = plan_jobs(
        &parsed_config, 
        request.profile.as_deref(), 
        has_treeignore
    )?;

    let mut rendered_outputs = Vec::new();

    for job in jobs {
        let matcher = compile_matcher(&job.effective_ignore.patterns)?;
        let entries = walk_filtered(&request.target_path, &matcher)?;
        let tree = build_tree(&request.target_path, &entries);
        let rendered = render_tree(&tree)?;

        if request.profile.is_none() {
            rendered_outputs.push(format!("[{}]\n{}", job.label, rendered));
        } else {
            rendered_outputs.push(rendered);
        }
    }

    Ok(rendered_outputs.join("\n\n"))
}

fn validate_target_path(path: &Path) -> Result<(), AppError> {
    if !path.exists() {
        return Err(AppError::InvalidTargetPath(format!(
            "Target path does not exist: {}",
            path.display()
        )));
    }

    if !path.is_dir() {
        return Err(AppError::InvalidTargetPath(format!(
            "Target path is not a directory:{}",
            path.display()
        )));
    }

    Ok(())
}

fn load_and_parse_config(config_source: &ConfigSource) -> Result<crate::config::model::ParsedConfig, AppError> {
    match config_source {
        ConfigSource::TreeIgnore(path) => {
            let contents = fs::read_to_string(path)?;
            Ok(parse_treeignore(&contents)?)
        }
        ConfigSource::GitIgnore(path) => {
            let contents = fs::read_to_string(path)?;
            Ok(parse_gitignore(&contents)?)
        }
        ConfigSource::None => Ok(crate::config::model::ParsedConfig::default()),
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;

    fn create_temp_dir(prefix: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let dir = std::env::temp_dir().join(format!("{prefix}_{unique}"));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn runs_without_config() {
        let dir = create_temp_dir("treeit_runs_without_config");
        fs::create_dir_all(dir.join("src")).unwrap();
        fs::write(dir.join("src").join("main.rs"), "fn main() {}").unwrap();

        let request = CliRequest {
            target_path: dir.clone(),
            profile: None,
        };

        let output = run(request).unwrap();

        assert!(output.contains("[general]"));
        assert!(output.contains("src/"));
        assert!(output.contains("main.rs"));

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn runs_with_treeignore_and_profile() {
        let dir = create_temp_dir("treeit_runs_with_treeignore_and_profile");

        fs::create_dir_all(dir.join("src")).unwrap();
        fs::create_dir_all(dir.join("tests")).unwrap();
        fs::write(dir.join("src").join("main.rs"), "fn main() {}").unwrap();
        fs::write(
            dir.join(".treeignore"),
            r#"
tree_docs = [
    "tests/"
]
"#,
        )
        .unwrap();

        let request = CliRequest {
            target_path: dir.clone(),
            profile: None,
        };

        let output = run(request).unwrap();

        assert!(output.contains("[general]"));
        assert!(output.contains("[tree_docs]"));

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn runs_single_profile_only() {
        let dir = create_temp_dir("treeit_runs_single_profile");

        fs::create_dir_all(dir.join("src")).unwrap();
        fs::create_dir_all(dir.join("tests")).unwrap();
        fs::write(dir.join("src").join("main.rs"), "fn main() {}").unwrap();
        fs::write(
            dir.join(".treeignore"),
            r#"
tree_docs = [
    "tests/"
]
"#,
        )
        .unwrap();

        let request = CliRequest {
            target_path: dir.clone(),
            profile: Some("tree_docs".to_string()),
        };

        let output = run(request).unwrap();

        assert!(!output.contains("[general]"));
        assert!(output.contains("src/"));
        assert!(!output.contains("tests/"));

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn errors_on_invalid_profile() {
        let dir = create_temp_dir("treeit_errors_on_invalid_profile");

        fs::write(
            dir.join(".treeignore"),
            r#"
tree_docs = [
    "tests/"
]
"#,
        )
        .unwrap();

        let request = CliRequest {
            target_path: dir.clone(),
            profile: Some("missing_profile".to_string()),
        };

        let result = run(request);

        assert!(result.is_err());

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn errors_when_profile_is_requested_without_treeignore() {
        let dir = create_temp_dir("treeit_errors_profile_without_treeignore");

        let request = CliRequest {
            target_path: dir.clone(),
            profile: Some("tree_docs".to_string()),
        };

        let result = run(request);

        assert!(result.is_err());

        fs::remove_dir_all(dir).unwrap();
    }
}