use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, PartialEq)]
pub struct Manifest {
    path: PathBuf,
    content: String,
}

pub fn collect_manifests(workspace_dir: &Path) -> Vec<Manifest> {
    let mut packages = Vec::new();

    for entry in WalkDir::new(workspace_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.is_dir() {
            // Check for Cargo.toml and src folder
            let cargo_toml_path = path.join("Cargo.toml");
            let src_dir = path.join("src");

            if cargo_toml_path.exists() && src_dir.exists() && src_dir.is_dir() {
                // Read the content of Cargo.toml
                if let Ok(content) = fs::read_to_string(&cargo_toml_path) {
                    packages.push(Manifest {
                        path: path.to_path_buf(),
                        content,
                    });
                }
            }
        }
    }

    packages
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::prelude::*;
    use assert_fs::TempDir;

    #[test]
    fn test_collect_packages_with_valid_packages() {
        // Create a temporary directory structure for the workspace
        let temp_dir = TempDir::new().unwrap();

        // Set up a valid package with Cargo.toml and src directory
        let package1 = temp_dir.child("package1");
        package1.child("src").create_dir_all().unwrap();
        package1.child("Cargo.toml").write_str("[package]\nname = \"package1\"").unwrap();

        // Another valid package
        let package2 = temp_dir.child("package2");
        package2.child("src").create_dir_all().unwrap();
        package2.child("Cargo.toml").write_str("[package]\nname = \"package2\"").unwrap();

        // A package which is nested in a directory
        let package3 = temp_dir.child("lib/package3");
        package3.child("src").create_dir_all().unwrap();
        package3.child("Cargo.toml").write_str("[package]\nname = \"package3\"").unwrap();

        // Run the function and declare it as mutable for sorting
        let mut packages = collect_manifests(temp_dir.path());

        // Create a vector of expected Package instances and make it mutable
        let mut expected_packages = vec![
            Manifest {
                path: package1.path().to_path_buf(),
                content: "[package]\nname = \"package1\"".to_string(),
            },
            Manifest {
                path: package2.path().to_path_buf(),
                content: "[package]\nname = \"package2\"".to_string(),
            },
            Manifest {
                path: package3.path().to_path_buf(),
                content: "[package]\nname = \"package3\"".to_string(),
            },
        ];

        // Sort both vectors by path to ensure consistent ordering
        packages.sort_by(|a, b| a.path.cmp(&b.path));
        expected_packages.sort_by(|a, b| a.path.cmp(&b.path));

        // Assert that the collected packages match the expected packages
        assert_eq!(packages, expected_packages);
    }

    #[test]
    fn test_ignore_non_package_directories() {
        // Create a temporary directory structure for the workspace
        let temp_dir = TempDir::new().unwrap();

        // Directory without src or Cargo.toml (should be ignored)
        let non_package = temp_dir.child("non_package");
        non_package.create_dir_all().unwrap();

        // Directory with src but no Cargo.toml (should also be ignored)
        let src_only = temp_dir.child("src_only");
        src_only.child("src").create_dir_all().unwrap();

        // Incomplete package (missing src directory)
        let incomplete_package = temp_dir.child("incomplete_package");
        incomplete_package.child("Cargo.toml").write_str("[package]\nname = \"incomplete_package\"").unwrap();

        // Run the function
        let packages = collect_manifests(temp_dir.path());

        // Assert that no packages are detected
        assert_eq!(packages.len(), 0);
    }
}
