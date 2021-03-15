//!# Reddit api wrapper
//!Example program  
//!You need tokio to use this.  
//!Make sure to use `tokio = { version = "1.3.*", features = ["full"] }`
//!```rust
//!extern crate rapr;
//!extern crate tokio;
//!use crate::rapr::{RaprClient,RaSub};
//!#[tokio::main]
//!async fn main() -> Result<(), rapr::Error> {
//!     let client = RaprClient::new();
//!     let mut sub = RaSub::new("pics");
//!     client.fetch(10, &mut sub).await?;
//!     for post in sub.posts {
//!         println!("{}",post.title);
//!     }
//!     Ok(())
//!}
#[macro_use]
extern crate derive_error;
mod rapr;
pub use crate::rapr::Error;
pub use crate::rapr::{RaPost, RaPostItems, RaSub, RaprClient};

#[cfg(test)]
mod tests {
    use crate::rapr::{RaSub, RaprClient};
    #[tokio::test]
    async fn subreddit() -> Result<(), crate::rapr::Error> {
        let client = RaprClient::new();
        let mut sub = RaSub::new("rust");
        client.fetch(10, &mut sub).await?;
        for post in sub.posts {
            println!("{:#?}", post);
        }
        Ok(())
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
