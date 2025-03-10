use assert_cmd::Command;
use assert_fs::prelude::*;
use assert_fs::TempDir;

fn create_tmp_workspace() -> TempDir {
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

    temp_dir
}

#[test]
fn test_defaults() {
    let temp_dir = create_tmp_workspace();

    // Run the command without specifying anything
    Command::cargo_bin("cargo-workspace-analyzer")
        .unwrap()
        .current_dir(&temp_dir)
        .assert()
        .success();

    let output_file = temp_dir.child("workspace-analyzer.svg");
    assert!(output_file.exists(), "Expected output file was not created");
}

#[test]
fn test_default_workspace_dir() {
    let temp_dir = create_tmp_workspace();

    Command::cargo_bin("cargo-workspace-analyzer")
        .unwrap()
        .arg("-o")
        .arg("mmd")
        .current_dir(&temp_dir)
        .assert()
        .success();

    let output_file = temp_dir.child("workspace-analyzer.mmd");
    assert!(output_file.exists(), "Expected output file was not created");
}

#[test]
fn test_invalid_argument() {
    Command::cargo_bin("cargo-workspace-analyzer")
        .unwrap()
        .arg("--invalid-option")
        .assert()
        .failure();
}
