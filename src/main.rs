use tokio::*;
use reqwest::get;

#[tokio::main]
async fn main() {
    let res = get("https://api.thecatapi.com/v1/images/search")
        .await.unwrap();
    println!("Status: {}", res.status());
    let body = res.text().await.unwrap();
    println!("Body: {}", body);
}
