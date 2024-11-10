use walkdir::WalkDir;

pub fn count_packages(workspace_dir: &str) -> usize {
    let mut package_count = 0;

    for entry in WalkDir::new(workspace_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        if path.is_dir() {
            // Check for Cargo.toml and src folder
            let cargo_toml = path.join("Cargo.toml");
            let src_dir = path.join("src");

            if cargo_toml.exists() && src_dir.exists() && src_dir.is_dir() {
                package_count += 1;
            }
        }
    }

    package_count
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    use tempfile::tempdir;

    #[test]
    fn test_count_rust_packages() {
        // Create a temporary directory for our test workspace
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let workspace_path = temp_dir.path();

        // Helper function to create a package directory with Cargo.toml and src folder
        fn create_package(path: &Path) {
            let src_dir = path.join("src");
            fs::create_dir_all(&src_dir).expect("Failed to create src directory");
            fs::write(
                path.join("Cargo.toml"),
                b"[package]\nname = \"test_package\"",
            )
            .expect("Failed to create Cargo.toml");
        }

        // Set up test directories
        let package1 = workspace_path.join("package1");
        create_package(&package1);

        let package2 = workspace_path.join("package2");
        create_package(&package2);

        let package3 = workspace_path.join("lib/nested-package");
        create_package(&package3);

        // Create a directory without src or Cargo.toml (not a package)
        let non_package = workspace_path.join("non_package");
        fs::create_dir_all(&non_package).expect("Failed to create non-package directory");

        // Create a directory with src but no Cargo.toml (not a package)
        let src_only = workspace_path.join("src_only");
        fs::create_dir_all(&src_only.join("src")).expect("Failed to create src_only directory");

        // Run the test
        let package_count = count_packages(workspace_path.to_str().unwrap());
        assert_eq!(package_count, 3);
    }
}
