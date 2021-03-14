extern crate json;
extern crate reqwest;
extern crate tokio;
use chrono::{DateTime, Local};
use std::fmt;

#[derive(Debug)]
pub enum Error {
    JsonParseError,
    HttpGetError,
}

#[derive(Clone)]
pub struct RaPost {
    id: String,
    datetime: DateTime<Local>,
    title: String,
    text: Option<String>,
    json: json::JsonValue,
    pinned: bool,
}

impl fmt::Debug for RaPost {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RaPost")
            .field("id", &self.id)
            .field("datetime", &self.datetime)
            .field("title", &self.title)
            .field("text", &self.text)
            .finish()
    }
}

impl RaPost {
    pub fn new(id: &str, title: &str, json: &json::JsonValue, pinned: bool) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            datetime: Local::now(), // Temp
            text: Some(String::from("")),
            json: json.clone(),
            pinned,
        }
    }
    pub fn parse(post: &json::JsonValue) -> Result<RaPost, json::Error> {
        Ok(RaPost::new(
            json::stringify(post["name"].clone()).as_str(),
            json::stringify(post["title"].clone()).as_str(),
            post,
            post["pinned"].as_bool().unwrap(),
        ))
    }
}

#[derive(Debug)]
pub struct RaSub {
    pub name: String,
    pub posts: Vec<RaPost>,
    after: Option<String>,
}

impl RaSub {
    pub fn pinned_posts(&self) -> Option<Vec<RaPost>> {
        let mut pinned_posts: Vec<RaPost> = Vec::new();
        for post in &self.posts {
            if post.pinned {
                pinned_posts.push(post.clone());
            }
        }
        if pinned_posts.is_empty() {
            return None;
        } else {
            return Some(pinned_posts);
        }
    }
}

#[derive(Debug)]
pub struct RaprClient {
    oauth: Option<String>,
    rwclient: reqwest::Client,
}

impl RaprClient {
    pub fn new() -> Self {
        Self {
            oauth: None,
            rwclient: reqwest::Client::new(),
        }
    }
    pub fn subreddit(name: &str) -> RaSub {
        RaSub {
            name: String::from(name),
            posts: Vec::new(),
            after: None,
        }
    }
    pub async fn fetch(&self, count: u32, sub: &mut RaSub) -> Result<Vec<RaPost>, Error> {
        let url = match self.oauth {
            None => format!("https://reddit.com/r/{}.json", sub.name),
            Some(_) => format!("https://oauth.reddit.com/r/{}.json", sub.name),
        };

        let res = match &sub.after {
            None => self
                .rwclient
                .get(url)
                .query(&[("limit", count)])
                .send()
                .await
                .unwrap(),
            Some(after) => self
                .rwclient
                .get(url)
                .query(&[("limit", count.to_string()), ("after", after.to_string())])
                .send()
                .await
                .unwrap(),
        };

        let mut parsed = json::parse(res.text().await.unwrap().as_str()).unwrap();

        let raw_posts: Vec<json::JsonValue> = match parsed["data"]["children"].take() {
            json::JsonValue::Array(arr) => arr,
            _ => return Err(Error::JsonParseError),
        };

        let mut parsed_posts: Vec<RaPost> = Vec::new();

        for post in raw_posts {
            parsed_posts.push(RaPost::parse(&post["data"]).unwrap());
        }
        if parsed["data"]["after"].is_string() {
            sub.after = Some(parsed["data"]["after"].to_string());
        }
        sub.posts.append(&mut parsed_posts);
        Ok(parsed_posts)
    }
}
