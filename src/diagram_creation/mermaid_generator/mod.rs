use graph_creation::get_graph_from_manifests;
use crate::types::nested::ManifestFindings;

mod graph_creation;

// Function to generate the component diagram in Mermaid format
pub fn generate_mermaid_markdown(nested: ManifestFindings) -> String {
    let mut diagram = String::from("graph TD\n");
    let adjacent_list = get_graph_from_manifests(&nested);

    // Sort package names alphabetically
    let mut package_names: Vec<&String> = adjacent_list.keys().collect();
    package_names.sort();

    // Set to track packages that are already referenced in edges
    let mut referenced_packages = std::collections::HashSet::new();

    // Generate edges for packages in sorted order
    for pkg in package_names.iter() {
        if let Some(deps) = adjacent_list.get(*pkg) {
            for dep in deps {
                if adjacent_list.contains_key(dep) {
                    diagram.push_str(&format!("    {} --> {}\n", pkg, dep));
                    referenced_packages.insert(dep.clone());
                }
            }
        }
    }

    // Add standalone nodes for packages with no dependencies and not referenced in any edges
    for pkg in package_names.iter() {
        if let Some(deps) = adjacent_list.get(*pkg) {
            if deps.is_empty() && !referenced_packages.contains(*pkg) {
                diagram.push_str(&format!("    {}\n", pkg));
            }
        }
    }

    diagram
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::commons::{DependencyInfo, Package};
    use crate::types::nested::{Manifest, ManifestFinding};
    use std::collections::HashMap;
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
        let nested = vec![];
        let diagram = generate_mermaid_markdown(nested);
        let expected = "graph TD\n";
        assert_eq!(diagram, expected);
    }

    #[test]
    fn test_linear_dependencies() {
        let nested = vec![
            setup_manifest("package_a", vec!["package_b"]),
            setup_manifest("package_b", vec![]),
        ];

        let diagram = generate_mermaid_markdown(nested);
        let expected = "graph TD\n    package_a --> package_b\n";
        assert_eq!(diagram, expected);
    }

    #[test]
    fn test_complex_dependencies() {
        let nested = vec![
            setup_manifest("package_a", vec!["package_b"]),
            setup_manifest("package_b", vec![]),
        ];

        let diagram = generate_mermaid_markdown(nested);
        let expected = "graph TD\n    package_a --> package_b\n";
        assert_eq!(diagram, expected);
    }
}
