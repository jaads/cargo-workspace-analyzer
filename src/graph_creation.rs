use crate::manifest_types::nested::ManifestFindings;
use std::collections::HashMap;

pub type Graph = HashMap<String, Vec<String>>;

/// Populates the Graph from the root manifest and nested manifests (dependencies).
pub fn get_graph_from_manifests(nested: &ManifestFindings) -> Graph {
    let mut graph: Graph = HashMap::new();

    // Add nested package dependencies to the map
    for manifest_finding in nested {
        let package_name = &manifest_finding.manifest.package.name;
        let mut dependencies = Vec::new();

        if let Some(package_dependencies) = &manifest_finding.manifest.dependencies {
            for dep_name in package_dependencies.keys() {
                dependencies.push(dep_name.clone());
            }
            dependencies.sort();
        }

        graph.insert(package_name.clone(), dependencies);
    }

    graph
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest_types::commons::{DependencyInfo, Package};
    use crate::manifest_types::nested::{Manifest, ManifestFinding};
    use std::collections::HashMap;
    use std::path::PathBuf;

    /// Helper function to create a ManifestFinding for testing
    fn setup_manifest(name: &str, dependencies: Vec<&str>) -> ManifestFinding {
        let dependency_map: HashMap<String, DependencyInfo> = dependencies
            .into_iter()
            .map(|dep| (dep.to_string(), DependencyInfo::Simple("1.0".to_string())))
            .collect();
        ManifestFinding {
            path: PathBuf::from(name),
            manifest: Manifest {
                package: Package {
                    name: name.to_string(),
                },
                dependencies: if dependency_map.is_empty() {
                    None
                } else {
                    Some(dependency_map)
                },
            },
        }
    }

    #[test]
    fn test_single_package_no_dependencies() {
        // A single package with no dependencies
        let nested = vec![setup_manifest("package_a", vec![])];
        let graph = get_graph_from_manifests(&nested);
        let expected_graph: Graph = HashMap::from([("package_a".to_string(), vec![])]);
        assert_eq!(graph, expected_graph);
    }

    #[test]
    fn test_single_package_with_dependencies() {
        // A single package with dependencies
        let nested = vec![setup_manifest("package_a", vec!["package_b", "package_c"])];
        let graph = get_graph_from_manifests(&nested);
        let expected_graph: Graph = HashMap::from([(
            "package_a".to_string(),
            vec!["package_b".to_string(), "package_c".to_string()],
        )]);
        assert_eq!(graph, expected_graph);
    }

    #[test]
    fn test_multiple_packages_with_dependencies() {
        // Multiple packages with dependencies
        let nested = vec![
            setup_manifest("package_a", vec!["package_b"]),
            setup_manifest("package_b", vec!["package_c"]),
            setup_manifest("package_c", vec![]),
        ];

        let graph = get_graph_from_manifests(&nested);
        let expected_graph: Graph = HashMap::from([
            ("package_a".to_string(), vec!["package_b".to_string()]),
            ("package_b".to_string(), vec!["package_c".to_string()]),
            ("package_c".to_string(), vec![]),
        ]);

        assert_eq!(graph, expected_graph);
    }

    #[test]
    fn test_packages_with_no_mutual_dependencies() {
        // Multiple packages with no dependencies between them
        let nested = vec![
            setup_manifest("package_a", vec![]),
            setup_manifest("package_b", vec![]),
            setup_manifest("package_c", vec![]),
        ];

        let graph = get_graph_from_manifests(&nested);
        let expected_graph: Graph = HashMap::from([
            ("package_a".to_string(), vec![]),
            ("package_b".to_string(), vec![]),
            ("package_c".to_string(), vec![]),
        ]);

        assert_eq!(graph, expected_graph);
    }

    #[test]
    fn test_complex_dependency_graph() {
        // Complex graph with multiple dependencies and nested dependencies
        let nested = vec![
            setup_manifest("package_a", vec!["package_b", "package_c"]),
            setup_manifest("package_b", vec!["package_d"]),
            setup_manifest("package_c", vec!["package_d", "package_e"]),
            setup_manifest("package_d", vec![]),
            setup_manifest("package_e", vec![]),
        ];

        let graph = get_graph_from_manifests(&nested);
        let expected_graph: Graph = HashMap::from([
            (
                "package_a".to_string(),
                vec!["package_b".to_string(), "package_c".to_string()],
            ),
            ("package_b".to_string(), vec!["package_d".to_string()]),
            (
                "package_c".to_string(),
                vec!["package_d".to_string(), "package_e".to_string()],
            ),
            ("package_d".to_string(), vec![]),
            ("package_e".to_string(), vec![]),
        ]);

        assert_eq!(graph, expected_graph);
    }
}
