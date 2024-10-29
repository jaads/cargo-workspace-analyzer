use argh::FromArgs;

#[derive(FromArgs)]
/// CLI arguments
pub struct Arguments {
    /// the location of the workspace to analyze.
    /// Defaults to the current working directory.
    #[argh(option, default = "\".\".to_string()")]
    pub workspace_dir: String,
}

pub fn get_args() -> Arguments {
    let args: Arguments = argh::from_env();
    args
}

