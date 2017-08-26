use std::sync::Arc;

use eew::EEW;


pub trait Condition {
	fn is_satisfied(&self, eews: &[Arc<EEW>]) -> bool;
}
