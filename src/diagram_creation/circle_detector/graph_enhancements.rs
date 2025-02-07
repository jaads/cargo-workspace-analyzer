use crate::graph::Graph;
use std::collections::HashSet;

impl Graph {
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
