use std::collections::VecDeque;

use eew::Kind;
use eew::EEW;


const DEFAULT_MAX_BLOCK_COUNT: usize = 16;

pub struct EEWBuffer {
	buffer: VecDeque<Vec<EEW>>, // each element of buffer must have at least 1 EEW object
	max_block_count: usize
}

impl EEWBuffer {

	pub fn new() -> EEWBuffer
	{
		EEWBuffer { buffer: VecDeque::new(), max_block_count: DEFAULT_MAX_BLOCK_COUNT }
	}

	pub fn with_capacity(n: usize) -> EEWBuffer
	{
		assert!(n >= 1);
		EEWBuffer { buffer: VecDeque::new(), max_block_count: n }
	}

	fn lookup(&self, eew_id: &str) -> Option<usize>
	{
		return self.buffer.iter().position(|ref block|
			block.first().map(|ref eew| eew.id.as_str()) == Some(eew_id));
	}

	fn is_acceptable(&self, idx: usize, eew: &EEW) -> bool {

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

	fn extend_block(&mut self, idx: usize, eew: &EEW) -> bool {

		let to_accept = self.is_acceptable(idx, eew);
		if to_accept {
			self.buffer[idx].push(eew.clone());
		}
		return to_accept;
	}

	fn create_block(&mut self, eew: &EEW) {

		let block = vec! { eew.clone() };
		self.buffer.push_back(block);

		while self.buffer.len() > self.max_block_count {
			self.buffer.pop_front();
		}
	}

	pub fn append(&mut self, eew: &EEW) -> Option<&[EEW]>
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
