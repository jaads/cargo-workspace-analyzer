use crate::manifest_collector::nested::collect_manifests;
use crate::manifest_collector::root::get_root_manifest;
use crate::manifest_types::nested::ManifestFindings;
use crate::manifest_types::root::CargoRootManifest;
use std::path::Path;

mod nested;
mod reader;
mod root;

pub fn get_manifests(dir: &Path) -> (CargoRootManifest, ManifestFindings) {
    let root = get_root_manifest(dir);
    let nested = collect_manifests(dir);

    (root, nested)
}
