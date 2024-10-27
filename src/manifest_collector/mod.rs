use std::path::Path;
use crate::manifest_collector::nested::{collect_manifests, ManifestFindings};
use crate::manifest_collector::root::{get_root_manifest, CargoRootManifest};

mod nested;
mod root;
mod manifest_reader;

pub fn get_manifests(dir: &Path) -> (CargoRootManifest, ManifestFindings) {
    let root = get_root_manifest(dir);
    let nested = collect_manifests(dir);

    (root, nested)
}