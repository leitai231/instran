use serde::Deserialize;

const SYSTEM_PROMPT: &str = "\
You are an expert translator. Automatically detect the source language. \
If it is English, translate to Chinese. If it is Chinese, translate to English. \
For other languages, translate to English. \
ONLY output the final translated text. \
Do not add any explanations, notes, or conversational filler. \
Keep original formatting.";

#[derive(Deserialize)]
struct Response {
    content: Vec<ContentBlock>,
    stop_reason: Option<String>,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
enum ContentBlock {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(other)]
    Other,
}

pub fn translate(
    api_key: &str,
    api_url: &str,
    model: &str,
    text: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let body = serde_json::json!({
        "model": model,
        "max_tokens": 4096,
        "temperature": 0,
        "system": SYSTEM_PROMPT,
        "messages": [{"role": "user", "content": text}]
    });

    let agent = ureq::AgentBuilder::new()
        .timeout_connect(std::time::Duration::from_secs(10))
        .timeout_read(std::time::Duration::from_secs(30))
        .build();

    let resp = agent
        .post(api_url)
        .set("x-api-key", api_key)
        .set("anthropic-version", "2023-06-01")
        .set("content-type", "application/json")
        .send_json(&body);

    match resp {
        Ok(resp) => {
            let result: Response = resp.into_json()?;
            let text: String = result
                .content
                .into_iter()
                .filter_map(|b| match b {
                    ContentBlock::Text { text } => Some(text),
                    ContentBlock::Other => None,
                })
                .collect::<Vec<_>>()
                .join("");
            if text.is_empty() {
                return Err("empty response from API".into());
            }
            if result.stop_reason.as_deref() == Some("max_tokens") {
                return Err("translation truncated (text too long)".into());
            }
            Ok(text)
        }
        Err(ureq::Error::Status(code, resp)) => {
            let body = resp.into_string().unwrap_or_default();
            Err(format!("API {code}: {body}").into())
        }
        Err(e) => Err(Box::new(e) as Box<dyn std::error::Error>),
    }
}
