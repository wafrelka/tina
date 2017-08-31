use std::sync::Arc;

use collections::LimitedQueue;
use eew::{EEW, EEWPhase};
use condition::Condition;


const DEFAULT_MAX_BLOCK_COUNT: usize = 16;
const INITIAL_BLOCK_CAPACITY: usize = 64;

pub enum EEWBufferError {
	Order,
	Filter,
}

pub struct EEWList {
	pub full: Vec<Arc<EEW>>,
	pub filtered: Vec<Arc<EEW>>,
	pub latest: Arc<EEW>,
	pub id: String,
}

pub struct EEWBuffer<C> {
	cond: C,
	buffer: LimitedQueue<EEWList>,
}

impl EEWList {

	pub fn new<C>(eew: Arc<EEW>, cond: &C) -> EEWList
		where C: Condition
	{
		let id = eew.id.clone();
		let mut full = Vec::with_capacity(INITIAL_BLOCK_CAPACITY);
		let mut filtered = Vec::with_capacity(INITIAL_BLOCK_CAPACITY);

		full.push(eew.clone());
		if cond.is_satisfied(&eew, &vec!{}) {
			filtered.push(eew.clone());
		}

		EEWList { full: full, filtered: filtered, latest: eew, id: id }
	}

	pub fn is_acceptable(&self, eew: &EEW) -> bool
	{
		if eew.number != self.latest.number {
			eew.number > self.latest.number
		} else {
			let cur_cancel = eew.get_eew_phase() == Some(EEWPhase::Cancel);
			let prev_cancel = self.latest.get_eew_phase() == Some(EEWPhase::Cancel);
			cur_cancel && !prev_cancel
		}
	}

	pub fn append<C>(&mut self, eew: Arc<EEW>, cond: &C) -> Result<(), EEWBufferError>
		where C: Condition
	{
		let ok = self.is_acceptable(&eew);

		if ok {
			self.full.push(eew.clone());
			if cond.is_satisfied(&eew, &self.filtered) {
				self.filtered.push(eew);
				Ok(())
			} else {
				Err(EEWBufferError::Filter)
			}
		} else {
			Err(EEWBufferError::Order)
		}
	}
}

impl<C> EEWBuffer<C> where C: Condition {

	pub fn new(cond: C) -> EEWBuffer<C>
	{
		EEWBuffer::with_allocation(cond, DEFAULT_MAX_BLOCK_COUNT)
	}

	pub fn with_allocation(cond: C, limit: usize) -> EEWBuffer<C>
	{
		assert!(limit >= 1);
		EEWBuffer { cond: cond, buffer: LimitedQueue::with_allocation(limit) }
	}

	pub fn append(&mut self, eew: Arc<EEW>) -> Result<&EEWList, EEWBufferError>
	{
		let pos = self.buffer.iter().position(|ref block| block.id == eew.id);

		if let Some(index) = pos {

			self.buffer[index].append(eew, &self.cond).and(Ok(&self.buffer[index]))

		} else {

			self.buffer.push(EEWList::new(eew, &self.cond));
			let block = self.buffer.back().expect("buffer size must be > 0");

			if block.filtered.len() == 0 {
				Err(EEWBufferError::Filter)
			} else {
				Ok(&block)
			}
		}
	}
}
