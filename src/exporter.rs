use std::fs::File;
use std::io::Write;
use std::process::Command;

const INPUT_PATH: &str = "temp_diagram.mmd";
const OUTPUT_PATH: &str = "workspace-analyzer.svg";

/// Main function to generate a PNG from Mermaid code.
/// - `mermaid_code` is the Mermaid diagram as a string.
/// - `output_path` is the path where the PNG should be saved.
pub fn generate_mermaid_png(mermaid_code: &str) {
    verify_mmdc_installation();
    println!("Generating mermaid SVG...");
    write_mermaid_code_to_file(mermaid_code, INPUT_PATH);
    render_mermaid_to_png(INPUT_PATH);
    if let Err(e) = std::fs::remove_file(INPUT_PATH) {
        println!("Failed to remove file: {}", e);
    }
    println!("Diagram rendered successfully.")
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

/// Writes the Mermaid diagram code to a temporary `.mmd` file.
fn write_mermaid_code_to_file(mermaid_code: &str, file_path: &str) {
    let mut file = File::create(file_path).expect("Unable to create temp file.");
    file.write_all(mermaid_code.as_bytes())
        .expect("Unable to write to temp file.");
    println!("Mermaid code written to {}", file_path);
}

/// Renders the `.mmd` file as a PNG using `mmdc`.
fn render_mermaid_to_png(input_path: &str) {
    let output = Command::new("mmdc")
        .arg("-i")
        .arg(input_path)
        .arg("-o")
        .arg(OUTPUT_PATH)
        .output()
        .unwrap_or_else(|e| {
            panic!("Failed to execute mmdc: {}", e);
        });

    if output.status.success() {
        println!("Diagram successfully rendered to {}", OUTPUT_PATH);
    } else {
        eprintln!(
            "Error rendering diagram: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
