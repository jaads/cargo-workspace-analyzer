use crate::graph::Graph;
use std::collections::HashSet;

impl Graph {
    pub fn filter_out_non_workspace_members(&self) -> Graph {
        let node_set: HashSet<_> = self.adjacency_list.keys().cloned().collect();

        let filtered_adjacency_list = self
            .adjacency_list
            .iter()
            .map(|(package, dependencies)| {
                let filtered_dependencies = dependencies
                    .iter()
                    .filter(|dep| node_set.contains(*dep)) // Keep only dependencies that exist as keys
                    .cloned()
                    .collect();
                (package.clone(), filtered_dependencies)
            })
            .collect();

        Graph {
            adjacency_list: filtered_adjacency_list,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_dependencies() {
        let original_graph = Graph {
            adjacency_list: vec![
                (
                    "A".to_string(),
                    vec!["B".to_string(), "C".to_string(), "X".to_string()],
                ), // "X" is not a node
                ("B".to_string(), vec!["C".to_string(), "Y".to_string()]), // "Y" is not a node
                ("C".to_string(), vec!["A".to_string()]),                  // Circular dependency
                ("D".to_string(), vec!["E".to_string()]),                  // "E" is missing
            ]
            .into_iter()
            .collect(),
        };

        let expected_filtered_graph = Graph {
            adjacency_list: vec![
                ("A".to_string(), vec!["B".to_string(), "C".to_string()]), // "X" removed
                ("B".to_string(), vec!["C".to_string()]),                  // "Y" removed
                ("C".to_string(), vec!["A".to_string()]), // Circular dependency remains
                ("D".to_string(), vec![]),                // "E" removed
            ]
            .into_iter()
            .collect(),
        };

        let filtered_graph = original_graph.filter_out_non_workspace_members();
        assert_eq!(filtered_graph, expected_filtered_graph);
    }

    #[test]
    fn test_filter_empty_graph() {
        let empty_graph = Graph::new();
        let filtered_graph = empty_graph.filter_out_non_workspace_members();
        assert!(filtered_graph.adjacency_list.is_empty());
    }

    #[test]
    fn test_no_external_dependencies() {
        let graph = Graph {
            adjacency_list: vec![
                ("A".to_string(), vec!["B".to_string(), "C".to_string()]),
                ("B".to_string(), vec!["A".to_string(), "C".to_string()]),
                ("C".to_string(), vec!["A".to_string(), "B".to_string()]),
            ]
            .into_iter()
            .collect(),
        };

        let filtered_graph = graph.filter_out_non_workspace_members();
        assert_eq!(filtered_graph, graph); // No filtering needed, should be identical
    }
}
