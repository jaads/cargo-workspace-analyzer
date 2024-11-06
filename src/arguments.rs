use argh::FromArgs;

#[derive(FromArgs)]
/// CLI arguments
pub struct Arguments {
    /// the location of the workspace to analyze.
    /// Defaults to the current working directory.
    #[argh(option, default = "\".\".to_string()")]
    pub workspace_dir: String,

    /// renders the diagram and stores it to a PNG file in the current working directory.
    /// Default is false.
    #[argh(switch)]
    pub save_to_file: bool,
}

pub fn get_args() -> Arguments {
    let args: Arguments = argh::from_env();
    args
}

