use reqwest::{blocking::Client, header::AUTHORIZATION};

/// Just an empty serde_json array
pub const EMPTY_JSON_ARRAY: &serde_json::Value = &serde_json::json!([]);

/// Pushes `[]` as commands to Discord's API route
pub fn remove_commands(application_id: &str, bot_token: &str) -> Result<(), reqwest::Error> {
    let client = Client::new();

    let url = format!(
        "https://discord.com/api/v10/applications/{}/commands",
        application_id
    );

    let res = client
        .put(url)
        .header(AUTHORIZATION, format!("Bot {}", bot_token))
        .json(EMPTY_JSON_ARRAY)
        .send()?;

    res.error_for_status()?;

    Ok(())
}
