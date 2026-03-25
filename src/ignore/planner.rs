use std::fmt;

use crate::config::model::{ParsedConfig, ProfileConfig};
use crate::ignore::model::{EffectiveIgnoreConfig, TreeJob};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlanningError {
    ProfileRequiresTreeIgnore,
    ProfileNotFound(String),
}

impl fmt::Display for PlanningError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlanningError::ProfileRequiresTreeIgnore => {
                write!(f, "A profile can only be used when a .treeignore file exists")
            }
            PlanningError::ProfileNotFound(name) => {
                write!(f, "Profile '{name}' was not found in .treeignore")
            }
        }
    }
}

impl std::error::Error for PlanningError {}

pub fn plan_jobs(
    parsed_config: &ParsedConfig,
    selected_profile: Option<&str>,
    has_treeignore: bool,
) -> Result<Vec<TreeJob>, PlanningError> {
    match selected_profile {
        Some(profile_name) => {
            if !has_treeignore {
                return Err(PlanningError::ProfileRequiresTreeIgnore)
            }

            let normalized_name = profile_name.to_lowercase();
            let profile = parsed_config
                .profiles
                .iter()
                .find(|profile| profile.name == normalized_name)
                .ok_or_else(|| PlanningError::ProfileNotFound(normalized_name.clone()))?;

            Ok(vec![build_profile_job(parsed_config, profile)])
        }
        None => {
            let mut jobs = Vec::new();

            jobs.push(build_general_job(parsed_config));

            for profile in &parsed_config.profiles {
                jobs.push(build_profile_job(parsed_config, profile));
            }

            Ok(jobs)
        }
    }
}

fn build_general_job(parsed_config: &ParsedConfig) -> TreeJob {
    TreeJob {
        label: "general".to_string(),
        effective_ignore: EffectiveIgnoreConfig{
            patterns: parsed_config.global_excludes.clone(),
        },
    }
}

fn build_profile_job(parsed_config: &ParsedConfig, profile: &ProfileConfig) -> TreeJob {
    let mut patterns = parsed_config.global_excludes.clone();
    patterns.extend(profile.excludes.clone());

    TreeJob{
        label: profile.name.clone(),
        effective_ignore: EffectiveIgnoreConfig { patterns },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::model::{ParsedConfig, ProfileConfig};

    fn sample_config() -> ParsedConfig {
        ParsedConfig {
            global_excludes: vec!["node_modules/".to_string(), "dist/".to_string()],
            profiles: vec![
                ProfileConfig {
                    name: "tree_docs".to_string(),
                    excludes: vec!["tests/".to_string(), ".github/".to_string()],
                    tags: vec![],
                }, 
                ProfileConfig {
                    name: "tree_public".to_string(),
                    excludes: vec!["internal/".to_string()],
                    tags: vec![],
                },
            ],
            global_tags: vec![],
        }
    }

    #[test]
    fn default_execution_creates_general_and_all_profile_jobs() {
        let config = sample_config();

        let jobs = plan_jobs(&config, None, true).unwrap();

        assert_eq!(jobs.len(), 3);
        assert_eq!(jobs[0].label, "general");
        assert_eq!(jobs[1].label, "tree_docs");
        assert_eq!(jobs[2].label, "tree_public");
    }

    #[test]
    fn general_job_uses_only_global_excludes() {
        let config = sample_config();

        let jobs = plan_jobs(&config, None, true).unwrap();

        assert_eq!(
            jobs[0].effective_ignore.patterns,
            vec!["node_modules/".to_string(), "dist/".to_string()]
        );
    }

    #[test]
    fn profile_job_merges_global_and_profile_excludes() {
        let config = sample_config();

        let jobs = plan_jobs(&config, Some("tree_docs"), true).unwrap();
        
        assert_eq!(jobs.len(), 1);
        assert_eq!(jobs[0].label, "tree_docs");
        assert_eq!(
            jobs[0].effective_ignore.patterns,
            vec![
                "node_modules/".to_string(),
                "dist/".to_string(),
                "tests/".to_string(),
                ".github/".to_string(),
            ]
        );
    }

    #[test]
    fn selected_profile_is_case_sensitive() {
        let config = sample_config();

        let jobs = plan_jobs(&config, Some("TREE_DOCS"), true).unwrap();

        assert_eq!(jobs.len(), 1);
        assert_eq!(jobs[0].label, "tree_docs");
    }

    #[test]
    fn selecting_profile_without_treeignore_returns_error() {
        let config = sample_config();

        let result = plan_jobs(&config, Some("tree_docs"), false);

        assert!(matches!(
            result, 
            Err(PlanningError::ProfileRequiresTreeIgnore)
        ));
    }

    #[test]
    fn selecting_unknown_profile_returns_error() {
        let config = sample_config();

        let result = plan_jobs(&config, Some("tree_missing"), true);

        assert!(matches!(
            result,
            Err(PlanningError::ProfileNotFound(name)) if name == "tree_missing"
        ));
    }

    #[test]
    fn default_execution_without_profiles_still_creates_general_job() {
        let config = ParsedConfig {
            global_excludes: vec!["target/".to_string()],
            profiles: vec![],
            global_tags: vec![],
        };

        let jobs = plan_jobs(&config, None, false).unwrap();

        assert_eq!(jobs.len(), 1);
        assert_eq!(jobs[0].label, "general");
        assert_eq!(
            jobs[0].effective_ignore.patterns,
            vec!["target/".to_string()]
        );
    }
}