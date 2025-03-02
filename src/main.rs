#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use crate::arguments::get_args;
use crate::diagram_creation::create_diagram;
use crate::exporter::generate_mermaid_png;
use crate::manifests_collector::get_dependency_graph;
use crate::metrics::CouplingMetric;
use crate::package_counter::count_packages;
use std::collections::HashMap;
use std::io;
use std::path::Path;

use crate::tui::TUI;
use ratatui::{style::Stylize, widgets::Widget};

mod arguments;
mod dependency_filter;
mod diagram_creation;
mod exporter;
mod graph;
mod manifests_collector;
mod metrics;
mod package_counter;
mod tui;
mod types;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let args = get_args();
    let amount_of_packages = count_packages(&args.directory);

    let graph = get_dependency_graph(Path::new(&args.directory));
    let filtered = graph.filter_out_non_workspace_members();
    let coupling_metrics = graph.calculate_coupling();

    let mut app = TUI {
        exit: false,
        simple_metrics: SimpleMetrics {
            simple_amount: amount_of_packages,
            total_packages: graph.get_node_count(),
            workspace_packages: filtered.get_node_count(),
            total_dependencies: graph.get_edge_count(),
            workspace_dependencies: filtered.get_edge_count(),
        },
        coupling_metrics,
        scroll_offset: 0,
    };

    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug)]
pub struct SimpleMetrics {
    simple_amount: usize,
    total_packages: usize,
    workspace_packages: usize,
    total_dependencies: usize,
    workspace_dependencies: usize,
}

pub struct CouplingMetricsWidget<'a> {
    pub metrics: &'a HashMap<String, (usize, usize, f32)>,
    pub scroll_offset: usize,
}

// remaining parts from the old main
// fn main() {
//
//     // create diagram, incl. highlights of circular deps
//     let result = create_diagram(&filtered);
//
//     // save to file if needed
//     if args.no_file {
//         println!("{}", result);
//     } else {
//         generate_mermaid_png(&result);
//     }

// }
