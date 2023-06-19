use colour::{dark_green, yellow};
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

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
fn render_articles(articles: &Articles) -> String {
    // Print articles in the console
    for article in &articles.results {
        dark_green!("> {}\n", article.title);
        yellow!("- {}\n\n", article.link);
    }

    // Generate HTML code
    let mut html = String::new();
    html.push_str("<html>\n");
    html.push_str("<head>\n");
    html.push_str("<link rel=\"stylesheet\" type=\"text/css\" href=\"style.css\">\n");
    html.push_str("</head>\n");
    html.push_str("<body>\n");
    for article in &articles.results {
        html.push_str("<h2>");
        html.push_str(&article.title);
        html.push_str("</h2>\n");
        html.push_str("<a href=\"");
        html.push_str(&article.link);
        html.push_str("\">");
        html.push_str(&article.link);
        html.push_str("</a><br><br>\n");
    }
    html.push_str("</body></html>");

    html
}

fn main() -> Result<(), Box<dyn Error>> {
    let url =
        "https://newsdata.io/api/1/news?apikey=pub_2476866e13bc3820611c19181e841a1c783e4&language=en&country=gb&category=technology,top";
    let articles: Articles = get_articles(url)?;

    let html = render_articles(&articles);

    // Create and write HTML content to a file
    let mut file = File::create("articles.html")?;
    file.write_all(html.as_bytes())?;

    Ok(())
}
