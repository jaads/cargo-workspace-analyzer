use crate::diagram::circle_detector::highlight_cycles_in_mermaid;
use crate::diagram::diagram_creation::generate_dependency_diagram;
use crate::manifest_types::nested::ManifestFindings;

mod circle_detector;
mod diagram_creation;
mod graph;

pub fn create_diagram(nested: ManifestFindings) -> String {
    // create mermaid diagram as text
    let diagram = generate_dependency_diagram(nested);
    let result = highlight_cycles_in_mermaid(&diagram);

    result
}
