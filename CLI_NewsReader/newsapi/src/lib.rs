use serde::Deserialize;

#[derive(thiserror::Error, Debug)]
pub enum NewsApiError {
    #[error("Failed fetching articles")]
    RequestFailed(ureq::Error),
    #[error("Failed Conversion of response to string")]
    FailedResponseToString(std::io::Error),
    #[error("Article Parsing Failed")]
    FailedParsing(serde_json::Error),
}

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
pub fn get_articles(url: &str) -> Result<Articles, NewsApiError> {
    let response = ureq::get(url)
        .call()
        .map_err(|e| NewsApiError::RequestFailed(e))?
        .into_string().map_err(|e| NewsApiError::FailedResponseToString(e))?;
    let articles: Articles = serde_json::from_str(&response).map_err(|e| NewsApiError::FailedParsing(e))?;

    Ok(articles)
}
