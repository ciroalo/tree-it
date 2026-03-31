mod common;

use std::fs;

use common::fixture::{cleanup_temp_dir, create_temp_dir};
use tree_it::app::run::{CliRequest, run};

#[test]
fn selected_profile_generates_only_that_profile_output() {
    let dir = create_temp_dir("treeit_profile_selected");

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

    cleanup_temp_dir(&dir);
}

#[test]
fn selected_profile_is_case_insensitive() {
    let dir = create_temp_dir("treeit_profile_selected");

    fs::create_dir_all(dir.join("src")).unwrap();

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
        profile: Some("TREE_DOCS".to_string()),
    };

    let output = run(request).unwrap();

    assert!(output.contains("src/"));

    cleanup_temp_dir(&dir);
}
