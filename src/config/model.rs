use std::path::PathBuf;

enum ConfigSource {
    TreeIgnore(PathBuf),
    GitIgnore(PathBuf),
    None,
}

struct ParsedConfig {
    global_excludes: Vec<String>,
    profiles: Vec<ProfileConfig>,
    global_tags: Vec<String>,
}

struct ProfileConfig {
    name: String,
    excludes: Vec<String>,
    tags: Vec<String>,
}