use std::collections::{HashMap, HashSet};

/// Represents a directed graph to detect cycles.
struct Graph {
    adjacency_list: HashMap<String, Vec<String>>,
}

impl Graph {
    /// Creates a new graph.
    fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }

    /// Adds a directed edge from `from` to `to`.
    fn add_edge(&mut self, from: &str, to: &str) {
        self.adjacency_list
            .entry(from.to_string())
            .or_insert_with(Vec::new)
            .push(to.to_string());
    }

    /// Detects cycles and returns a set of edges involved in cycles.
    fn detect_cycles(&self) -> HashSet<(String, String)> {
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

/// Checks a Mermaid diagram string for circular dependencies and marks cycles in red.
pub fn highlight_cycles_in_mermaid(mermaid_diagram: &str) -> String {
    let mut graph = Graph::new();
    let mut edges = Vec::new();

    // Parse the mermaid string to extract edges.
    for line in mermaid_diagram.lines() {
        if let Some((from, to)) = parse_edge(line) {
            graph.add_edge(&from, &to);
            edges.push((from.clone(), to.clone(), line.to_string()));
        }
    }

    // Detect cycles
    let cycle_edges = graph.detect_cycles();

    // Start the output with "graph TD"
    let mut highlighted_diagram = String::from("graph TD\n");

    // Modify the mermaid string to highlight cycles in red
    for (from, to, original_line) in edges {
        if cycle_edges.contains(&(from.clone(), to.clone())) {
            // Modify the line to mark the edge as red
            highlighted_diagram.push_str(&format!("{}:::red\n", original_line));
        } else {
            // Keep the line as-is if it's not part of a cycle
            highlighted_diagram.push_str(&format!("{}\n", original_line));
        }
    }

    // Add red style definition to the diagram for edges in cycles
    highlighted_diagram.push_str("classDef red stroke:#ff0000,stroke-width:2px;\n");

    highlighted_diagram
}


/// Parses a line for an edge in a Mermaid diagram of the form `A --> B`.
fn parse_edge(line: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() >= 3 && parts[1] == "-->" {
        Some((parts[0].to_string(), parts[2].to_string()))
    } else {
        None
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_cycles() {
        let mermaid_diagram = r#"
            graph TD
            A --> B
            B --> C
            C --> D
        "#;

        let result = highlight_cycles_in_mermaid(mermaid_diagram);

        // Expecting no red highlights as there are no cycles
        assert!(!result.contains(":::red"));
    }

    #[test]
    fn test_single_cycle() {
        let mermaid_diagram = r#"
            graph TD
            A --> B
            B --> C
            C --> A
        "#;

        let result = highlight_cycles_in_mermaid(mermaid_diagram);

        // Expecting red highlights on all edges that form a cycle
        assert!(result.contains("A --> B:::red"));
        assert!(result.contains("B --> C:::red"));
        assert!(result.contains("C --> A:::red"));
    }

    #[test]
    fn test_multiple_cycles() {
        let mermaid_diagram = r#"
            graph TD
            A --> B
            B --> C
            C --> A
            D --> E
            E --> F
            F --> D
        "#;

        let result = highlight_cycles_in_mermaid(mermaid_diagram);

        // Expecting red highlights on edges that form cycles
        assert!(result.contains("A --> B:::red"));
        assert!(result.contains("B --> C:::red"));
        assert!(result.contains("C --> A:::red"));
        assert!(result.contains("D --> E:::red"));
        assert!(result.contains("E --> F:::red"));
        assert!(result.contains("F --> D:::red"));
    }

    #[test]
    fn test_disconnected_graph_with_cycle() {
        let mermaid_diagram = r#"
            graph TD
            A --> B
            B --> C
            C --> A
            D --> E
        "#;

        let result = highlight_cycles_in_mermaid(mermaid_diagram);

        // Expecting red highlights only on edges that form the cycle
        assert!(result.contains("A --> B:::red"));
        assert!(result.contains("B --> C:::red"));
        assert!(result.contains("C --> A:::red"));
        assert!(!result.contains("D --> E:::red"));
    }

    #[test]
    fn test_graph_with_self_loop() {
        let mermaid_diagram = r#"
            graph TD
            A --> A
            B --> C
        "#;

        let result = highlight_cycles_in_mermaid(mermaid_diagram);

        // Expecting the self-loop to be highlighted in red as it forms a cycle
        assert!(result.contains("A --> A:::red"));
        assert!(!result.contains("B --> C:::red"));
    }

    #[test]
    fn test_complex_graph_with_mixed_cycles() {
        let mermaid_diagram = r#"
            graph TD
            A --> B
            B --> C
            C --> A
            B --> D
            D --> E
            E --> B
            F --> G
        "#;

        let result = highlight_cycles_in_mermaid(mermaid_diagram);

        // Expecting red highlights only on edges that form cycles
        assert!(result.contains("A --> B:::red"));
        assert!(result.contains("B --> C:::red"));
        assert!(result.contains("C --> A:::red"));
        assert!(result.contains("B --> D"));
        assert!(result.contains("D --> E:::red"));
        assert!(result.contains("E --> B:::red"));
        assert!(!result.contains("F --> G:::red"));
    }
}
