use std::io::Write;
use std::process::{Command, Stdio};

/// Read text from Wayland clipboard (Super+C), falling back to primary selection (mouse select).
pub fn read() -> Result<String, Box<dyn std::error::Error>> {
    if let Ok(text) = read_clipboard()
        && !text.is_empty()
    {
        return Ok(text);
    }
    read_primary()
}

fn read_clipboard() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("wl-paste").output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("wl-paste: {stderr}").into());
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

/// Write text to Wayland clipboard. wl-copy forks to serve clipboard content.
pub fn write(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut child = Command::new("wl-copy").stdin(Stdio::piped()).spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(text.as_bytes())?;
    }

    let status = child.wait()?;
    if !status.success() {
        return Err(format!("wl-copy exited with {status}").into());
    }
    Ok(())
}
