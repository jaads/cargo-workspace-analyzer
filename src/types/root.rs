use crate::types::commons::Dependencies;
use serde::Deserialize;

/// The "virtual" manifest
#[derive(Deserialize, Debug)]
pub struct CargoRootManifest {
    pub workspace: Option<Workspace>,
    #[allow(dead_code)] // may be used later
    pub dependencies: Option<Dependencies>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)] // may be used later
pub struct Workspace {
    pub members: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
    pub default_members: Option<Vec<String>>,
}

impl Default for CargoRootManifest {
    fn default() -> Self {
        CargoRootManifest {
            workspace: None,
            dependencies: None,
        }
    }
}
