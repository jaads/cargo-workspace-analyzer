use crate::diagram_creation::circle_detector::detect_circular_dependencies;
use crate::diagram_creation::mermaid_generator::generate_mermaid_markdown;
use crate::graph::Graph;

mod circle_detector;
mod mermaid_generator;

pub fn create_diagram(graph: Graph) -> String {
    let diagram = generate_mermaid_markdown(graph);
    detect_circular_dependencies(&diagram)
}
