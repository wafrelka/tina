extern crate tina;

use tina::*;

#[test]
fn it_should_find_pushed_element()
{
	let mut q = IndexedLimitedQueue::new(3);

	q.upsert("a", 1);
	q.upsert("b", 2);
	q.upsert("c", 3);

	assert_eq!(q.get("a"), Some(&1));
	assert_eq!(q.get("b"), Some(&2));
	assert_eq!(q.get("c"), Some(&3));
}

#[test]
fn it_should_update_element()
{
	let mut q = IndexedLimitedQueue::new(3);

	q.upsert("a", 1);
	q.upsert("b", 2);
	q.upsert("c", 3);
	q.upsert("b", 4);

	assert_eq!(q.get("a"), Some(&1));
	assert_eq!(q.get("b"), Some(&4));
	assert_eq!(q.get("c"), Some(&3));
}

#[test]
fn it_should_forget_element()
{
	let mut q = IndexedLimitedQueue::new(3);

	q.upsert("a", 1);
	q.upsert("b", 2);
	q.upsert("c", 3);
	q.upsert("d", 4);

	assert_eq!(q.get("a"), None);
	assert_eq!(q.get("b"), Some(&2));
}

#[test]
fn it_should_create_default_element()
{
	let mut q = IndexedLimitedQueue::new(3);

	q.upsert("a", 1);
	q.upsert("b", 2);
	q.upsert("c", 3);

	assert_eq!(q.get_mut_default("a"), &1);
	assert_eq!(q.get_mut_default("d"), &0);
	assert_eq!(q.get("a"), None);
}

#[test]
fn it_should_return_old_element_when_upserting()
{
	let mut q = IndexedLimitedQueue::new(2);

	assert_eq!(q.upsert("a", 100), None);
	assert_eq!(q.upsert("a", 200), Some(100));
	assert_eq!(q.upsert("a", 300), Some(200));

	q.upsert("b", 100);
	q.upsert("c", 100);

	assert_eq!(q.upsert("a", 100), None);
}
