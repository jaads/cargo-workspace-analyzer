use crate::package_count::count_packages;

mod package_count;


fn main() {
    let workspace_dir = ".";
    println!("Number of packages in workspace: {}", count_packages(workspace_dir));
}

