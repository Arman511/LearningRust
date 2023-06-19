use dotenv::dotenv;
use newsapi::{get_articles, Articles};
use std::error::Error;
mod theme;

/// Renders the articles in both console and HTML format.
fn render_articles(articles: &Articles) {
    let theme = theme::default();
    theme.print_text("# Top headlines\n\n");
    // Print articles in the console
    for article in &articles.results {
        theme.print_text(&format!("`{}`",article.title));
        theme.print_text(&format!("> {}",article.link));
        theme.print_text("---")
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
