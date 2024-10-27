use std::path::Path;
use crate::manifest_collector::package_manifests::collect_manifests;
use crate::manifest_collector::virtual_manifest::get_root_manifest;

mod package_manifests;
mod virtual_manifest;

pub fn get_manifest(dir: &Path) {
    collect_manifests(dir);
    get_root_manifest(dir);
}