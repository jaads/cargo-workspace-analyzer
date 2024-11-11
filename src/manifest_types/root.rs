use crate::manifest_types::commons::{Dependencies, Package};
use serde::Deserialize;

/// The "virtual" manifest
#[derive(Deserialize, Debug)]
pub struct CargoRootManifest {
    pub workspace: Option<Workspace>,
    pub package: Option<Package>,
    pub dependencies: Option<Dependencies>,
    pub dev_dependencies: Option<Dependencies>,
    pub build_dependencies: Option<Dependencies>,
}

#[derive(Deserialize, Debug)]
pub struct Workspace {
    pub members: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
    pub default_members: Option<Vec<String>>,
}

impl Default for CargoRootManifest {
    fn default() -> Self {
        CargoRootManifest {
            package: None,
            workspace: None,
            dependencies: None,
            dev_dependencies: None,
            build_dependencies: None,
        }
    }
}
