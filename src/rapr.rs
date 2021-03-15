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

#[derive(Debug, Clone, Copy)]
pub enum TextType {
    HTML,
    Raw,
    Empty,
}

/// Reddit post object
/// (more like repost object).  
/// Keeps track of post information.
#[derive(Clone)]
pub struct RaPost {
    pub id: String,
    pub datetime: DateTime<Local>,
    pub title: String,
    pub text: Option<String>,
    texttype: TextType,
    pub permalink: String,
    pub upvotes: u32,
    pub downvotes: u32,
    pub json: json::JsonValue,
}

impl fmt::Debug for RaPost {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RaPost")
            .field("id", &self.id)
            .field("datetime", &self.datetime)
            .field("title", &self.title)
            .field("text", &self.text)
            .field("texttype", &self.texttype)
            .field("upvotes", &self.upvotes)
            .field("downvotes", &self.downvotes)
            .finish()
    }
}

impl RaPost {
    /// Generate new post object (you probably don't need this).
    pub fn new(
        id: &str,
        title: &str,
        text: Option<&str>,
        texttype: TextType,
        permalink: &str,
        upvotes: u32,
        downvotes: u32,
        json: &json::JsonValue,
    ) -> Self {
        let _text = match text {
            Some(text) => Some(text.to_string()),
            None => None,
        };
        Self {
            id: id.to_string(),
            title: title.to_string(),
            datetime: Local::now(), // Temp
            text: _text,
            texttype,
            permalink: String::from(permalink),
            upvotes,
            downvotes,
            json: json.clone(),
        }
    }
    /// Parse json from `json['data']['children']` array elements.
    pub fn parse(post: &json::JsonValue) -> Result<RaPost, json::Error> {
        let mut text: Option<&str> = post["selftext"].as_str();
        let mut texttype: TextType = TextType::Empty;
        // Reddit always returns an empty string on selftext
        // if there is no text. So this shouldn't panic!
        // Some also have both selftext and selftext_html
        // I am only taking selftext from these.
        if text.unwrap().is_empty() {
            text = post["selftext_html"].as_str();
            if text != None {
                texttype = TextType::HTML;
            }
        } else {
            texttype = TextType::Raw;
        }
        Ok(RaPost::new(
            post["name"].as_str().unwrap(),
            post["title"].as_str().unwrap(),
            text,
            texttype,
            post["permalink"].as_str().unwrap(),
            post["ups"].as_u32().unwrap(),
            post["downs"].as_u32().unwrap(),
            post,
        ))
    }
}

///# RaSub
///Subreddit object   
///Keeps track of posts
#[derive(Debug)]
pub struct RaSub {
    pub name: String,
    pub posts: Vec<RaPost>,
    after: Option<String>,
}

impl RaSub {
    /// Generate subreddit object
    pub fn new(name: &str) -> RaSub {
        RaSub {
            name: String::from(name),
            posts: Vec::new(),
            after: None,
        }
    }
}

/// Reddit api client.  
/// Uses a [reqwest::Client](https://docs.rs/reqwest/0.11.2/reqwest/struct.Client.html) internally.  
/// Currently no authentication
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

    /// Fetch posts from subreddit and store them in the subreddit object.
    /// Note: First fetch always seems to pull two pinned posts which are not marked pinned in the json
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
            if post["kind"].as_str().unwrap() == "t3" {
                parsed_posts.push(RaPost::parse(&post["data"]).unwrap());
            }
        }
        if parsed["data"]["after"].is_string() {
            sub.after = Some(parsed["data"]["after"].to_string());
        }
        sub.posts.append(&mut parsed_posts);
        Ok(parsed_posts)
    }
}
