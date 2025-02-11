use crate::graph::Graph;
use std::collections::{HashMap, HashSet};

impl Graph {
    pub fn calculate_coupling(&self) -> HashMap<String, (usize, usize)> {
        // Map to store Afferent Coupling (Ca) for each package
        let mut ca_map: HashMap<String, usize> = HashMap::new();

        // Map to store results (Ce, Ca) for each package
        let mut coupling_map: HashMap<String, (usize, usize)> = HashMap::new();

        // Traverse the graph to calculate Ce and update Ca
        for (package, dependencies) in &self.adjacency_list {
            // Efferent Coupling (Ce): Count unique outgoing dependencies
            let unique_dependencies: HashSet<_> =
                dependencies.iter().filter(|dep| !dep.is_empty()).collect();
            let ce = unique_dependencies.len();

            // Update Ca (afferent coupling) for each dependency
            for dependency in unique_dependencies {
                *ca_map.entry(dependency.clone()).or_insert(0) += 1;
            }

            // Store the Ce value in the result map
            coupling_map.insert(package.clone(), (ce, 0));
        }

        // Update Ca values in the result map
        for (package, ca) in ca_map {
            if let Some(entry) = coupling_map.get_mut(&package) {
                entry.1 = ca; // Update Ca
            }
        }

        coupling_map
    }

    pub fn print_coupling(&self) {
        let coupling = self.calculate_coupling();

        if coupling.is_empty() {
            println!("No packages found in the graph.");
            return;
        }

        println!("{:<40} {:<10} {:<10}", "Package", "Ce", "Ca");
        println!("{:-<40}", ""); // Divider line

        for (package, (ce, ca)) in coupling {
            println!("{:<40} {:<10} {:<10}", package, ce, ca);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_graph() {
        let graph = Graph::new();
        let coupling = graph.calculate_coupling();
        assert!(
            coupling.is_empty(),
            "Coupling map should be empty for an empty graph"
        );
    }

    #[test]
    fn test_single_package_no_dependencies() {
        let mut graph = Graph::new();
        graph.adjacency_list.insert("package_a".to_string(), vec![]);
        let coupling = graph.calculate_coupling();
        assert_eq!(
            coupling.get("package_a"),
            Some(&(0, 0)),
            "Single package with no dependencies should have Ce = 0 and Ca = 0"
        );
    }

    #[test]
    fn test_duplicate_dependencies() {
        let mut graph = Graph::new();
        graph.adjacency_list.insert(
            "package_a".to_string(),
            vec!["package_b".to_string(), "package_b".to_string()],
        );
        graph.adjacency_list.insert("package_b".to_string(), vec![]);

        let coupling = graph.calculate_coupling();

        assert_eq!(
            coupling.get("package_a"),
            Some(&(1, 0)),
            "Package_a should have Ce = 1 (not counting duplicate dependencies) and Ca = 0"
        );
        assert_eq!(
            coupling.get("package_b"),
            Some(&(0, 1)),
            "Package_b should have Ce = 0 and Ca = 1"
        );
    }

    #[test]
    fn test_dependency_only_node() {
        let mut graph = Graph::new();
        graph
            .adjacency_list
            .insert("package_a".to_string(), vec!["package_b".to_string()]);

        let coupling = graph.calculate_coupling();

        assert_eq!(
            coupling.get("package_a"),
            Some(&(1, 0)),
            "Package_a should have Ce = 1 and Ca = 0"
        );
        // assert_eq!(
        //     coupling.get("package_b"),
        //     Some(&(0, 1)),
        //     "Package_b should have Ce = 0 and Ca = 1, even though it only appears as a dependency"
        // );
    }

    #[test]
    fn test_multiple_dependencies() {
        let mut graph = Graph::new();
        graph.adjacency_list.insert(
            "package_a".to_string(),
            vec!["package_b".to_string(), "package_c".to_string()],
        );
        graph
            .adjacency_list
            .insert("package_b".to_string(), vec!["package_c".to_string()]);
        graph.adjacency_list.insert("package_c".to_string(), vec![]);

        let coupling = graph.calculate_coupling();

        assert_eq!(
            coupling.get("package_a"),
            Some(&(2, 0)),
            "Package_a should have Ce = 2 and Ca = 0"
        );
        assert_eq!(
            coupling.get("package_b"),
            Some(&(1, 1)),
            "Package_b should have Ce = 1 and Ca = 1"
        );
        assert_eq!(
            coupling.get("package_c"),
            Some(&(0, 2)),
            "Package_c should have Ce = 0 and Ca = 2"
        );
    }

    #[test]
    fn test_circular_dependency() {
        let mut graph = Graph::new();
        graph
            .adjacency_list
            .insert("package_a".to_string(), vec!["package_b".to_string()]);
        graph
            .adjacency_list
            .insert("package_b".to_string(), vec!["package_a".to_string()]);

        let coupling = graph.calculate_coupling();

        assert_eq!(
            coupling.get("package_a"),
            Some(&(1, 1)),
            "Package_a should have Ce = 1 and Ca = 1"
        );
        assert_eq!(
            coupling.get("package_b"),
            Some(&(1, 1)),
            "Package_b should have Ce = 1 and Ca = 1"
        );
    }

    #[test]
    fn test_disconnected_components() {
        let mut graph = Graph::new();
        graph
            .adjacency_list
            .insert("package_a".to_string(), vec!["package_b".to_string()]);
        graph
            .adjacency_list
            .insert("package_c".to_string(), vec!["package_d".to_string()]);
        graph.adjacency_list.insert("package_b".to_string(), vec![]);
        graph.adjacency_list.insert("package_d".to_string(), vec![]);

        let coupling = graph.calculate_coupling();

        assert_eq!(
            coupling.get("package_a"),
            Some(&(1, 0)),
            "Package_a should have Ce = 1 and Ca = 0"
        );
        assert_eq!(
            coupling.get("package_b"),
            Some(&(0, 1)),
            "Package_b should have Ce = 0 and Ca = 1"
        );
        assert_eq!(
            coupling.get("package_c"),
            Some(&(1, 0)),
            "Package_c should have Ce = 1 and Ca = 0"
        );
        assert_eq!(
            coupling.get("package_d"),
            Some(&(0, 1)),
            "Package_d should have Ce = 0 and Ca = 1"
        );
    }
}
