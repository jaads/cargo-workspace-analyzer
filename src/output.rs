use crate::graph::Graph;
use crate::metrics::CouplingMetric;

pub fn print_counts(amount_of_packages: usize, graph: &Graph, filtered: &Graph) {
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
        " {:<40} {:<10} {:<10} {:<10}",
        "Package", "Fan In", "Fan Out", "Instability",
    );
    println!("{:-<75}", ""); // Divider line

    for metric in metrics {
        println!(
            "{:<40} {:<10} {:<10} {:.2}",
            metric.0, metric.1.fan_in, metric.1.fan_out, metric.1.instability
        );
    }
}
