extern crate tina;

use tina::*;

#[test]
fn it_should_find_pushed_element()
{
	let mut q = IndexedLimitedQueue::new(3);

	q.upsert("a".to_owned(), 1);
	q.upsert("b".to_owned(), 2);
	q.upsert("c".to_owned(), 3);

	assert_eq!(q.get("a"), Some(&1));
	assert_eq!(q.get("b"), Some(&2));
	assert_eq!(q.get("c"), Some(&3));
}

#[test]
fn it_should_update_element()
{
	let mut q = IndexedLimitedQueue::new(3);

	q.upsert("a".to_owned(), 1);
	q.upsert("b".to_owned(), 2);
	q.upsert("c".to_owned(), 3);
	q.upsert("b".to_owned(), 4);

	assert_eq!(q.get("a"), Some(&1));
	assert_eq!(q.get("b"), Some(&4));
	assert_eq!(q.get("c"), Some(&3));
}

#[test]
fn it_should_forget_element()
{
	let mut q = IndexedLimitedQueue::new(3);

	q.upsert("a".to_owned(), 1);
	q.upsert("b".to_owned(), 2);
	q.upsert("c".to_owned(), 3);
	q.upsert("d".to_owned(), 4);

	assert_eq!(q.get("a"), None);
	assert_eq!(q.get("b"), Some(&2));
}

#[test]
fn it_should_create_default_element()
{
	let mut q = IndexedLimitedQueue::new(3);

	q.upsert("a".to_owned(), 1);
	q.upsert("b".to_owned(), 2);
	q.upsert("c".to_owned(), 3);

	assert_eq!(q.get_mut_default("a".to_owned()), &1);
	assert_eq!(q.get_mut_default("d".to_owned()), &0);
	assert_eq!(q.get("a".to_owned()), None);
}

#[test]
fn it_should_return_old_element_when_upserting()
{
	let mut q = IndexedLimitedQueue::new(2);

	assert_eq!(q.upsert("a".to_owned(), 100), None);
	assert_eq!(q.upsert("a".to_owned(), 200), Some(100));
	assert_eq!(q.upsert("a".to_owned(), 300), Some(200));

	q.upsert("b".to_owned(), 100);
	q.upsert("c".to_owned(), 100);

	assert_eq!(q.upsert("a".to_owned(), 100), None);
}
