//!# Reddit api wrapper
//!Example program  
//!You need tokio to use this.  
//!Make sure to use `tokio = { version = "1.3.*", features = ["full"] }`
//!```rust
//!extern crate rapr;
//!extern crate tokio;
//!use crate::rapr::{RaprClient,RaSub};
//!#[tokio::main]
//!async fn main() {
//!     let client = RaprClient::new();
//!     let mut sub = RaSub::new("pics");
//!     client.fetch(10, &mut sub).await.unwrap();
//!     for post in sub.posts {
//!         println!("{}",post.title);
//!     }
//!}
mod rapr;
pub use crate::rapr::{RaPost, RaSub, RaprClient};

#[cfg(test)]
mod tests {
    use crate::rapr::{RaSub, RaprClient};
    #[tokio::test]
    async fn subreddit() {
        let client = RaprClient::new();
        let mut sub = RaSub::new("rust");
        client.fetch(10, &mut sub).await.unwrap();
        // println!("{:#?}", sub.posts.len());
        // client.fetch(5, &mut sub).await.unwrap();
        for post in sub.posts {
            println!("{:#?}", post);
        }
    }
    #[tokio::test]
    async fn title() {
        let client = RaprClient::new();
        let mut sub = RaSub::new("rust");
        client.fetch(10, &mut sub).await.unwrap();
        for post in sub.posts {
            println!("{}", post.title);
        }
    }
}
