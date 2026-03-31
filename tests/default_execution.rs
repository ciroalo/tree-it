mod common;

use std::fs;

use common::fixture::{cleanup_temp_dir, create_temp_dir};
use tree_it::app::run::{CliRequest, run};

#[test]
fn default_execution_without_config_generates_general_tree() {
    let dir = create_temp_dir("treeit_default_no_config");

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

    cleanup_temp_dir(&dir);
}

#[test]
fn default_execution_with_treeignore_generates_general_label_and_profile_outputs() {
    let dir = create_temp_dir("treeit_default_with_config");

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

    cleanup_temp_dir(&dir);
}
