use crate::manifests_collector::nested::collect_manifests;
use crate::manifests_collector::root::get_root_manifest;
use crate::types::nested::ManifestFindings;
use crate::types::root::CargoRootManifest;
use std::path::Path;

mod nested;
mod reader;
mod root;

pub fn get_manifests(dir: &Path) -> (CargoRootManifest, ManifestFindings) {
    let root = get_root_manifest(dir);
    let nested = collect_manifests(dir);

    (root, nested)
}
