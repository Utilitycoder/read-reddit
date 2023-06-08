use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SubredditListing {
    pub data: SubredditChildren,
}

#[derive(Debug, Deserialize)]
pub struct SubredditChildren {
    pub children: Vec<RedditPost>,
}

#[derive(Debug, Deserialize)]
pub struct RedditPost {
    pub data: PostData,
}

#[derive(Debug, Deserialize)]
pub struct PostData {
    pub title: String,
    pub permalink: String,
}
