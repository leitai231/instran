use std::io::Write;
use std::process::{Command, Stdio};

/// Show translation result in a popup. macOS uses editable osascript dialog, Linux uses ghostty.
/// Fire-and-forget — instran exits immediately.
pub fn show(text: &str) {
    if cfg!(target_os = "macos") {
        let script = r#"on run argv
    set theText to item 1 of argv
    set result to display dialog "" default answer theText with title "instran" buttons {"Close", "Copy"} default button "Copy" cancel button "Close"
    if button returned of result is "Copy" then
        set the clipboard to (text returned of result)
    end if
end run"#;

        let mut child = match Command::new("osascript")
            .args(["-", text])
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
        {
            Ok(c) => c,
            Err(_) => return,
        };
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(script.as_bytes());
        }
        return;
    }

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
