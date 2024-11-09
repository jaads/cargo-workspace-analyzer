use crate::diagram::circle_detector::highlight_cycles_in_mermaid;
use crate::diagram::diagram_creation::generate_dependency_diagram;
use crate::manifest_types::nested::ManifestFindings;
use crate::manifest_types::root::CargoRootManifest;

mod graph;
mod circle_detector;
mod diagram_creation;

pub fn create_diagram(root: CargoRootManifest, nested: ManifestFindings) -> String{
    // create mermaid diagram as text
    let diagram = generate_dependency_diagram(root, nested);
    let result = highlight_cycles_in_mermaid(&diagram);

    result
}