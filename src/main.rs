#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use std::path::Path;
use crate::arguments::get_args;
use crate::manifest_collector::get_manifests;
use crate::package_count::count_packages;

mod package_count;
mod arguments;
mod manifest_collector;

fn main() {
    let args = get_args();

    // count packages
    let amount_of_packages = count_packages(&args.workspace_dir);
    println!("Number of packages in workspace: {}", amount_of_packages);

    // inspect manifests
    let (root, nested) = get_manifests(Path::new(&args.workspace_dir));
}

