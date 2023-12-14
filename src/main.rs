use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use serde_json::Value;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = http_client()?;

    println!("Retrieving devices from Philips Hue...");
    let response = client
        .get(format!(
            "https://{}/clip/v2/resource",
            env::var("HUE_ENDPOINT")?
        ))
        .send()
        .await?
        .json::<Value>()
        .await?;
    eprintln!("response = {:#?}", response);
    Ok(())
}

fn http_client() -> Result<Client, reqwest::Error> {
    let hue_application_key = env::var("HUE_APP_KEY").unwrap();
    let mut headers = HeaderMap::new();
    headers.insert(
        "hue-application-key",
        HeaderValue::from_str(&hue_application_key).unwrap(),
    );
    Client::builder()
        .gzip(true)
        .danger_accept_invalid_certs(true)
        .default_headers(headers)
        .build()
}
