use std::path::Path;

use serde::Deserialize;
use std::collections::HashMap;
use crate::manifest_collector::manifest_reader::load_cargo_toml_content;

/// The "virtual" manifest
#[derive(Deserialize, Debug)]
pub struct CargoRootManifest {
    pub package: Option<Package>,
    pub workspace: Option<Workspace>,
    pub dependencies: Option<HashMap<String, Dependency>>,
    pub dev_dependencies: Option<HashMap<String, Dependency>>,
    pub build_dependencies: Option<HashMap<String, Dependency>>,
    pub features: Option<HashMap<String, Vec<String>>>,
}

#[derive(Deserialize, Debug)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub edition: Option<String>,
    pub authors: Option<Vec<String>>,
    pub description: Option<String>,
    pub license: Option<String>,
    pub repository: Option<String>,
    pub documentation: Option<String>,
    pub homepage: Option<String>,
    pub readme: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Workspace {
    pub members: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
    pub default_members: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Dependency {
    Simple(String),
    Detailed {
        version: Option<String>,
        path: Option<String>,
        git: Option<String>,
        branch: Option<String>,
        tag: Option<String>,
        optional: Option<bool>,
        features: Option<Vec<String>>,
        default_features: Option<bool>,
    },
}


impl Default for CargoRootManifest {
    fn default() -> Self {
        CargoRootManifest {
            package: None,
            workspace: None,
            dependencies: Some(HashMap::new()),
            dev_dependencies: Some(HashMap::new()),
            build_dependencies: Some(HashMap::new()),
            features: Some(HashMap::new()),
        }
    }
}

impl Default for Package {
    fn default() -> Self {
        Package {
            name: "".to_string(),
            version: "".to_string(),
            edition: Some(String::from("2021")),
            authors: None,
            description: None,
            license: None,
            repository: None,
            documentation: None,
            homepage: None,
            readme: None,
        }
    }
}


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
            assert_eq!(package.version, "0.1.0");
            assert_eq!(package.description, None);
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
