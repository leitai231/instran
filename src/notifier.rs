use std::process::Command;

/// Show loading notification, return its ID for later replacement.
pub fn loading() -> Option<String> {
    let output = Command::new("notify-send")
        .args([
            "--print-id",
            "--app-name=instran",
            "instran",
            "⏳ Translating...",
        ])
        .output()
        .ok()?;

    let id = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if id.is_empty() { None } else { Some(id) }
}

/// Replace notification with success message.
pub fn success(id: Option<&str>, body: &str) {
    let mut cmd = Command::new("notify-send");
    cmd.arg("--app-name=instran");
    if let Some(id) = id {
        cmd.arg(format!("--replace-id={id}"));
    }
    cmd.args(["✅ instran", body]);
    let _ = cmd.status();
}

/// Replace notification with error message (or send standalone if no ID).
pub fn error(id: Option<&str>, msg: &str) {
    let mut cmd = Command::new("notify-send");
    cmd.args(["--urgency=critical", "--app-name=instran"]);
    if let Some(id) = id {
        cmd.arg(format!("--replace-id={id}"));
    }
    cmd.args(["❌ instran", msg]);
    let _ = cmd.status();
}
