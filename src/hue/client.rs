use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use serde_json::Value;
use std::env;
use std::env::VarError;
use thiserror::Error;

pub struct HueClient {
    client: Client,
    endpoint: String,
}

impl HueClient {
    pub fn new() -> Result<HueClient, HueClientError> {
        Ok(HueClient {
            client: HueClient::http_client()?,
            endpoint: Self::env_var("HUE_ENDPOINT".to_string())?,
        })
    }

    pub async fn fetch_devices(&self) -> Result<Value, HueClientError> {
        let response = self
            .client
            .get(format!("https://{}/clip/v2/resource", self.endpoint))
            .send()
            .await?
            .json::<Value>()
            .await?;
        eprintln!("response = {:#?}", response);
        Ok(response)
    }

    fn http_client() -> Result<Client, HueClientError> {
        let hue_application_key = Self::env_var("HUE_APP_KEY".to_string())?;
        let mut headers = HeaderMap::new();
        headers.insert(
            "hue-application-key",
            HeaderValue::from_str(&hue_application_key)
                .map_err(|_| HueClientError::InvalidHeaderValue(hue_application_key.to_string()))?,
        );
        Ok(Client::builder()
            .gzip(true)
            .danger_accept_invalid_certs(true)
            .default_headers(headers)
            .build()?)
    }

    fn env_var(key: String) -> Result<String, HueClientError> {
        env::var(&key).map_err(|e| match e {
            VarError::NotPresent => HueClientError::EnvVarNotPresent(key),
            VarError::NotUnicode(_) => HueClientError::EnvVarNotUnicode(key),
        })
    }
}

#[derive(Error, Debug)]
pub enum HueClientError {
    #[error("environment variable '{0}' not present")]
    EnvVarNotPresent(String),
    #[error("environment variable '{0}' does not contain valid unicode data")]
    EnvVarNotUnicode(String),
    #[error("invalid header value '{0}'")]
    InvalidHeaderValue(String),
    #[error(transparent)]
    RequestError(#[from] reqwest::Error),
}
