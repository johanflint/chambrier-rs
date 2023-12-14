use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use serde_json::Value;
use std::env;
use std::error::Error;

pub struct HueClient {
    client: Client,
}

impl HueClient {
    pub fn new() -> Result<HueClient, Box<dyn Error>> {
        Ok(HueClient {
            client: HueClient::http_client()?,
        })
    }

    pub async fn fetch_devices(&self) -> Result<Value, Box<dyn Error>> {
        let response = self
            .client
            .get(format!(
                "https://{}/clip/v2/resource",
                env::var("HUE_ENDPOINT")?
            ))
            .send()
            .await?
            .json::<Value>()
            .await?;
        eprintln!("response = {:#?}", response);
        Ok(response)
    }

    fn http_client() -> Result<Client, Box<dyn Error>> {
        let hue_application_key = env::var("HUE_APP_KEY")?;
        let mut headers = HeaderMap::new();
        headers.insert(
            "hue-application-key",
            HeaderValue::from_str(&hue_application_key)?,
        );
        Ok(Client::builder()
            .gzip(true)
            .danger_accept_invalid_certs(true)
            .default_headers(headers)
            .build()?)
    }
}
