use std::error::Error;

struct Articles{
    articles: Vec<Article>,
}

struct Article{
    title: String,
    link: String,
}

fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>>{
    let response = ureq::get(url).call()?.into_string()?;
    dbg!(response);

    todo!()
}


fn main(){
    // Create the request URL and parameters
    let url = "https://newsdata.io/api/1/news?apikey=pub_2476866e13bc3820611c19181e841a1c783e4&language=en&country=gb";
    let articles = get_articles(url);
    
}
