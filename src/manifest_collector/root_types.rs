use std::collections::HashMap;
use serde::Deserialize;

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