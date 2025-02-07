use crate::graph::Graph;
use crate::manifests_collector::nested::collect_manifests;
use crate::manifests_collector::root::get_root_manifest;
use std::path::Path;

mod graph_creation;
mod nested;
mod reader;
mod root;

pub fn get_dependency_graph(dir: &Path) -> Graph {
    let _root = get_root_manifest(dir);
    let nested = collect_manifests(dir);
    Graph::new_from_manifests(&nested)
}
