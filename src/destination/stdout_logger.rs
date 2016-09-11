pub struct StdoutLogger { }

impl StdoutLogger {

	pub fn new() -> StdoutLogger
	{
		StdoutLogger {}
	}

	pub fn output(&self, data: &str)
	{
		println!("{}", data);
	}
}
