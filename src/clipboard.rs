use std::io::Write;
use std::process::{Command, Stdio};

/// Read text from Wayland primary selection (mouse-selected text).
pub fn read() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("wl-paste")
        .args(["--primary", "--no-newline"])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("wl-paste: {stderr}").into());
    }

    Ok(String::from_utf8(output.stdout)?)
}

/// Write text to Wayland clipboard. wl-copy forks to serve clipboard content.
pub fn write(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut child = Command::new("wl-copy")
        .stdin(Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(text.as_bytes())?;
    }

    child.wait()?;
    Ok(())
}
