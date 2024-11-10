use std::collections::HashMap;
use serde::Deserialize;
use crate::manifest_types::commons::{Dependency, Package};

/// The "virtual" manifest
#[derive(Deserialize, Debug)]
pub struct CargoRootManifest {
    pub workspace: Option<Workspace>,
    pub package: Option<Package>,
    pub dependencies: Option<HashMap<String, Dependency>>,
    pub dev_dependencies: Option<HashMap<String, Dependency>>,
    pub build_dependencies: Option<HashMap<String, Dependency>>,
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

