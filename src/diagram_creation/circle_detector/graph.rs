use std::collections::{HashMap, HashSet};

/// Represents a directed graph to detect cycles.
pub struct Graph {
    adjacency_list: HashMap<String, Vec<String>>,
}

impl Graph {
    /// Creates a new graph.
    pub(crate) fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }

    /// Adds a directed edge from `from` to `to`.
    pub(crate) fn add_edge(&mut self, from: &str, to: &str) {
        self.adjacency_list
            .entry(from.to_string())
            .or_insert_with(Vec::new)
            .push(to.to_string());
    }

    /// Detects cycles and returns a set of edges involved in cycles.
    pub(crate) fn detect_cycles(&self) -> HashSet<(String, String)> {
        let mut visited = HashSet::new();
        let mut stack = HashSet::new();
        let mut path = Vec::new();
        let mut cycle_edges = HashSet::new();

        for node in self.adjacency_list.keys() {
            if !visited.contains(node) {
                self.dfs(node, &mut visited, &mut stack, &mut path, &mut cycle_edges);
            }
        }
        cycle_edges
    }

    /// Helper function for DFS traversal to detect cycles.
    fn dfs(
        &self,
        node: &str,
        visited: &mut HashSet<String>,
        stack: &mut HashSet<String>,
        path: &mut Vec<String>,
        cycle_edges: &mut HashSet<(String, String)>,
    ) -> bool {
        if stack.contains(node) {
            // Mark all edges in the cycle
            let cycle_start_index = path.iter().position(|n| n == node).unwrap();
            for i in cycle_start_index..path.len() - 1 {
                cycle_edges.insert((path[i].clone(), path[i + 1].clone()));
            }
            cycle_edges.insert((path[path.len() - 1].clone(), node.to_string())); // close the cycle
            return true;
        }

        if visited.contains(node) {
            return false;
        }

        visited.insert(node.to_string());
        stack.insert(node.to_string());
        path.push(node.to_string());

        if let Some(neighbors) = self.adjacency_list.get(node) {
            for neighbor in neighbors {
                if self.dfs(neighbor, visited, stack, path, cycle_edges) {
                    return true;
                }
            }
        }

        stack.remove(node);
        path.pop();
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_edge() {
        // Create a new graph
        let mut graph = Graph::new();

        // Add some edges
        graph.add_edge("A", "B");
        graph.add_edge("A", "C");
        graph.add_edge("B", "D");
        graph.add_edge("D", "E");

        // Check adjacency list for correctness
        let adj_list = &graph.adjacency_list;

        // Verify that "A" points to "B" and "C"
        assert!(adj_list.contains_key("A"));
        assert_eq!(adj_list["A"], vec!["B", "C"]);

        // Verify that "B" points to "D"
        assert!(adj_list.contains_key("B"));
        assert_eq!(adj_list["B"], vec!["D"]);

        // Verify that "D" points to "E"
        assert!(adj_list.contains_key("D"));
        assert_eq!(adj_list["D"], vec!["E"]);

        // Verify that "E" has no outgoing edges
        assert!(adj_list.get("E").is_none());
    }
}
