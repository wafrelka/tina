use std::sync::Arc;

use eew::EEW;


pub trait Condition {
	fn is_satisfied(&self, latest: &Arc<EEW>, prevs: &[Arc<EEW>]) -> bool;
}
