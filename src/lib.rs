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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        let json = r#"
        {
            "data": {
                "children": [
                    {
                        "data": {
                            "title": "Title 1",
                            "permalink": "/r/rust/comments/12345/title_1"
                        }
                    },
                    {
                        "data": {
                            "title": "Title 2",
                            "permalink": "/r/rust/comments/12345/title_2"
                        }
                    }
                ]
            }
        }
        "#;

        let listing: SubredditListing = serde_json::from_str(json).unwrap();

        assert_eq!(listing.data.children.len(), 2);
        assert_eq!(listing.data.children[0].data.title, "Title 1");
        assert_eq!(
            listing.data.children[0].data.permalink,
            "/r/rust/comments/12345/title_1"
        );
    }
}
