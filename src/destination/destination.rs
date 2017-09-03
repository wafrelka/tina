use std::sync::Arc;

use eew::EEW;


pub trait Destination {
	fn emit(&mut self, latest: &Arc<EEW>, eews: &[Arc<EEW>]);
}
