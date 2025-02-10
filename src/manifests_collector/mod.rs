use crate::graph::Graph;
use crate::manifests_collector::filter::filter_findings_by_workspace;
use crate::manifests_collector::nested::collect_manifests;
use crate::manifests_collector::root::get_root_manifest;
use std::path::Path;

mod filter;
mod graph_creation;
mod nested;
mod reader;
mod root;

pub fn get_dependency_graph(dir: &Path) -> Graph {
    let root_finding = get_root_manifest(dir);
    let nested = collect_manifests(dir);
    let filtered = filter_findings_by_workspace(&root_finding, nested);
    Graph::new_from_manifests(&filtered)
}
