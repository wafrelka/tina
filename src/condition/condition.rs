use eew::EEW;

pub trait Condition {
	fn is_satisfied(&self, latest: &EEW, prev: Option<&EEW>) -> bool;
}
