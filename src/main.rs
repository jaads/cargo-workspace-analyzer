#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use crate::arguments::get_args;
use crate::diagram_creation::create_diagram;
use crate::exporter::generate_mermaid_png;
use crate::manifests_collector::get_dependency_graph;
use crate::package_counter::count_packages;
use std::path::Path;

mod arguments;
mod dependency_filter;
mod diagram_creation;
mod exporter;
mod graph;
mod manifests_collector;
mod metrics;
mod package_counter;
mod types;

fn main() {
    let args = get_args();

    // count packages
    let amount_of_packages = count_packages(&args.directory);

    // load filtered manifests
    let graph = get_dependency_graph(Path::new(&args.directory));

    // filter dependencies to only include references to workspace members
    let filtered = graph.filter_dependencies();

    println!(
        "Found {} packages in total and {} workspace members.",
        amount_of_packages,
        graph.get_node_count(),
    );

    println!(
        "Found {} dependencies in total and {} workspace dependencies.",
        graph.get_edge_count(),
        filtered.get_edge_count()
    );

    // create diagram, incl. highlights of circular deps
    let result = create_diagram(&filtered);

    // save to file if needed
    if args.no_file {
        println!("{}", result);
    } else {
        generate_mermaid_png(&result);
    }

    // calculate and print the metrics
    filtered.print_coupling();
}
