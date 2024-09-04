use log::info;
use reqwest::Client;
use std::error::Error;

const SERVER_URL: &str = "https://127.0.0.1:8081/code";

pub async fn get_ws_code(password: String) -> Result<String, Box<dyn Error>> {
    let client = Client::new();

    let response = client
        .post(SERVER_URL)
        .header("X-Secret-Code", password)
        .send()
        .await?;

    if response.status().is_success() {
        info!("Server response: {}", response.text().await?);
    } else {
        info!("Failed to authorize: {}", response.status());
    }

    Ok(String::new())
}
