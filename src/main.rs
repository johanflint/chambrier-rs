use std::error::Error;

use crate::hue::{HueClient, HueObserver};

mod event;
mod hue;
mod model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Retrieving devices from Philips Hue...");
    let client = HueClient::new()?;
    let observer = HueObserver::new(client);
    let response = observer.observe().await?;
    eprintln!("response = {:#?}", response);
    Ok(())
}
