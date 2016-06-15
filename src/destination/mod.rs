mod interface;
mod twitter_client;
mod stdout_logger;

pub use self::interface::Destination;
pub use self::twitter_client::{TwitterClient, TwitterEmitter};
pub use self::stdout_logger::{StdoutLogger, StdoutLoggerEmitter};
