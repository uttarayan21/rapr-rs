extern crate json;
extern crate reqwest;
extern crate tokio;
use std::fmt;

/// Error enum
#[derive(Debug, Error)]
pub enum Error {
    UnexpectedJson,
    NoneError,
    JsonParseError(json::Error),
    RequestError(reqwest::Error),
}

/// Items related to the post
#[derive(Debug, Clone)]
pub struct RaPostItems {
    pub upvotes: u32,
    pub downvotes: u32,
    pub permalink: String,
    pub url: Option<String>,
}

impl RaPostItems {
    /// Create a new struct of items for a post  
    /// You probably don't need this fucntion.
    pub fn new(upvotes: u32, downvotes: u32, permalink: &str, url: Option<String>) -> Self {
        Self {
            upvotes,
            downvotes,
            permalink: permalink.to_string(),
            url,
        }
    }
}

/// Reddit post object
/// (more like repost object).  
/// Keeps track of post information.
#[derive(Clone)]
pub struct RaPost {
    pub id: String,
    pub title: String,
    pub text: Option<String>,
    pub items: RaPostItems,
    pub json: json::JsonValue,
}

impl fmt::Debug for RaPost {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RaPost")
            .field("id", &self.id)
            .field("title", &self.title)
            .field("text", &self.text)
            .field("permalink", &self.items.permalink)
            .field("url", &self.items.url)
            .finish()
    }
}

impl RaPost {
    /// Generate new post object (you probably don't need this).
    pub fn new(
        id: &str,
        title: &str,
        text: Option<&str>,
        items: RaPostItems,
        json: &json::JsonValue,
    ) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            text: text.map(String::from),
            items,
            json: json.clone(),
        }
    }

    /// Parse json from `json['data']['children']` array elements.
    /// Note: In case of url mission reddit make url the permalink
    pub fn parse(post: &json::JsonValue) -> Result<RaPost, Error> {
        let id = post["name"].as_str().ok_or(Error::UnexpectedJson)?;
        let title = post["title"].as_str().ok_or(Error::UnexpectedJson)?;
        let upvotes = post["ups"].as_u32().ok_or(Error::UnexpectedJson)?;
        let downvotes = post["downs"].as_u32().ok_or(Error::UnexpectedJson)?;
        let permalink = post["permalink"].as_str().ok_or(Error::UnexpectedJson)?;
        let url = post["url"].as_str().map(String::from);

        let mut selftext = post["selftext"].as_str();
        if selftext.ok_or(Error::NoneError)?.is_empty() {
            selftext = post["selftext_html"].as_str();
        }
        let items = RaPostItems::new(upvotes, downvotes, permalink, url);

        Ok(RaPost::new(id, title, selftext, items, post))
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

impl From<&str> for RaSub {
    fn from(name: &str) -> Self {
        Self {
            name: String::from(name),
            posts: Vec::new(),
            after: None,
        }
    }
}
impl RaSub {
    /// Generate subreddit object
    pub fn new(name: &str) -> Self {
        Self::from(name)
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

impl Default for RaprClient {
    /// Return the default RaprClient
    fn default() -> Self {
        Self::new()
    }
}

impl RaprClient {
    /// Make a RaprClient
    pub fn new() -> Self {
        Self {
            oauth: None,
            rwclient: reqwest::Client::new(),
        }
    }
    /// Make a RaprClient with a custom user_agent (since reddit ratelimits same user agent being
    /// used a ton of times)
    pub fn with_user_agent(user_agent: &str) -> Result<Self, Error> {
        Ok(Self {
            oauth: None,
            rwclient: reqwest::Client::builder().user_agent(user_agent).build()?,
        })
    }
    pub fn default() -> Self {
        Self::new()
    }

    /// Fetch posts from subreddit and store them in the subreddit object.  
    /// Note: First fetch always seems to pull two pinned posts which are not marked pinned in the json
    pub async fn fetch(&self, count: u32, sub: &mut RaSub) -> Result<Vec<RaPost>, Error> {
        let url = match self.oauth {
            None => format!("https://reddit.com/r/{}.json", sub.name),
            Some(_) => format!("https://oauth.reddit.com/r/{}.json", sub.name),
        };

        let res = match &sub.after {
            None => {
                self.rwclient
                    .get(url)
                    .query(&[("limit", count), ("raw_json", 1)])
                    .send()
                    .await?
            }
            Some(after) => {
                self.rwclient
                    .get(url)
                    .query(&[
                        ("limit", count.to_string().as_str()),
                        ("after", after.as_str()),
                        ("raw_json", "1"),
                    ])
                    .send()
                    .await?
            }
        };

        let mut parsed = json::parse(res.text().await?.as_str())?;

        let raw_posts: Vec<json::JsonValue> = match parsed["data"]["children"].take() {
            json::JsonValue::Array(arr) => arr,
            _ => return Err(Error::UnexpectedJson),
        };

        let mut parsed_posts: Vec<RaPost> = Vec::new();

        for post in raw_posts {
            if post["kind"].as_str().ok_or(Error::NoneError)? == "t3" {
                // 't3' is the 'post' type in reddit api
                parsed_posts.push(RaPost::parse(&post["data"])?);
            }
        }
        if parsed["data"]["after"].is_string() {
            sub.after = Some(parsed["data"]["after"].to_string());
        }
        sub.posts.append(&mut parsed_posts);
        Ok(parsed_posts)
    }
}
