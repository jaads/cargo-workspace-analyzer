#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use crate::arguments::get_args;
use crate::diagram_creation::create_diagram;
use crate::exporter::export;
use crate::manifests_collector::get_dependency_graph;
use crate::output::{print_counts, print_coupling};
use crate::package_counter::count_packages;
use std::path::Path;

mod arguments;
mod dependency_filter;
mod diagram_creation;
mod exporter;
mod graph;
mod manifests_collector;
mod metrics;
mod output;
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

    print_counts(amount_of_packages, &graph, &filtered);

    // create diagram, incl. highlights of circular deps
    let mmd = create_diagram(&filtered);

    export(&mmd, args.output_format);

    // calculate and print the metrics
    let metrics = filtered.calculate_coupling();
    print_coupling(metrics);
}
