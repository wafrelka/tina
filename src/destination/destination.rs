use eew::EEW;

pub trait Destination {

	const WAKE_TIMEOUT_SECS: u64 = 120;
	fn emit(&mut self, latest: &EEW, prev: Option<&EEW>);
	fn wake(&mut self) { }

}
