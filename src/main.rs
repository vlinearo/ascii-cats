#[allow(
    dead_code,
    unused_imports,
    unused_variables
)]

use tokio::*;
use reqwest::{get, Client};
use serde::Deserialize;
use color_eyre::{self, eyre::Ok, owo_colors::OwoColorize};
use pretty_hex::PrettyHex;

#[derive(Deserialize)]
struct CatImage {
    id: String,
    url: String,
    width: usize,
    height: usize
}

#[tokio::main]
async fn main() {
    let url = get_asciicat_url().await.unwrap();
    println!("URL: {}", url);
    let image_bytes = get_asciicats_bytes().await.unwrap();
    println!("{}", &image_bytes[..200].hex_dump());
}


async fn get_asciicat_url() -> color_eyre::Result<String> {
    let res = get("https://api.thecatapi.com/v1/images/search")
        .await?;
    if !res.status().is_success() {
        return Err(color_eyre::eyre::eyre!(
            "The Cat API returned HTTP {}",
            res.status()
        ));
    }
    let images: Vec<CatImage> = res.json().await?;
    let Some(image) =  images.first() else {
        return Err( color_eyre::eyre::eyre!("Cant get url from cat image!")
        )
    };
    Ok(image.url.clone())
}

async fn get_asciicats_bytes() -> color_eyre::Result<Vec<u8>> {
    let client = Client::default();
    let image = client
        .get("https://api.thecatapi.com/v1/images/search")
        .send()
        .await?.error_for_status()?
        .json::<Vec<CatImage>>()
        .await?
        .pop()
        .ok_or_else(|| color_eyre::eyre::eyre!("Error while getting cat image: "))?;

    Ok(client
        .get(image.url)
        .send()
        .await?.error_for_status()?.bytes().await?.to_vec())
}