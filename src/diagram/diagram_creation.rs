use crate::manifest_types::nested::ManifestFindings;
use crate::manifest_types::root::CargoRootManifest;
use std::collections::HashMap;

// Function to generate the component diagram in Mermaid format
pub fn generate_dependency_diagram(root: CargoRootManifest, nested: ManifestFindings) -> String {
    let mut diagram = String::from("graph TD\n");

    // Create a map from package names to their dependencies
    let mut dependency_map: HashMap<String, Vec<String>> = HashMap::new();

    // Add root package dependencies to the map
    if let Some(root_package) = &root.package {
        let root_name = &root_package.name;
        let mut dependencies = Vec::new();

        if let Some(root_dependencies) = &root.dependencies {
            for dep_name in root_dependencies.keys() {
                dependencies.push(dep_name.clone());
            }
        }

        dependency_map.insert(root_name.clone(), dependencies);
    }

    // Add nested package dependencies to the map
    for manifest_finding in &nested {
        let package_name = &manifest_finding.manifest.package.name;
        let mut dependencies = Vec::new();

        if let Some(package_dependencies) = &manifest_finding.manifest.dependencies {
            for dep_name in package_dependencies.keys() {
                dependencies.push(dep_name.clone());
            }
        }

        dependency_map.insert(package_name.clone(), dependencies);
    }

    // Sort package names, prioritizing the root package if it exists
    let mut package_names: Vec<&String> = dependency_map.keys().collect();
    package_names.sort();

    // Set to track packages that are already referenced in edges
    let mut referenced_packages = std::collections::HashSet::new();

    // Generate edges, prioritizing the root package
    if let Some(root_package) = &root.package {
        if let Some(deps) = dependency_map.get(&root_package.name) {
            let mut sorted_deps = deps.clone();
            sorted_deps.sort(); // Sort dependencies to ensure consistent output
            for dep in sorted_deps {
                if dependency_map.contains_key(&dep) {
                    diagram.push_str(&format!("    {} --> {}\n", root_package.name, dep));
                    referenced_packages.insert(dep.clone());
                }
            }
        }
    }

    // Generate edges for other packages in sorted order
    for pkg in package_names.iter() {
        if Some(*pkg) != root.package.as_ref().map(|p| &p.name) {
            if let Some(deps) = dependency_map.get(*pkg) {
                for dep in deps {
                    if dependency_map.contains_key(dep) {
                        diagram.push_str(&format!("    {} --> {}\n", pkg, dep));
                        referenced_packages.insert(dep.clone());
                    }
                }
            }
        }
    }

    // Add standalone nodes for packages with no dependencies and not referenced in any edges
    for pkg in package_names.iter() {
        if let Some(deps) = dependency_map.get(*pkg) {
            if deps.is_empty() && !referenced_packages.contains(*pkg) {
                diagram.push_str(&format!("    {}\n", pkg));
            }
        }
    }

    // Ensure the root package is shown if no nodes or edges were added
    if diagram == "graph TD\n" {
        if let Some(root_package) = &root.package {
            diagram.push_str(&format!("    {}\n", root_package.name));
        }
    }

    diagram
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest_types::commons::{DependencyInfo, Package};
    use crate::manifest_types::nested::{Manifest, ManifestFinding};
    use std::path::PathBuf;

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
        // Single package, no dependencies
        let root_manifest = CargoRootManifest {
            package: Some(Package {
                name: "root_package".to_string(),
            }),
            workspace: None,
            dependencies: None,
            dev_dependencies: None,
            build_dependencies: None,
        };

        let nested = vec![];

        let diagram = generate_dependency_diagram(root_manifest, nested);
        let expected = "graph TD\n    root_package\n";
        assert_eq!(diagram, expected);
    }

    #[test]
    fn test_linear_dependencies() {
        // root_package --> package_a --> package_b
        let root_manifest = CargoRootManifest {
            package: Some(Package {
                name: "root_package".to_string(),
            }),
            dependencies: Some(
                [(
                    "package_a".to_string(),
                    DependencyInfo::Simple("1.0".to_string()),
                )]
                .into_iter()
                .collect(),
            ),
            ..Default::default()
        };

        let nested = vec![
            setup_manifest("package_a", vec!["package_b"]),
            setup_manifest("package_b", vec![]),
        ];

        let diagram = generate_dependency_diagram(root_manifest, nested);
        let expected = "graph TD\n    root_package --> package_a\n    package_a --> package_b\n";
        assert_eq!(diagram, expected);
    }

    #[test]
    fn test_complex_dependencies() {
        // root_package depends on package_a and package_b; package_a depends on package_b
        let root_manifest = CargoRootManifest {
            package: Some(Package {
                name: "root_package".to_string(),
            }),
            dependencies: Some(
                [
                    (
                        "package_a".to_string(),
                        DependencyInfo::Simple("1.0".to_string()),
                    ),
                    (
                        "package_b".to_string(),
                        DependencyInfo::Simple("1.0".to_string()),
                    ),
                ]
                .into_iter()
                .collect(),
            ),
            ..Default::default()
        };

        let nested = vec![
            setup_manifest("package_a", vec!["package_b"]),
            setup_manifest("package_b", vec![]),
        ];

        let diagram = generate_dependency_diagram(root_manifest, nested);
        let expected = "graph TD\n    root_package --> package_a\n    root_package --> package_b\n    package_a --> package_b\n";
        assert_eq!(diagram, expected);
    }
}
