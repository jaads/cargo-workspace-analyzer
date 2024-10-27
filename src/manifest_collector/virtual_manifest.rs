use std::fs;
use std::path::Path;

/// Reads the contents of `Cargo.toml` in the specified directory.
/// Panics if the file does not exist or cannot be read.
pub fn get_root_manifest(dir: &Path) -> String {
    let cargo_toml_path = dir.join("Cargo.toml");

    if cargo_toml_path.exists() {
        fs::read_to_string(cargo_toml_path).expect("Failed to read Cargo.toml")
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
        let cargo_toml_path = temp_dir.path().join("Cargo.toml");

        // Write a dummy Cargo.toml file
        let mut file = File::create(&cargo_toml_path).expect("Failed to create Cargo.toml");
        writeln!(file, "[package]\nname = \"test\"\nversion = \"0.1.0\"").expect("Failed to write to Cargo.toml");

        // Test that the function reads the file correctly
        let contents = get_root_manifest(temp_dir.path());
        assert!(contents.contains("[package]"));
        assert!(contents.contains("name = \"test\""));
        assert!(contents.contains("version = \"0.1.0\""));

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
