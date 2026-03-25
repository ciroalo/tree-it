mod common;

use std::path::PathBuf;
use std::fs;

use common::fixture::{cleanup_temp_dir, create_temp_dir};
use tree_it::app::run::{run, CliRequest};

#[test]
fn invalid_profile_returns_error() {
    let dir = create_temp_dir("tree_it_invalid_profile");

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
        profile: Some("missing".to_string()),
    };

    let result = run(request);

    assert!(result.is_err());

    cleanup_temp_dir(&dir);
}

#[test]
fn profile_without_treeignore_returns_error() {
    let dir = create_temp_dir("tree_it_profile_requires_treeignore");

    let request = CliRequest {
        target_path: dir.clone(),
        profile: Some("missing".to_string()),
    };

    let result = run(request);

    assert!(result.is_err());

    cleanup_temp_dir(&dir);
}

#[test]
fn invalid_path_returns_error() {
    let request = CliRequest {
        target_path: PathBuf::from("/definitely/not/a/real/tree_it/path"),
        profile: None,
    };

    let result = run(request);

    assert!(result.is_err());
}