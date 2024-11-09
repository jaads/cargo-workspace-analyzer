#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use std::path::Path;
use crate::arguments::get_args;
use crate::diagram::{create_diagram};
use crate::manifest_collector::get_manifests;
use crate::package_counter::count_packages;
use crate::exporter::generate_mermaid_png;

mod package_counter;
mod arguments;
mod manifest_collector;
mod manifest_types;
mod exporter;
mod diagram;

fn main() {
    let args = get_args();

    // count packages
    let amount_of_packages = count_packages(&args.directory);
    println!("{} packages in total", amount_of_packages);

    // load manifests
    let (root, nested) = get_manifests(Path::new(&args.directory));

    // create diagram, incl. highlights of circular deps
    let result = create_diagram(root,nested);

    // save to file if needed
    if args.no_file {
        println!("{}", result);
    } else {
        generate_mermaid_png(&result);
    }
}

