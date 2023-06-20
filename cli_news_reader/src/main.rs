use dotenv::dotenv;
use newsapi::{self, Article, Category, Country, Languages, NewsAPI};
use std::error::Error;
mod theme;

/// Renders the articles in both console and HTML format.
fn render_articles(articles: &Vec<Article>) {
    let theme = theme::default();
    theme.print_text("# Top headlines\n\n");
    // Print articles in the console
    for article in articles {
        theme.print_text(&format!("`{}`", article.get_title()));
        theme.print_text(&format!("> {}", article.get_link()));
        theme.print_text("---")
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;
    let api_key = std::env::var("API_KEY")?;
    let mut newsapi = NewsAPI::new(&api_key);
    newsapi.category(Category::TopHeadLines);
    newsapi.country(Country::GB);
    newsapi.language(Languages::EN);
    let articles = newsapi.fetch_async().await?;
    render_articles(articles.articles());

    Ok(())
}
