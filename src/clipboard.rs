use std::io::Write;
use std::process::{Command, Stdio};

/// Read text from clipboard, falling back to primary selection on Wayland.
pub fn read() -> Result<String, Box<dyn std::error::Error>> {
    if let Ok(text) = read_clipboard()
        && !text.is_empty()
    {
        return Ok(text);
    }
    if cfg!(target_os = "linux") {
        return read_primary();
    }
    Err("clipboard is empty".into())
}

fn read_clipboard() -> Result<String, Box<dyn std::error::Error>> {
    let cmd = if cfg!(target_os = "macos") { "pbpaste" } else { "wl-paste" };
    let output = Command::new(cmd).output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("{cmd}: {stderr}").into());
    }
    Ok(String::from_utf8(output.stdout)?)
}

fn read_primary() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("wl-paste").arg("--primary").output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("wl-paste: {stderr}").into());
    }
    Ok(String::from_utf8(output.stdout)?)
}

/// Write text to clipboard.
pub fn write(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let cmd = if cfg!(target_os = "macos") { "pbcopy" } else { "wl-copy" };
    let mut child = Command::new(cmd).stdin(Stdio::piped()).spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(text.as_bytes())?;
    }

    let status = child.wait()?;
    if !status.success() {
        return Err(format!("{cmd} exited with {status}").into());
    }
    Ok(())
}
