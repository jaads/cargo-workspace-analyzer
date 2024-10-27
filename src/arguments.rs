use argh::FromArgs;

#[derive(FromArgs)]
/// A tool to analyze Rust workspace packages
pub struct Arguments {
    /// the directory of the workspace to analyze.
    /// Default to the current directory
    #[argh(option, default = "\".\".to_string()")]
    pub workspace_dir: String,
}

pub fn get_args() -> Arguments {
    let args: Arguments = argh::from_env();
    args
}

