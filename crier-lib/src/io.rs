use atom_syndication::Feed;

pub enum FeedMethod {
    Read,
    Create,
    Update,
}

pub trait FeedGet {
    fn get(&self, s: &str, method: Option<FeedMethod>) -> Result<Feed, u64>;
}

pub trait FeedPut {
    fn put(&self, feed: &Feed, s: &str, method: Option<FeedMethod>) -> u64;
}

#[cfg(feature = "fs")]
pub mod fs;
