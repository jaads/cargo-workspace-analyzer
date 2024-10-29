use std::collections::HashMap;
use std::path::PathBuf;
use serde::Deserialize;
use crate::manifest_types::commons::{Dependency, Package};

/// The Cargo.toml file aka. the manifest from a package.
#[derive(Deserialize, Debug)]
pub struct Manifest {
    pub package: Package,
    pub dependencies: Option<HashMap<String, Dependency>>,
}

#[derive(Debug)]
pub struct ManifestFinding {
    pub path: PathBuf,
    pub manifest: Manifest,
}

pub type ManifestFindings = Vec<ManifestFinding>;