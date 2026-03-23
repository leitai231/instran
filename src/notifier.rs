use std::process::{Command, Stdio};

/// Show loading notification, return its ID for later replacement.
pub fn loading() -> Option<String> {
    if cfg!(target_os = "macos") {
        let child = Command::new("osascript")
            .args([
                "-e",
                "display dialog \"⏳ Translating...\" with title \"instran\" buttons {\"OK\"} default button 1 giving up after 60",
            ])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .ok()?;
        return Some(child.id().to_string());
    }
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
pub fn success(id: Option<&str>) {
    if cfg!(target_os = "macos") {
        kill_dialog(id);
        return;
    }
    let mut cmd = Command::new("notify-send");
    cmd.arg("--app-name=instran");
    if let Some(id) = id {
        cmd.arg(format!("--replace-id={id}"));
    }
    cmd.args(["✅ instran", "Done"]);
    let _ = cmd.status();
}

/// Replace notification with error message (or send standalone if no ID).
pub fn error(id: Option<&str>, msg: &str) {
    if cfg!(target_os = "macos") {
        kill_dialog(id);
        return;
    }
    let mut cmd = Command::new("notify-send");
    cmd.args(["--urgency=critical", "--app-name=instran"]);
    if let Some(id) = id {
        cmd.arg(format!("--replace-id={id}"));
    }
    cmd.args(["❌ instran", msg]);
    let _ = cmd.status();
}

/// Kill a macOS dialog process by PID.
fn kill_dialog(id: Option<&str>) {
    if let Some(pid) = id {
        let _ = Command::new("kill").arg(pid).status();
    }
}
