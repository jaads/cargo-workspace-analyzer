use std::path::Path;

use crate::manifests_collector::reader::load_cargo_toml_content;
use crate::types::root::{CargoRootManifest, CargoRootManifestFinding};

/// Reads the contents of `Cargo.toml` in the specified directory.
/// Panics if the file does not exist or cannot be read.
pub fn get_root_manifest(dir: &Path) -> CargoRootManifestFinding {
    let cargo_toml_path = dir.join("Cargo.toml");

    if cargo_toml_path.exists() {
        let manifest = load_cargo_toml_content::<CargoRootManifest>(&cargo_toml_path)
            .expect("Failed to parse root manifest");
        if manifest.workspace.is_none() {
            panic!("This directory doesn't seem to be a workspace. There is no workspace definition in the root manifest. ");
        }
        CargoRootManifestFinding {
            manifest,
            path: dir.to_path_buf(),
        }
    } else {
        panic!("No Cargo.toml file found in the specified directory");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::{tempdir, TempDir};

    #[test]
    fn test_read_cargo_toml_success() {
        // Create a temporary directory
        let temp_dir = TempDir::new().unwrap();

        // Write a dummy Cargo.toml file
        let cargo_toml_path = temp_dir.path().join("Cargo.toml");
        let mut file = File::create(&cargo_toml_path).expect("Failed to create Cargo.toml");
        writeln!(file, "[workspace]\nmembers = [\"package1\"]\n")
            .expect("Failed to write to Cargo.toml");

        // Test that the function reads the file correctly
        // Pass the full path to Cargo.toml to get_root_manifest
        let manifest = get_root_manifest(&temp_dir.path());

        // Check that the `package` section exists and has the expected values
        if let Some(manifest) = manifest.manifest.workspace {
            assert_eq!(manifest.members, Some(vec!["package1".to_string()]));
        }

        // Temporary directory is cleaned up automatically
    }

    #[test]
    #[should_panic]
    fn test_read_cargo_toml_not_found() {
        // Create a temporary directory without a Cargo.toml file
        let temp_dir = tempdir().expect("Failed to create temp dir");

        // This should panic because Cargo.toml does not exist in the temp directory
        get_root_manifest(temp_dir.path());
    }
}
