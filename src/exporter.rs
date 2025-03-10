use crate::arguments::FileExportOptions;
use std::fs::{remove_file, File};
use std::io::Write;
use std::process::Command;

const SVG_OUTPUT: &str = "workspace-analyzer.svg";
const MMD_OUTPUT: &str = "workspace-analyzer.mmd";

pub fn export(content: &str, output_format: FileExportOptions) {
    // let path = Path::new(MMD_OUTPUT_PATH);
    write_mermaid_code_to_file(content, MMD_OUTPUT);

    match output_format {
        FileExportOptions::SVG => {
            replace_mmd_file_with_svg();
            println!("✅ File successfully written to: {}", SVG_OUTPUT);
        }
        FileExportOptions::MMD => {
            println!("✅ File successfully written to: {}", MMD_OUTPUT);
        }
    }
}

pub fn replace_mmd_file_with_svg() {
    verify_mmdc_installation();
    render_mermaid_to_svg();
    if let Err(e) = remove_file(MMD_OUTPUT) {
        println!("Failed to remove file: {}", e);
    }
}

fn verify_mmdc_installation() {
    let is_installed = Command::new("mmdc")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false);

    if !is_installed {
        eprintln!(
            "Error: Mermaid CLI (`mmdc`) is not installed. \
             Please install it by running `npm install -g @mermaid-js/mermaid-cli`."
        );

        panic!("Mermaid CLI (`mmdc`) not found");
    }
}

fn write_mermaid_code_to_file(mermaid_code: &str, file_path: &str) {
    let mut file = File::create(file_path).expect("Unable to create file.");
    file.write_all(mermaid_code.as_bytes())
        .expect("Unable to write to file.");
}

/// Renders the `.mmd` file as a SVG using `mmdc`.
fn render_mermaid_to_svg() {
    let output = Command::new("mmdc")
        .arg("-i")
        .arg(MMD_OUTPUT)
        .arg("-o")
        .arg(SVG_OUTPUT)
        .output()
        .unwrap_or_else(|e| {
            panic!("Failed to execute mmdc: {}", e);
        });

    if !output.status.success() {
        eprintln!(
            "Error rendering diagram: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::prelude::*;
    use std::fs;

    #[test]
    fn test_write_mermaid_code_to_file() {
        let temp_file = assert_fs::NamedTempFile::new("test.mmd").unwrap();
        let test_content = "graph TD; A-->B;";
        write_mermaid_code_to_file(test_content, temp_file.path().to_str().unwrap());
        assert!(temp_file.path().exists(), "Expected file was not created.");
        assert_eq!(fs::read_to_string(temp_file.path()).unwrap(), test_content);
    }

    #[test]
    fn test_export_mmd_file() {
        let temp_dir = assert_fs::TempDir::new().unwrap();
        let mmd_output_path = temp_dir.child("workspace-analyzer.mmd");
        let test_content = "graph TD; A-->B;";
        std::env::set_current_dir(&temp_dir).expect("Failed to change directory");
        export(test_content, FileExportOptions::MMD);
        assert!(mmd_output_path.path().exists(), "MMD file was not created.");
    }
}
