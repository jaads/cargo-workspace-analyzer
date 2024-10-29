#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use std::path::Path;
use crate::arguments::get_args;
use crate::diagram::generate_dependency_diagram;
use crate::manifest_collector::get_manifests;
use crate::package_count::count_packages;

mod package_count;
mod arguments;
mod manifest_collector;
mod diagram;
mod manifest_types;

fn main() {
    let args = get_args();

    // count packages
    let amount_of_packages = count_packages(&args.workspace_dir);
    println!("Number of packages in workspace: {}", amount_of_packages);

    // inspect manifests
    let (root, nested) = get_manifests(Path::new(&args.workspace_dir));

    let diagram = generate_dependency_diagram(root, nested);

    println!("{}", diagram);
}

