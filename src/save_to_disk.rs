use std::fs::File;
use std::io;
use std::io::Write;
use std::process::Command;

/// Main function to generate a PNG from Mermaid code.
/// - `mermaid_code` is the Mermaid diagram as a string.
/// - `output_path` is the path where the PNG should be saved.
pub fn generate_mermaid_png(mermaid_code: &str, output_path: &str) -> io::Result<()> {
    let input_path = "temp_diagram.mmd";

    // Write the Mermaid code to a temporary file
    write_mermaid_code_to_file(mermaid_code, input_path)?;

    // Render the file to a PNG
    render_mermaid_to_png(input_path, output_path)?;

    // Clean up the temporary file
    std::fs::remove_file(input_path)?;
    Ok(())
}


/// Writes the Mermaid diagram code to a temporary `.mmd` file.
fn write_mermaid_code_to_file(mermaid_code: &str, file_path: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(mermaid_code.as_bytes())?;
    println!("Mermaid code written to {}", file_path);
    Ok(())
}

/// Renders the `.mmd` file as a PNG using `mmdc`.
fn render_mermaid_to_png(input_path: &str, output_path: &str) -> io::Result<()> {
    let output = Command::new("mmdc")
        .arg("-i")
        .arg(input_path)
        .arg("-o")
        .arg(output_path)
        .output()?;

    if output.status.success() {
        println!("Diagram successfully rendered to {}", output_path);
    } else {
        eprintln!(
            "Error rendering diagram: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}


