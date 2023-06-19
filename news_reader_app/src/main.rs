use colour::{dark_green, yellow};
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct Articles {
    results: Vec<Article>,
}

#[derive(Deserialize, Debug)]
struct Article {
    title: String,
    link: String,
}

/// Fetches articles from the provided URL and deserializes the response into the `Articles` struct.
fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>> {
    let response = ureq::get(url).call()?.into_string()?;
    let articles: Articles = serde_json::from_str(&response)?;

    Ok(articles)
}

/// Renders the articles in both console and HTML format.
fn render_articles(articles: &Articles) {
    // Print articles in the console
    for article in &articles.results {
        dark_green!("> {}\n", article.title);
        yellow!("- {}\n\n", article.link);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let url =
        "https://newsdata.io/api/1/news?apikey=pub_2476866e13bc3820611c19181e841a1c783e4&language=en&country=gb&category=technology,top";
    let articles: Articles = get_articles(url)?;

    render_articles(&articles);

    Ok(())
}
