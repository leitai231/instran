mod clipboard;
mod notifier;
mod translator;

use std::process;

fn main() {
    if let Err(e) = run() {
        eprintln!("instran: {e}");
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("ANTHROPIC_API_KEY")
        .map_err(|_| "set ANTHROPIC_API_KEY")?;
    let api_url = std::env::var("INSTRAN_API_URL")
        .unwrap_or_else(|_| "https://api.anthropic.com/v1/messages".into());
    let model = std::env::var("INSTRAN_MODEL")
        .unwrap_or_else(|_| "claude-sonnet-4-20250514".into());

    let text = clipboard::read()?;
    if text.is_empty() {
        return Err("primary selection is empty".into());
    }

    let notif_id = notifier::loading();

    match translator::translate(&api_key, &api_url, &model, &text) {
        Ok(result) => {
            clipboard::write(&result)?;
            let preview: String = result.chars().take(80).collect();
            notifier::success(notif_id.as_deref(), &preview);
        }
        Err(e) => {
            notifier::error(notif_id.as_deref(), &e.to_string());
            return Err(e);
        }
    }

    Ok(())
}
