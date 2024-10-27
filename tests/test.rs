use assert_cmd::Command;
use assert_fs::prelude::*;

#[test]
fn test_default_workspace_dir() {
    // Create a temporary directory structure
    let temp_dir = assert_fs::TempDir::new().unwrap();
    temp_dir.child("package1/src").create_dir_all().unwrap();
    temp_dir.child("package1/Cargo.toml").touch().unwrap();

    // Run the command without specifying --workspace-dir
    Command::cargo_bin("cargo-workspace-analyzer") // replace with your actual binary name
        .unwrap()
        .current_dir(&temp_dir) // run from within the temporary directory
        .assert()
        .success()
        .stdout(predicates::str::contains("Number of packages in workspace: 1"));
}

#[test]
fn test_custom_workspace_dir() {
    // Create a temporary directory structure
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let custom_dir = temp_dir.child("custom_workspace");
    custom_dir.child("package1/src").create_dir_all().unwrap();
    custom_dir.child("package1/Cargo.toml").touch().unwrap();

    // Run the command with --workspace-dir set to the custom directory
    Command::cargo_bin("cargo-workspace-analyzer") // replace with your actual binary name
        .unwrap()
        .arg("--workspace-dir")
        .arg(custom_dir.path())
        .assert()
        .success()
        .stdout(predicates::str::contains("Number of packages in workspace: 1"));
}

#[test]
fn test_invalid_argument() {
    Command::cargo_bin("cargo-workspace-analyzer") // replace with your actual binary name
        .unwrap()
        .arg("--invalid-option")
        .assert()
        .failure();
    // .stderr(predicates::str::contains("unrecognized option"));
}
