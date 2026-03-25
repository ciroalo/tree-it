mod common;

use std::fs;

use common::fixture::{cleanup_temp_dir, create_temp_dir};
use tree_it::app::run::{run, CliRequest};

#[test]
fn gitignore_is_used_when_treeignore_is_missing() {
    let dir = create_temp_dir("treeit_gitignore_fallback");

    fs::create_dir_all(dir.join("src")).unwrap();
    fs::create_dir_all(dir.join("target")).unwrap();
    fs::write(dir.join("src").join("main.rs"), "fn main() {}").unwrap();
    fs::write(dir.join("target").join("app.o"), "binary").unwrap();

    fs::write(dir.join(".gitignore"), "target/\n").unwrap();

    let request = CliRequest {
        target_path: dir.clone(),
        profile: None,
    };

    let output = run(request).unwrap();

    assert!(output.contains("[general]"));
    assert!(output.contains("src/"));
    assert!(!output.contains("target"));

    cleanup_temp_dir(&dir);
}

#[test]
fn treeignore_takes_priority_over_gitignore() {
    let dir = create_temp_dir("treeit_treeignore_over_gitignore");

    fs::create_dir_all(dir.join("src")).unwrap();
    fs::create_dir_all(dir.join("target")).unwrap();
    fs::create_dir_all(dir.join("tests")).unwrap();

    fs::write(dir.join("src").join("main.rs"), "fn main() {}").unwrap();
    fs::write(dir.join("target").join("app.o"), "binary").unwrap();

    fs::write(dir.join(".gitignore"), "target/\n").unwrap();
    fs::write(
        dir.join(".treeignore"),
        r#"
tests/
"#,
    )
    .unwrap();

    let request = CliRequest {
        target_path: dir.clone(),
        profile: None,
    };

    let output = run(request).unwrap();

    assert!(output.contains("[general]"));
    assert!(output.contains("src"));
    assert!(output.contains("target"));
    assert!(!output.contains("tests"));

    cleanup_temp_dir(&dir);
}