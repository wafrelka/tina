extern crate tina;

use tina::*;

#[test]
fn it_should_erase_old_blocks_with_fifo_manner()
{
	let mut q = LimitedQueue::new(4);

	q.push(1);
	q.push(2);
	assert_eq!(q.iter().collect::<Vec<_>>(), vec!{&1, &2});

	q.push(3);
	q.push(4);
	assert_eq!(q.iter().collect::<Vec<_>>(), vec!{&1, &2, &3, &4});

	q.push(5);
	assert_eq!(q.iter().collect::<Vec<_>>(), vec!{&2, &3, &4, &5});

	q.push(6);
	q.push(7);
	q.push(8);
	q.push(9);
	assert_eq!(q.iter().collect::<Vec<_>>(), vec!{&6, &7, &8, &9});
}
