use std::sync::Arc;

use collections::IndexedLimitedQueue;
use eew::EEW;

#[derive(Debug, Clone)]
pub struct EEWHistory {
	q: IndexedLimitedQueue<Vec<Arc<EEW>>>,
}

impl EEWHistory {

	pub fn new(limit: usize) -> EEWHistory
	{
		EEWHistory { q: IndexedLimitedQueue::new(limit) }
	}

	fn is_acceptable(&self, eew: &EEW) -> bool
	{
		match self.q.get(eew.id.as_ref()) {
			None => true,
			Some(vec) => {
				let latest = vec.last().expect("each buffer contains at least 1 EEW");
				latest.is_succeeded_by(eew)
			}
		}
	}

	pub fn append(&mut self, eew: EEW) -> Option<Arc<EEW>>
	{
		if self.is_acceptable(&eew) {
			let v = self.q.get_mut_default(eew.id.as_ref());
			let arc = Arc::new(eew);
			v.push(arc.clone());
			Some(arc)
		} else {
			None
		}
	}
}
