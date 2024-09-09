use reqwest::Client;
use std::error::Error;
use std::io;

const SERVER_URL: &str = "https://127.0.0.1:8081/code";

pub async fn get_ws_code(password: String) -> Result<String, Box<dyn Error>> {
    let client = Client::new();

    let response = client
        .post(SERVER_URL)
        .header("X-Secret-Code", password)
        .send()
        .await?;

    if response.status().is_success() {
        let text = response.text().await?;
        Ok(text)
    } else {
        let text = response.text().await?.replace('"', "");
        Err(Box::new(io::Error::new(io::ErrorKind::Other, text)))
    }
}
