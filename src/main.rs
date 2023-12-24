use std::error::Error;

use crate::hue::client::HueClient;

mod event;
mod hue;
mod model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Retrieving devices from Philips Hue...");
    let client = HueClient::new()?;
    let response = client.fetch_devices().await?;
    eprintln!("response = {:#?}", response);
    Ok(())
}
