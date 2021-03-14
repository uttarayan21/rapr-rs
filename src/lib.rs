mod rapr;
pub use crate::rapr::RaprClient;

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn subreddit() {
        use crate::rapr::RaprClient;
        let client = RaprClient::new();
        let mut sub = RaprClient::subreddit("rust");
        client.fetch(10, &mut sub).await.unwrap();
        println!("{:#?}", sub.posts.len());
        client.fetch(5, &mut sub).await.unwrap();
        println!("{:#?}", sub.posts.len());
        println!("{:#?}", sub.pinned_posts());
    }
}
