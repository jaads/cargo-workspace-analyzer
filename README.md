# Cargo Workspace Analyzer

A CLI tool which provides insights about how packages
within a [Cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) are related to each other.
Currently, the following is supported.

See `cargo-workspace-analyzer --help` for all options.

## Workspace Visualization

It visualizes the workspace with a [Mermaid](https://mermaid.js.org/) diagram. That way the user can see how packages
depend on each other may identify layers of the application. As an example, here is the resulting diagram a randomly
selected workspace, [Tauri](https://github.com/tauri-apps/tauri).

<img src="examples/tauri.svg" alt="example diagram of packages within a workspace">

To have such diagram gives you the following advantages:

- a high level overview of the software
- an idea about the degree of coupling between your packages

By default, the tool creates an SVG file called `cargo-workspace-analyzer.svg`. You can change the output format to a
`.mmd` file with the option `-o mdd`.

## Circular Dependency Detection

This analyzer finds circular dependencies. It highlights those packages, which form a circle. By running the analyzer
regularly, one can detect circular dependencies before they get hard if not impossible to resolve later on. See
this [example](https://www.mermaidchart.com/raw/35c87214-1aea-46a9-b633-8fd3bd4f90ad?theme=light&version=v0.1&format=svg).

## Metric calculations

The created graph is used to calculate common metrics in regard to the coupling of packages.
The following metric are supported:

- Fan In
- Fan Out
- Instability Metric

## Package and Dependency Count

It will also display the amount of packages and the amount of dependencies.
Once for all the founds packages and dependencies in the codebase, and once only for the packages actually within the
workspace and interdependencies.

# Installation

Install it globally:

 ```sh
 cargo install cargo-workspace-analyzer
 ```

To render the Mermaid diagram and store it so disk (which is the default behaviour), you would need to have
the [Mermaid CLI](https://github.com/mermaid-js/mermaid-cli) installed as well, which run on Node.js.

```sh
npm install -g @mermaid-js/mermaid-cli
 ```

# Usage

For all details, use `cargo-workspace-analyzer --help`. However here is how you can use it generally:
Navigate to a Cargo workspace and run the tool:

 ```sh
 cd path/to/your/workspace
 cargo-workspace-analyzer
 ```

Or use an argument to specify the location of the workspace and run it from where ever you want.

 ```sh
 cargo-workspace-analyzer --working-dir /path/to/your/workspace
 ```

By default, it will create an SVG files in the directory where it run.
You can also create a mmd file, in which then only the diagram is placed in [Mermaid](https://mermaid.js.org/) syntax.
You can use it then for further processing. Here's an example of the content:

```
graph TD
    service-1 --> db-connector
    API --> service-2
    API --> service-1
    service-2 --> db-connector
```



