mod twitter_client;
mod stdout_logger;

pub use self::twitter_client::TwitterClient;
pub use self::stdout_logger::StdoutLogger;


pub trait Destination<O> {
	fn output(&self, data: O) -> Result<(),O>;
}
