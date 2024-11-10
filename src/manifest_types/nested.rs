use crate::manifest_types::commons::{DependencyInfo, Package};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;


type Name = String;

/// The Cargo.toml file aka. the manifest from a package.
#[derive(Deserialize, Debug)]
pub struct Manifest {
    pub package: Package,
    pub dependencies: Option<HashMap<Name, DependencyInfo>>,
}

#[derive(Debug)]
pub struct ManifestFinding {
    pub path: PathBuf,
    pub manifest: Manifest,
}

pub type ManifestFindings = Vec<ManifestFinding>;
