use std::sync::Arc;

use collections::LimitedQueue;
use eew::Kind;
use eew::EEW;


const DEFAULT_MAX_BLOCK_COUNT: usize = 16;
const INITIAL_BLOCK_CAPACITY: usize = 64;

pub struct EEWBuffer {
	buffer: LimitedQueue<Vec<Arc<EEW>>>, // each element of buffer must have at least 1 EEW object
}

impl EEWBuffer {

	pub fn new() -> EEWBuffer
	{
		EEWBuffer::with_allocation(DEFAULT_MAX_BLOCK_COUNT)
	}

	pub fn with_allocation(limit: usize) -> EEWBuffer
	{
		assert!(limit >= 1);
		EEWBuffer { buffer: LimitedQueue::with_allocation(limit) }
	}

	fn lookup(&self, eew_id: &str) -> Option<usize>
	{
		self.buffer.iter().position(|block| block.first().map(|eew| eew.id.as_ref()) == Some(eew_id))
	}

	fn is_acceptable(&self, idx: usize, eew: &EEW) -> bool
	{
		let block = &self.buffer[idx];
		let last_eew = block.last().expect("a block must have at least 1 element");

		if last_eew.number != eew.number {
			return last_eew.number < eew.number;
		}

		let is_cancel = |e: &EEW| {
			match e.kind {
				Kind::Cancel | Kind::DrillCancel => true,
				_ => false
			}
		};

		return !is_cancel(last_eew) && is_cancel(eew);
	}

	fn extend_block(&mut self, idx: usize, eew: Arc<EEW>) -> bool
	{
		let to_accept = self.is_acceptable(idx, &eew);
		if to_accept {
			self.buffer[idx].push(eew);
		}
		to_accept
	}

	fn create_block(&mut self, eew: Arc<EEW>)
	{
		let mut v = Vec::with_capacity(INITIAL_BLOCK_CAPACITY);
		v.push(eew);
		self.buffer.push(v);
	}

	pub fn append(&mut self, eew: Arc<EEW>) -> Option<&[Arc<EEW>]>
	{
		match self.lookup(&eew.id) {

			Some(idx) => {
				match self.extend_block(idx, eew) {
					true => Some(&self.buffer[idx]),
					false => None
				}
			},
			None => {
				self.create_block(eew);
				Some(&self.buffer.back().expect("a buffer must have at least 1 block"))
			}
		}
	}
}
