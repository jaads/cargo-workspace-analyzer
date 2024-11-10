use crate::manifest_types::commons::{DependencyInfo, Package};
use serde::Deserialize;
use std::collections::HashMap;

/// The "virtual" manifest
#[derive(Deserialize, Debug)]
pub struct CargoRootManifest {
    pub workspace: Option<Workspace>,
    pub package: Option<Package>,
    pub dependencies: Option<HashMap<String, DependencyInfo>>,
    pub dev_dependencies: Option<HashMap<String, DependencyInfo>>,
    pub build_dependencies: Option<HashMap<String, DependencyInfo>>,
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
            dependencies: Some(HashMap::new()),
            dev_dependencies: Some(HashMap::new()),
            build_dependencies: Some(HashMap::new()),
        }
    }
}
