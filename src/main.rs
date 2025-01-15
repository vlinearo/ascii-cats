#[allow(
    dead_code,
    unused_imports,
    unused_variables
)]

use tokio::*;
use reqwest::get;
use serde::Deserialize;
use color_eyre::{self, eyre::Ok};


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