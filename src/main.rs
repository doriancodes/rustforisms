use serde::Deserialize;
use reqwest::Error;

#[derive(Deserialize, Debug)]
struct Quote {
    q: String,
    a: String
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "https://zenquotes.io/api/random";
    let res = reqwest::get(url).await?;

    let quote: Vec<Quote> = res.json().await?;
    println!("Aphorism: {}", quote[0].q);
    println!("Author: {}", quote[0].a);
    Ok(())
}
