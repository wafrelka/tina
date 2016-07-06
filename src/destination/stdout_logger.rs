use destination::{OutputError, Destination};


pub struct StdoutLogger { }

impl StdoutLogger {

	pub fn new() -> StdoutLogger
	{
		StdoutLogger {}
	}
}

impl Destination<String> for StdoutLogger {

	fn output(&self, data: String) -> Result<(), OutputError<String>>
	{
		println!("{}", data);
		return Ok(());
	}
}
