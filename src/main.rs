use tokio::*;
use reqwest::get;
use serde::Deserialize;


#[derive(Deserialize)]
struct CatImage {
    id: String,
    url: String,
    width: usize,
    height: usize
}

#[tokio::main]
async fn main() {
    let res = get("https://api.thecatapi.com/v1/images/search")
        .await.expect("Request was failed");
    let images: Vec<CatImage> = res.json()
        .await.unwrap();
    let image = images.first().unwrap();
    println!("{}", image.url)
}
