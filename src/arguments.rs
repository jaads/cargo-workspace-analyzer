use argh::FromArgs;

#[derive(FromArgs)]
/// CLI arguments
pub struct Arguments {
    /// the directory of the workspace to analyze.
    /// Defaults to the current working directory.
    #[argh(option, default = "\".\".to_string()", short = 'd')]
    pub directory: String,

    /// renders the diagram and stores it to a PNG file in the current working directory.
    /// Default is false.
    #[argh(switch)]
    pub no_file: bool,
}

pub fn get_args() -> Arguments {
    let args: Arguments = argh::from_env();
    args
}

