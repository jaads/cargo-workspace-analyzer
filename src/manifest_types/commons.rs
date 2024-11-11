use serde::Deserialize;
use std::collections::HashMap;

/// Both, the root and the nested manifest must have a `name` attribute.
#[derive(Deserialize, Debug)]
pub struct Package {
    pub name: String,
}

/// Workspace dependency is very similar to package dependencies except.
/// They differ regarding `optional` and how features are handled.
/// The parts which are inspected current are in both cases the same.
#[derive(Deserialize, Debug)]
#[serde(untagged)]
#[allow(dead_code)] // may be used later
pub enum DependencyInfo {
    Simple(Version),
    Detailed {
        path: Option<String>,
        workspace: Option<bool>,
    },
}

pub type Dependencies = HashMap<Name, DependencyInfo>;

type Version = String;
type Name = String;
