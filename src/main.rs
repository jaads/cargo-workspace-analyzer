#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use crate::arguments::get_args;
use crate::diagram_creation::create_diagram;
use crate::exporter::export;
use crate::manifests_collector::get_dependency_graph;
use crate::metrics::CouplingMetric;
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
    let mmd = create_diagram(&filtered);

    export(&mmd, args.output_format);

    // calculate and print the metrics
    let metrics = filtered.calculate_coupling();
    print_coupling(metrics);
}

pub fn print_coupling(metrics: CouplingMetric) {
    if metrics.is_empty() {
        println!("No packages found in the graph.");
        return;
    }

    println!();
    println!("Metrics:");
    println!();
    println!(
        "{:<40} {:<20} {:<20} {:<10}",
        "Package", "Efferent Coupling", "Afferent Coupling", "Instability"
    );
    println!("{:-<95}", ""); // Divider line

    for (package, (ce, ca, instability)) in metrics {
        println!("{:<40} {:<20} {:<20} {:.2}", package, ce, ca, instability);
    }
}
