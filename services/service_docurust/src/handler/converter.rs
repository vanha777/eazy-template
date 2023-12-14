use base64::decode;
use lib_errors::Errors;
use std::io::Write;
use std::process::{Command, Stdio};

// Function to convert HTML to PDF
pub async fn convert_html_to_pdf_base64(base64_html: String) -> Result<Vec<u8>, Errors> {
    // Decode the Base64-encoded HTML string
    let html_bytes = match decode(&base64_html) {
        Ok(bytes) => bytes,
        Err(_) => return Err(Errors::Error("Invalid Base64-encoded HTML".to_string())),
    };

    let mut child = Command::new("wkhtmltopdf")
        .args(&["--quiet", "-", "-"]) // Read from stdin, write to stdout
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| Errors::Error(e.to_string()))?;

    // Write decoded HTML content to stdin of wkhtmltopdf process
    if let Some(stdin) = child.stdin.as_mut() {
        stdin
            .write_all(&html_bytes)
            .map_err(|e| Errors::Error(e.to_string()))?;
    }

    // Wait for the process to complete and capture the output
    let output = child
        .wait_with_output()
        .map_err(|e| Errors::Error(e.to_string()))?;

    if !output.status.success() {
        return Err(Errors::Error("Failed to convert HTML to PDF".to_string()));
    }

    Ok(output.stdout)
}

pub fn local_test_bytes() -> Result<(), Errors> {
    // Execute the command
    let status = Command::new("wkhtmltopdf")
        .arg("input.html")
        .arg("output.pdf")
        .status()
        .map_err(|e| Errors::Error(e.to_string()))?;

    // Check if the command was successful
    if status.success() {
        println!("PDF successfully created.");
    } else {
        eprintln!("Failed to create PDF.");
    }

    Ok(())
}
