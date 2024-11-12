use crate::diagram::graph::Graph;

/// Builds a graph from a Mermaid diagram string by parsing edges.
pub fn build_graph_from_mermaid(mermaid_diagram: &str) -> (Graph, Vec<(String, String, String)>) {
    let mut graph = Graph::new();
    let mut edges = Vec::new();

    // Parse the mermaid string to extract edges.
    for line in mermaid_diagram.lines() {
        if let Some((from, to)) = parse_edge(line) {
            graph.add_edge(&from, &to);
            edges.push((from.clone(), to.clone(), line.to_string()));
        }
    }

    (graph, edges)
}

/// Checks a Mermaid diagram string for circular dependencies and marks cycles in red.
pub fn highlight_cycles_in_mermaid(mermaid_diagram: &str) -> String {
    // Build the graph and get the parsed edges.
    let (graph, edges) = build_graph_from_mermaid(mermaid_diagram);

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
