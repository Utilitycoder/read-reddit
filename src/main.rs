use colored::*;
use read_reddit::SubredditListing;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    const REDDIT_BASE_URL: &str = "https://www.reddit.com";
    // user supplied subreddit from command line
    let subreddit = std::env::args().nth(1).unwrap_or_else(|| "rust".to_string());
    println!("Subreddit: {}", subreddit.green());

    // Construct the URL for the subreddit's hot topics
    let url = format!("{}/r/{}/hot.json", REDDIT_BASE_URL, subreddit);

    // Send the HTTP GET request
    let client = Client::new();

    // add user agent header
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::USER_AGENT,
        reqwest::header::HeaderValue::from_static("rust"),
    );
    // add headers to client
    let response_body = client
        .get(&url)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    // Parse the json
    let response: SubredditListing = serde_json::from_str(&response_body)?;

    // Process and display the hot topics
    for post in response.data.children {
        println!("{}", format!("Title: {}", post.data.title).green());
        println!(
            "{}",
            format!("Permalink: {}{}", REDDIT_BASE_URL, post.data.permalink).red()
        );
        println!("---");
    }

    Ok(())
}
