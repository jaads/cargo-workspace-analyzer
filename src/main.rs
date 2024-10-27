use crate::arguments::get_args;
use crate::package_count::count_packages;

mod package_count;
mod arguments;

fn main() {
    let args = get_args();

    println!("Number of packages in workspace: {}", count_packages(&args.workspace_dir));
}

