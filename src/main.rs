#[allow(
    dead_code,
    unused_imports,
    unused_variables
)]

use tokio::*;
use reqwest::Client;
use serde::Deserialize;
use color_eyre::{self, eyre::Ok};
use image::load_from_memory;
use artem;

#[derive(Deserialize)]
struct CatImage {
    url: String,
}

#[tokio::main]
async fn main() {
    let cat = get_acii_cat().await.unwrap();
    println!("{}", cat)
}

async fn get_acii_cat() -> color_eyre::Result<String> {
    let client = Client::default();
    let image = client
        .get("https://api.thecatapi.com/v1/images/search")
        .send()
        .await?.error_for_status()?
        .json::<Vec<CatImage>>()
        .await?
        .pop()
        .ok_or_else(|| color_eyre::eyre::eyre!("Error while getting cat image: "))?;

    let bytes = client.get(image.url)
        .send().await?
        .error_for_status()?.bytes().await?;

    let image = load_from_memory(&bytes)?;
    let ascii_cat = artem::convert(image, &artem::ConfigBuilder::new().build());

    Ok(ascii_cat)
}