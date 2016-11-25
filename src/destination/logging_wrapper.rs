pub struct LoggingWrapper { }

impl LoggingWrapper {

	pub fn new() -> LoggingWrapper
	{
		LoggingWrapper {}
	}

	pub fn output(&self, data: &str)
	{
		info!("{}", data);
	}
}
