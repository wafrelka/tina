use std::thread::sleep;
use std::time::Duration;


const DEFAULT_MAX_COUNT: u32 = 10;
const DEFAULT_RATE: u32 = 2;

pub struct Moderator {
	count: u32,
	max_count: u32,
	rate: u32
}

impl Moderator {

	pub fn new() -> Moderator
	{
		Moderator::with_custom_rate(DEFAULT_MAX_COUNT, DEFAULT_RATE)
	}

	pub fn with_custom_rate(max_count: u32, rate: u32) -> Moderator
	{
		Moderator {
			count: 0,
			max_count: max_count,
			rate: rate
		}
	}

	pub fn reset(&mut self)
	{
		self.count = 0;
	}

	pub fn add_count(&mut self)
	{
		if self.count < self.max_count {
			self.count += 1;
		}
	}

	pub fn compute_next_interval(&self) -> u32
	{
		self.rate.pow(self.count)
	}


	pub fn wait_for_retry(&self)
	{
		let s = self.compute_next_interval();
		sleep(Duration::from_secs(s as u64));
	}
}
