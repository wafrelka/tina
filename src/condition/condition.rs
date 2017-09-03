use std::sync::Arc;

use eew::EEW;


pub trait Condition {
	fn is_satisfied(&self, latest: &Arc<EEW>, eews: &[Arc<EEW>]) -> bool;
}
