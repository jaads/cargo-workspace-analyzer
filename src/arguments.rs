use argh::FromArgs;
use std::str::FromStr;

#[derive(FromArgs)]
/// Welcome! Below you see all configuration options you can use.
pub struct Arguments {
    /// the directory of the workspace to analyze.
    /// Defaults to the current working directory.
    #[argh(option, default = "\".\".to_string()", short = 'd')]
    pub directory: String,

    /// specifies the type of the output file which is going to be stored in the current working directory.
    /// Default is set to SVG.
    #[argh(option, short = 'o', default = "FileExportOptions::SVG")]
    pub output_format: FileExportOptions,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileExportOptions {
    SVG,
    MMD,
}

impl FromStr for FileExportOptions {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "svg" => Ok(FileExportOptions::SVG),
            "mmd" => Ok(FileExportOptions::MMD),
            _ => Err(format!("Invalid output format: {}. Allowed: svg, mmd", s)),
        }
    }
}

pub fn get_args() -> Arguments {
    let args: Arguments = argh::from_env();
    args
}
