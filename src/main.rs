use reqwest::Error;
use scraper::{Html, Selector};
use std::collections::HashSet;
use url::Url;
use dashmap::DashSet;
use tokio::task;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let start_url = "https://www.rust-lang.org/"; // Seed URL
    let max_depth = 3; // Maximum crawling depth

    // Shared state to track visited URLs
    let visited = Arc::new(DashSet::new());

    // Start crawling
    crawl(start_url, max_depth, &visited).await;

    println!("Crawling completed.");
    Ok(())
}

async fn crawl(start_url: &str, max_depth: usize, visited: &DashSet<String>) {
    let visited = Arc::new(visited.clone());
    let mut queue = vec![(start_url.to_string(), max_depth)];

    while let Some((url, depth)) = queue.pop() {
        if depth == 0 || visited.contains(&url) {
            continue;
        }

        // Mark the URL as visited
        visited.insert(url.clone());

        // Fetch and parse the page
        match fetch_page(&url).await {
            Ok(html) => {
                println!("Crawled: {}", url);

                // Extract links from the page
                let links = extract_links(&html, &url);

                // Add links to the queue for further crawling
                for link in links {
                    if !visited.contains(&link) {
                        queue.push((link, depth - 1));
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to fetch {}: {}", url, e);
            }
        }
    }
}

async fn fetch_page(url: &str) -> Result<String, Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

fn extract_links(html: &str, base_url: &str) -> HashSet<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("a[href]").unwrap();
    let base_url = Url::parse(base_url).unwrap();

    let mut links = HashSet::new();
    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            if let Ok(url) = base_url.join(href) {
                if url.scheme() == "http" || url.scheme() == "https" {
                    links.insert(url.to_string());
                }
            }
        }
    }
    links
}