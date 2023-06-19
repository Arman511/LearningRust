use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct Articles {
    pub results: Vec<Article>,
}

#[derive(Deserialize, Debug)]
pub struct Article {
    pub title: String,
    pub link: String,
}

/// Fetches articles from the provided URL and deserializes the response into the `Articles` struct.
pub fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>> {
    let response = ureq::get(url).call()?.into_string()?;
    let articles: Articles = serde_json::from_str(&response)?;

    Ok(articles)
}
