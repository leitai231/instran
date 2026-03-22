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
    let api_key = std::env::var("INSTRAN_API_KEY")
        .or_else(|_| std::env::var("ANTHROPIC_API_KEY"))
        .map_err(|_| "set INSTRAN_API_KEY or ANTHROPIC_API_KEY")?;
    let api_url = std::env::var("INSTRAN_API_URL")
        .unwrap_or_else(|_| "https://api.anthropic.com/v1/messages".into());
    let model =
        std::env::var("INSTRAN_MODEL").unwrap_or_else(|_| "claude-sonnet-4-20250514".into());

    let text = clipboard::read()?;
    if text.is_empty() {
        return Err("primary selection is empty".into());
    }

    let notif_id = notifier::loading();

    let result = match translator::translate(&api_key, &api_url, &model, &text) {
        Ok(r) => r,
        Err(e) => {
            notifier::error(notif_id.as_deref(), &e.to_string());
            return Err(e);
        }
    };

    if let Err(e) = clipboard::write(&result) {
        notifier::error(notif_id.as_deref(), &e.to_string());
        return Err(e);
    }

    let preview: String = result.chars().take(80).collect();
    notifier::success(notif_id.as_deref(), &preview);

    Ok(())
}
