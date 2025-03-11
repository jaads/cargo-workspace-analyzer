use crate::graph::Graph;
use crate::metrics::CouplingMetric;
use tabled::settings::Style;
use tabled::{Table, Tabled};

#[derive(Tabled)]
#[tabled(rename_all = "PascalCase")]
struct CouplingRow {
    package: String,
    fan_in: usize,
    fan_out: usize,
    instability: String,
}

#[derive(Tabled)]
#[tabled(rename_all = "PascalCase")]
struct CountMetrics {
    category: &'static str,
    packages: usize,
    dependencies: usize,
}

pub fn print_counts(graph: &Graph, filtered: &Graph) {
    let counts = vec![
        CountMetrics {
            category: "Total",
            packages: graph.get_node_count(),
            dependencies: graph.get_edge_count(),
        },
        CountMetrics {
            category: "Workspace",
            packages: filtered.get_node_count(),
            dependencies: filtered.get_edge_count(),
        },
    ];

    let table = Table::new(counts).with(Style::rounded()).to_string();
    println!("{}", table);
}

pub fn print_coupling(metrics: CouplingMetric) {
    if metrics.is_empty() {
        println!("No packages found in the graph.");
        return;
    }

    let rows: Vec<CouplingRow> = metrics
        .into_iter()
        .map(|(package, data)| CouplingRow {
            package,
            fan_in: data.fan_in,
            fan_out: data.fan_out,
            instability: format!("{:.2}", data.instability),
        })
        .collect();

    let table = Table::new(rows).with(Style::rounded()).to_string();

    println!("{}", table);
}
