use assert_cmd::Command;
use assert_fs::prelude::*;

#[test]
fn test_default_workspace_dir() {
    // Create a temporary directory structure
    let temp_dir = assert_fs::TempDir::new().unwrap();

    // Create a root Cargo.toml file for the workspace
    temp_dir
        .child("Cargo.toml")
        .write_str(
            r#"
        [workspace]
        members = ["package1"]
    "#,
        )
        .unwrap();

    // Create a nested Cargo.toml file
    temp_dir.child("package1/src").create_dir_all().unwrap();
    temp_dir
        .child("package1/Cargo.toml")
        .write_str(
            r#"
        [package]
        name = "package1"
        version = "0.1.0"
        edition = "2018"
    "#,
        )
        .unwrap();

    // Run the command without specifying --workspace-dir
    Command::cargo_bin("cargo-workspace-analyzer")
        .unwrap()
        .current_dir(&temp_dir)
        .assert()
        .success();
}

#[test]
fn test_invalid_argument() {
    Command::cargo_bin("cargo-workspace-analyzer")
        .unwrap()
        .arg("--invalid-option")
        .assert()
        .failure();
    // .stderr(predicates::str::contains("unrecognized option"));
}
