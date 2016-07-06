mod twitter_client;
mod stdout_logger;

pub use self::twitter_client::TwitterClient;
pub use self::stdout_logger::StdoutLogger;


pub enum OutputError<O> {
	Retriable(O),
	Unrecoverable
}

pub trait Destination<O> : Send {
	fn output(&self, data: O) -> Result<(),OutputError<O>>;
}
