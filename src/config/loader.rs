use std::path::Path;

use crate::config::model::ConfigSource;

pub fn resolve_config_source(target_dir: &Path) -> ConfigSource {
    let treeignore_path = target_dir.join(".treeignore");
    if treeignore_path.is_file() {
        return ConfigSource::TreeIgnore(treeignore_path);
    }

    let gitignore_path = target_dir.join(".gitignore");
    if gitignore_path.is_file() {
        return ConfigSource::TreeIgnore(gitignore_path);
    }

    ConfigSource::None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn create_temp_dir(prefix: &str) -> std::path::PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let dir = std::env::temp_dir().join(format!("{prefix}_{unique}"));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn resolves_treeignore_when_present() {
        let dir = create_temp_dir("tree-it_resolves_treeignore");
        fs::write(dir.join(".treeignore"), "node_modules/\n").unwrap();

        let result = resolve_config_source(&dir);

        match result {
            ConfigSource::TreeIgnore(path) => {
                assert_eq!(path, dir.join(".treeignore"));
            }
            _ => panic!("expected Treeignore"),
        }

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn resolves_gitignore_when_present() {
        let dir = create_temp_dir("tree-it_resolves_gitignore");
        fs::write(dir.join(".gitignore"), "target/\n").unwrap();

        let result = resolve_config_source(&dir);

        match result {
            ConfigSource::TreeIgnore(path) => {
                assert_eq!(path, dir.join(".gitignore"));
            }
            _ => panic!("expected Gitignore"),
        }

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn prefers_treeignore_when_both_exist() {
        let dir = create_temp_dir("tree-it_prefers_treeignore_over_gitignore");
        fs::write(dir.join(".treeignore"), "node_modules/\n").unwrap();
        fs::write(dir.join(".gitignore"), "target/\n").unwrap();

        let result = resolve_config_source(&dir);

        match result {
            ConfigSource::TreeIgnore(path) => {
                assert_eq!(path, dir.join(".treeignore"));
            }
            _ => panic!("expected Treeignore"),
        }

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn returns_none_when_no_config_exists() {
        let dir = create_temp_dir("tree-it_none_when_no_config");

        let result = resolve_config_source(&dir);

        assert!(matches!(result, ConfigSource::None));

        fs::remove_dir_all(dir).unwrap();
    }
}