#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use std::path::Path;
use crate::arguments::get_args;
use crate::circular_deps_finder::highlight_cycles_in_mermaid;
use crate::diagram_creation::generate_dependency_diagram;
use crate::manifest_collector::get_manifests;
use crate::package_count::count_packages;
use crate::save_to_disk::generate_mermaid_png;

mod package_count;
mod arguments;
mod manifest_collector;
mod diagram_creation;
mod manifest_types;
mod circular_deps_finder;
mod save_to_disk;

fn main() {
    let args = get_args();

    // count packages
    let amount_of_packages = count_packages(&args.workspace_dir);
    println!("Number of packages in workspace: {}", amount_of_packages);

    // inspect manifests
    let (root, nested) = get_manifests(Path::new(&args.workspace_dir));

    // create mermaid diagram in syntax
    let diagram = generate_dependency_diagram(root, nested);
    let result = highlight_cycles_in_mermaid(&diagram);


    if args.save_to_file {
        // write to file
        let output_path = "diagram_output.png";
        match generate_mermaid_png(&result, output_path) {
            Ok(_) => println!("Diagram rendered successfully."),
            Err(e) => eprintln!("Failed to render diagram: {}", e),
        }
    } else {
        println!("{}", result);
    }
}

