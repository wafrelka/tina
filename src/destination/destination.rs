use eew::EEW;

pub trait Destination {
	fn emit(&mut self, latest: &EEW, prev: Option<&EEW>);
}
