mod interface;
mod twitter_client;

pub use self::interface::Destination;
pub use self::twitter_client::{TwitterClient, TwitterEmitter};
