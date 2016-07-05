use std::collections::VecDeque;

use eew::EEW;


const DEFAULT_MAX_BLOCK_COUNT: usize = 32;

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
		return self.buffer.iter().position(|ref b|
			b.first().map(|ref e| e.id.as_str()) == Some(eew_id));
	}

	fn extend_block(&mut self, idx: usize, eew: EEW) {

		let ref mut block = self.buffer[idx];

		let is_latest = {
			let last_eew = block.last().expect("a block must have at least 1 element");
			last_eew.number < eew.number
		};

		if is_latest {
			block.push(eew);
		}
	}

	fn create_block(&mut self, eew: EEW) {

		let block = vec! { eew };
		self.buffer.push_back(block);

		while self.buffer.len() > self.max_block_count {
			self.buffer.pop_front();
		}
	}

	pub fn append(&mut self, eew: EEW) -> &[EEW]
	{
		match self.lookup(&eew.id) {

			Some(idx) => {
				self.extend_block(idx, eew);
				&self.buffer[idx]
			},
			None => {
				self.create_block(eew);
				&self.buffer.back().expect("a buffer must have at least 1 block")
			}
		}
	}
}
