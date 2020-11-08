use select::document::Document;
use select::predicate::Name;
use reqwest::Url;
use postgres::{Client, NoTls};
use std::collections::HashSet;

fn main() {
        get_page();
}


#[tokio::main]
async fn get_page() -> Result<(), reqwest::Error> {
    // let mut client = Client::connect("host=localhost user=postgres", NoTls)?;

    let res = reqwest::get("https://news.ycombinator.com/").await?;
   // println!("Body:\n\n{}", res.text().await?);
    let x =  res.text().await?;
    get_new_links(x);
    Ok(())
}

fn get_new_links(html: String) {
    let found_urls = Document::from(html.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .filter(|n| n.contains("//"))
        .map(|u| Url::parse(u).unwrap())
        .collect::<HashSet<reqwest::Url>>();

    let urls = found_urls.iter()
        .filter_map(|x| x.host_str())
        .collect::<HashSet<&str>>();

    println!("{:?}", &urls);
}