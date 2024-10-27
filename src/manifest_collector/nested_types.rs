use std::collections::HashMap;
use std::path::PathBuf;
use serde::Deserialize;

/// The Cargo.toml file aka. the manifest from a package.
#[derive(Deserialize, Debug, PartialEq)]
pub struct Manifest {
    pub package: Package,
    pub dependencies: Option<HashMap<String, Dependency>>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Package {
    pub name: String,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum Dependency {
    Simple(String),
    Detailed { version: Option<String>, path: Option<String> },
}

#[derive(Debug, PartialEq)]
pub struct ManifestFinding {
    pub path: PathBuf,
    pub manifest: Manifest,
}