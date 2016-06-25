use std::collections::VecDeque;

use eew::EEW;


const MAX_BLOCK_COUNT: usize = 32;

pub struct EEWBuffer {
	buffer: VecDeque<Vec<EEW>> // each element of buffer must have at least 1 EEW object
}

impl EEWBuffer {

	pub fn new() -> EEWBuffer
	{
		EEWBuffer { buffer: VecDeque::new() }
	}

	fn lookup(&self, eew_id: &str) -> Option<usize>
	{
		for idx in 0..(self.buffer.len()) {
			if self.buffer[idx][0].id == eew_id {
				return Some(idx);
			}
		}

		return None;
	}

	fn extend_block(&mut self, idx: usize, eew: EEW) {

		let ref mut block = self.buffer[idx];

		let to_add = {
			let last_eew = block.last().expect("block must have at least 1 element");
			last_eew.number < eew.number
		};

		if to_add {
			block.push(eew);
		}
	}

	fn create_block(&mut self, eew: EEW) {

		let block = vec! { eew };
		self.buffer.push_back(block);

		while self.buffer.len() > MAX_BLOCK_COUNT {
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
				&self.buffer.back().expect("buffer must have at least 1 block")
			}
		}
	}
}
