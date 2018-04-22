use std::mem;

use std::collections::vec_deque::{Iter, IterMut};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct LimitedQueue<T> {
	q: VecDeque<T>,
	limit: usize,
}

impl<T> LimitedQueue<T> {

	pub fn new(limit: usize) -> LimitedQueue<T>
	{
		LimitedQueue { q: VecDeque::new(), limit: limit }
	}

	pub fn with_allocation(limit: usize) -> LimitedQueue<T>
	{
		LimitedQueue { q: VecDeque::with_capacity(limit), limit: limit }
	}

	pub fn push(&mut self, item: T)
	{
		self.q.push_back(item);
		while self.q.len() > self.limit {
			self.q.pop_front();
		}
	}

	pub fn iter(&self) -> Iter<T> { self.q.iter() }
	pub fn iter_mut(&mut self) -> IterMut<T> { self.q.iter_mut() }
	pub fn front(&self) -> Option<&T> { self.q.front() }
	pub fn front_mut(&mut self) -> Option<&mut T> { self.q.front_mut() }
	pub fn back(&self) -> Option<&T> { self.q.back() }
	pub fn back_mut(&mut self) -> Option<&mut T> { self.q.back_mut() }
}

#[derive(Debug, Clone)]
pub struct IndexedLimitedQueue<D> {
	buffer: LimitedQueue<(String, D)>,
}

impl<D> IndexedLimitedQueue<D> {

	pub fn new(limit: usize) -> IndexedLimitedQueue<D>
	{
		assert!(limit > 0);
		IndexedLimitedQueue { buffer: LimitedQueue::new(limit) }
	}

	pub fn with_allocation(limit: usize) -> IndexedLimitedQueue<D>
	{
		assert!(limit > 0);
		IndexedLimitedQueue { buffer: LimitedQueue::with_allocation(limit) }
	}

	pub fn get<I>(&self, idx: I) -> Option<&D>
		where I: PartialEq<String>
	{
		self.buffer.iter().find(|ref e| idx == e.0).map(|ref e| &e.1)
	}

	pub fn get_mut<I>(&mut self, idx: I) -> Option<&mut D>
		where I: PartialEq<String>
	{
		self.buffer.iter_mut().find(|ref e| idx == e.0).map(|e| &mut e.1)
	}

	pub fn get_mut_default<I>(&mut self, idx: I) -> &mut D
		where I: PartialEq<String> + Into<String>, D: Default
	{
		if self.buffer.iter().find(|ref e| idx == e.0).is_none() {
			self.buffer.push((idx.into(), Default::default()));
			self.buffer.back_mut().map(|e| &mut e.1).expect("always exists")
		} else {
			self.buffer.iter_mut().find(|ref e| idx == e.0).map(|e| &mut e.1).expect("always exists")
		}
	}

	pub fn upsert<I>(&mut self, idx: I, mut data: D) -> Option<D>
		where I: PartialEq<String> + Into<String>
	{
		if let Some(it) = self.buffer.iter_mut().find(|ref e| idx == e.0).map(|e| &mut e.1) {
			mem::swap(it, &mut data);
			return Some(data);
		}

		self.buffer.push((idx.into(), data));
		None
	}
}
