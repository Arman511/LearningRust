use colour::{dark_green, yellow};
use newsapi::{Articles, get_articles};
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
    let url =
        "https://newsdata.io/api/1/news?apikey=pub_2476866e13bc3820611c19181e841a1c783e4&language=en&country=gb&category=technology,top";
    let articles = get_articles(url)?;

    render_articles(&articles);

    Ok(())
}
