use std::path::Path;
use crate::manifest_collector::nested::{collect_manifests, ManifestFindings};
use crate::manifest_collector::root::{get_root_manifest};
use crate::manifest_collector::root_types::CargoRootManifest;

mod nested;
mod root;
mod reader;
pub mod nested_types;
pub mod root_types;

pub fn get_manifests(dir: &Path) -> (CargoRootManifest, ManifestFindings) {
    let root = get_root_manifest(dir);
    let nested = collect_manifests(dir);

    (root, nested)
}