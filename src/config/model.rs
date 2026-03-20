use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigSource {
    TreeIgnore(PathBuf),
    GitIgnore(PathBuf),
    None,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ParsedConfig {
    pub global_excludes: Vec<String>,
    pub profiles: Vec<ProfileConfig>,
    pub global_tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ProfileConfig {
    pub name: String,
    pub excludes: Vec<String>,
    pub tags: Vec<String>,
}