extern crate url_crawler;
use url_crawler::*;
use std::sync::Arc;

/// Function for filtering content in the crawler before a HEAD request.
///
/// Only allow directory entries, and files that have the `deb` extension.

pub struct Parser {

}

impl Parser {
    fn apt_filter(url: &Url) -> bool {
        let url = url.as_str();
        url.ends_with("/") || url.ends_with(".deb")
    }

    pub fn run(url: String) {
        // Create a crawler designed to crawl the given website.
        let crawler = Crawler::new(url.to_owned())
            // Use four threads for fetching
            .threads(4)
            // Check if a URL matches this filter before performing a HEAD request on it.
            .pre_fetch(Arc::new(apt_filter))
            // Initialize the crawler and begin crawling. This returns immediately.
            .crawl();

        // Process url entries as they become available
        for file in crawler {
            println!("{:#?}", file);
        }
    }
}