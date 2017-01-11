use std::sync::Arc;

use eew::EEW;


pub trait Destination {
	fn emit(&mut self, eews: &[Arc<EEW>], latest: Arc<EEW>);
}
