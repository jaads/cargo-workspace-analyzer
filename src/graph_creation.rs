use crate::manifest_types::nested::ManifestFindings;
use crate::manifest_types::root::CargoRootManifest;
use std::collections::HashMap;

pub type Graph = HashMap<String, Vec<String>>;

/// Populates the Graph from the root manifest and nested manifests (dependencies).
pub fn get_graph_from_manifests(root: &CargoRootManifest, nested: &ManifestFindings) -> Graph {
    let mut graph: Graph = HashMap::new();

    // Add root package dependencies to the map
    if let Some(root_package) = &root.package {
        let root_name = &root_package.name;
        let mut dependencies = Vec::new();

        if let Some(root_dependencies) = &root.dependencies {
            for dep_name in root_dependencies.keys() {
                dependencies.push(dep_name.clone());
            }
        }

        graph.insert(root_name.clone(), dependencies);
    }

    // Add nested package dependencies to the map
    for manifest_finding in nested {
        let package_name = &manifest_finding.manifest.package.name;
        let mut dependencies = Vec::new();

        if let Some(package_dependencies) = &manifest_finding.manifest.dependencies {
            for dep_name in package_dependencies.keys() {
                dependencies.push(dep_name.clone());
            }
        }

        graph.insert(package_name.clone(), dependencies);
    }

    graph
}
