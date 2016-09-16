use std::ops::{Index, IndexMut};
use std::collections::vec_deque::Iter;
use std::collections::VecDeque;


pub struct LimitedQueue<T> {
	q: VecDeque<T>,
	limit: usize
}

impl<T> LimitedQueue<T> {

	pub fn new(limit: usize) -> LimitedQueue<T>
	{
		let q = VecDeque::new();
		LimitedQueue { q: q, limit: limit }
	}

	pub fn with_allocation(limit: usize) -> LimitedQueue<T>
	{
		let q = VecDeque::with_capacity(limit);
		LimitedQueue { q: q, limit: limit }
	}

	pub fn push(&mut self, item: T)
	{
		self.q.push_back(item);
		while self.q.len() > self.limit {
			self.q.pop_front();
		}
	}

	pub fn iter(&self) -> Iter<T> { self.q.iter() }
	pub fn front(&self) -> Option<&T> { self.q.front() }
	pub fn back(&self) -> Option<&T> { self.q.back() }
}

impl<T> Index<usize> for LimitedQueue<T> {
	type Output = T;
	fn index(&self, index: usize) -> &T { &self.q[index] }
}

impl<T> IndexMut<usize> for LimitedQueue<T> {
	fn index_mut(&mut self, index: usize) -> &mut T { &mut self.q[index] }
}
