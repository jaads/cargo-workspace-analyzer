use crate::manifest_types::commons::{Dependencies, Package};
use serde::Deserialize;
use std::path::PathBuf;

/// The Cargo.toml file aka. the manifest from a package.
#[derive(Deserialize, Debug)]
pub struct Manifest {
    pub package: Package,
    pub dependencies: Option<Dependencies>,
}

#[derive(Debug)]
pub struct ManifestFinding {
    pub path: PathBuf,
    pub manifest: Manifest,
}

pub type ManifestFindings = Vec<ManifestFinding>;
