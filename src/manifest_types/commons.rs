use serde::Deserialize;

/// Both, the root and the nested manifest must have a `name` attribute.
#[derive(Deserialize, Debug)]
pub struct Package {
    pub name: String,
}

impl Default for Package {
    fn default() -> Self {
        Package {
            name: "".to_string(),
        }
    }
}

/// Workspace dependency is very similar to package dependencies except.
/// They differ regarding `optional` and how features are handled.
/// The parts which are inspected current are in both cases the same.
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Dependency {
    Simple(String),
    Detailed { path: Option<String> },
}
