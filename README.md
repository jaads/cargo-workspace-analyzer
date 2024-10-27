# Cargo workspace analyzer

A CLI tool which provides insights about
a [Cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html).

Currently, the following is supported:

- count the number of packages within a workspace

## How to use

Install it globally with `cargo install`:

 ```sh
 cargo install cargo-workspace-analyzer
 ```

Then navigate to a Cargo workspace and run the tool:

 ```sh
 cd path/to/your/workspace
 cargo-workspace-analyzer
 ```

## Roadmap

The following is currently planed.

- show how packages are related to each other