#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use std::path::Path;
use crate::arguments::get_args;
use crate::circular_deps_finder::highlight_cycles_in_mermaid;
use crate::diagram_generator::generate_dependency_diagram;
use crate::manifest_collector::get_manifests;
use crate::package_counter::count_packages;
use crate::exporter::generate_mermaid_png;

mod package_counter;
mod arguments;
mod manifest_collector;
mod diagram_generator;
mod manifest_types;
mod circular_deps_finder;
mod exporter;

fn main() {
    let args = get_args();

    // count packages
    let amount_of_packages = count_packages(&args.directory);
    println!("{} packages in total", amount_of_packages);

    // load manifests
    let (root, nested) = get_manifests(Path::new(&args.directory));

    // create mermaid diagram as text
    let diagram = generate_dependency_diagram(root, nested);
    let result = highlight_cycles_in_mermaid(&diagram);

    // save to file if needed
    if args.no_file {
        println!("{}", result);
    } else {
        generate_mermaid_png(&result);
    }
}

