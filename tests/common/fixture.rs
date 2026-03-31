use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn create_temp_dir(prefix: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    let dir = std::env::temp_dir().join(format!("{prefix}_{unique}"));
    fs::create_dir_all(&dir).unwrap();
    dir
}

pub fn cleanup_temp_dir(dir: &PathBuf) {
    if dir.exists() {
        fs::remove_dir_all(dir).unwrap();
    }
}
