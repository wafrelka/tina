extern crate tina;

use tina::*;


#[test]
fn it_should_count()
{
	let mut m = Moderator::with_custom_rate(10, 2);

	for _ in 0..5 {
		m.add_count();
	}

	assert_eq!(m.compute_next_interval(), 2u32.pow(5));
}

#[test]
fn it_should_reset_count()
{
	let mut m = Moderator::with_custom_rate(10, 2);

	for _ in 0..5 {
		m.add_count();
	}

	m.reset();
	m.add_count();
	m.add_count();

	assert_eq!(m.compute_next_interval(), 2u32.pow(2));
}

#[test]
fn it_should_saturate_counting()
{
	let mut m = Moderator::with_custom_rate(5, 2);

	for _ in 0..100 {
		m.add_count();
	}

	assert_eq!(m.compute_next_interval(), 2u32.pow(5));
}
