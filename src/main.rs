use select::document::Document;
use select::predicate::Name;
use reqwest::Url;

fn main() {
        get_page();
}


#[tokio::main]
async fn get_page() -> Result<(), reqwest::Error> {
    let res = reqwest::get("https://news.ycombinator.com/").await?;
   // println!("Body:\n\n{}", res.text().await?);
    let x =  res.text().await?;
    get_new_links(x);
    Ok(())
}

fn get_new_links(html: String) {
    let document = Document::from(&*html).find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .filter(|n| n.contains("http"))
        .map(|u| Url::parse(u))
        .for_each(|x| println!("{:?}", x));
}