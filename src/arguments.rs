use argh::FromArgs;

#[derive(FromArgs)]
/// CLI arguments
pub struct Arguments {
    /// the directory of the workspace to analyze.
    /// Defaults to the current working directory.
    #[argh(option, default = "\".\".to_string()", short = 'd')]
    pub directory: String,
}

pub fn get_args() -> Arguments {
    let args: Arguments = argh::from_env();
    args
}
