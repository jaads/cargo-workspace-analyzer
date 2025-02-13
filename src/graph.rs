use std::collections::HashMap;

/// Represents a directed graph.
#[derive(Debug)]
pub struct Graph {
    pub(crate) adjacency_list: HashMap<String, Vec<String>>,
}

impl Graph {
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

    pub fn get_edge_count(&self) -> usize {
        self.adjacency_list.values().map(|deps| deps.len()).sum()
    }

    pub fn get_node_count(&self) -> usize {
        self.adjacency_list.len()
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
