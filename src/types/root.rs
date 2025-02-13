use crate::types::commons::Dependencies;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct CargoRootManifestFinding {
    pub path: PathBuf,
    pub manifest: CargoRootManifest,
}

/// The "virtual" manifest
#[derive(Deserialize, Debug)]
pub struct CargoRootManifest {
    pub workspace: Option<Workspace>,
    #[allow(dead_code)] // may be used later
    pub dependencies: Option<Dependencies>,
}

#[derive(Deserialize, Debug, Default)]
#[allow(dead_code)] // may be used later
pub struct Workspace {
    pub members: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
    pub default_members: Option<Vec<String>>,
}
