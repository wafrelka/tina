use std::sync::Arc;

use slog::Logger;

use eew::EEW;
use destination::Destination;
use translator::format_eew_full;


pub struct Logging {
	logger: Logger,
}

impl Logging {

	pub fn new(logger: Logger) -> Logging
	{
		Logging { logger: logger }
	}
}

impl Destination for Logging {

	fn emit(&mut self, _: &[Arc<EEW>], latest: Arc<EEW>)
	{
		let out = format_eew_full(&latest);
		slog_info!(self.logger, "{}", out);
	}
}
