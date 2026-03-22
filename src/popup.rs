use std::process::{Command, Stdio};

/// Show translation result in a floating ghostty popup.
/// The window is fire-and-forget — instran exits immediately.
pub fn show(text: &str) {
    let script = format!(
        r#"cat <<'INSTRAN_EOF'
{}
INSTRAN_EOF
echo
echo -e '\033[2m[Press any key to close]\033[0m'
read -n1 -s"#,
        text
    );

    let _ = Command::new("ghostty")
        .args(["--title=instran-popup", "-e", "bash", "-c", &script])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();
}
