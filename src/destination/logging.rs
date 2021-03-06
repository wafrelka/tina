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

	fn emit(&mut self, latest: &EEW, _: Option<&EEW>)
	{
		let out = format_eew_full(latest);
		slog_info!(self.logger, "{}", out.trim_right_matches('\n'));
	}
}
