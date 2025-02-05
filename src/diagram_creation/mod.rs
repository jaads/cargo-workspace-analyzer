use crate::diagram_creation::circle_detector::detect_circular_dependencies;
use crate::diagram_creation::mermaid_generator::generate_mermaid_markdown;
use crate::types::nested::ManifestFindings;

mod circle_detector;
mod mermaid_generator;

pub fn create_diagram(nested: ManifestFindings) -> String {
    let diagram = generate_mermaid_markdown(nested);
    let result = detect_circular_dependencies(&diagram);
    result
}
