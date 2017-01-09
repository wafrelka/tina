use std::sync::Arc;

use eew::EEW;
use destination::Destination;
use translator::format_eew_full;


pub struct Logging { }

impl Logging {

	pub fn new() -> Logging
	{
		Logging {}
	}
}

impl Destination for Logging {

	fn emit(&mut self, _: &[Arc<EEW>], latest: Arc<EEW>)
	{
		let out = format_eew_full(&latest);
		info!("{}", out);
	}
}
