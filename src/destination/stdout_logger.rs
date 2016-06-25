use destination::interface::Destination;


pub struct StdoutLogger { }

impl StdoutLogger {

	pub fn new() -> StdoutLogger
	{
		StdoutLogger {}
	}
}

impl Destination<String> for StdoutLogger {

	fn output(&self, data: String) -> Result<(), String>
	{
		println!("{}", data);
		return Ok(());
	}
}
