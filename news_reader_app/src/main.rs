use colour::{dark_green, yellow};
use dotenv::dotenv;
use newsapi::{get_articles, Articles};
use std::error::Error;

/// Renders the articles in both console and HTML format.
fn render_articles(articles: &Articles) {
    // Print articles in the console
    for article in &articles.results {
        dark_green!("> {}\n", article.title);
        yellow!("- {}\n\n", article.link);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;
    let api_key = std::env::var("API_KEY")?;
    let language = "en";
    let country = "gb";
    let category = "top";
    let base_url = "https://newsdata.io/api/1/news";
    let fmt_url = format!("{}?apikey={}&language={}&country={}&category={}",
        base_url,
        api_key,
        language,
        country,
        category,
    );
    let articles = get_articles(&fmt_url)?;

    render_articles(&articles);

    Ok(())
}
