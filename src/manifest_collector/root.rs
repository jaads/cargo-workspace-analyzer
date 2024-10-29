use std::path::Path;

use crate::manifest_collector::reader::load_cargo_toml_content;
use crate::manifest_types::root::CargoRootManifest;

/// Reads the contents of `Cargo.toml` in the specified directory.
/// Panics if the file does not exist or cannot be read.
pub fn get_root_manifest(dir: &Path) -> CargoRootManifest {
    let cargo_toml_path = dir.join("Cargo.toml");

    if cargo_toml_path.exists() {
        load_cargo_toml_content::<CargoRootManifest>(&cargo_toml_path)
            .expect("Failed to parse Cargo.toml")
    } else {
        panic!("Cargo.toml file not found in the specified directory");
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::{tempdir, TempDir};
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_read_cargo_toml_success() {
        // Create a temporary directory
        let temp_dir = TempDir::new().unwrap();

        // Write a dummy Cargo.toml file
        let cargo_toml_path = temp_dir.path().join("Cargo.toml");
        let mut file = File::create(&cargo_toml_path).expect("Failed to create Cargo.toml");
        writeln!(file, "[package]\nname = \"test\"\nversion = \"0.1.0\"").expect("Failed to write to Cargo.toml");


        // Test that the function reads the file correctly
        // Pass the full path to Cargo.toml to get_root_manifest
        let manifest = get_root_manifest(&temp_dir.path());

        // Check that the `package` section exists and has the expected values
        if let Some(package) = manifest.package {
            assert_eq!(package.name, "test");
        } else {
            panic!("Expected package section in the manifest");
        }

        // Temporary directory is cleaned up automatically
    }

    #[test]
    #[should_panic(expected = "Cargo.toml file not found in the specified directory")]
    fn test_read_cargo_toml_not_found() {
        // Create a temporary directory without a Cargo.toml file
        let temp_dir = tempdir().expect("Failed to create temp dir");

        // This should panic because Cargo.toml does not exist in the temp directory
        get_root_manifest(temp_dir.path());
    }
}
