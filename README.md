# cargo-workspace-analyzer
Gives insights about cargo packages and how they are related to each other

 # Cargo workspace analyzer
 A CLI tool which traverses a [Cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) and gets insights of it.

 Currently, the following is supported:\
 - count the number of packages within the workspace


 Example
 ```sh
 cargo-install cargo-workspace-analyzer

 cd path/to/your/workspace
 cargo-workspace-analyzer
 ```
