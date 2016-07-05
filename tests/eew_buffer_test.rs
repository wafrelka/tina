#![cfg(test)]

extern crate chrono;
extern crate tina;

use chrono::*;
use tina::*;


fn make_dummy_eew(eew_id: &str, number: u32) -> EEW
{
	EEW { source: Source::Tokyo, kind: Kind::Cancel, issued_at: UTC.timestamp(12345, 0),
		occurred_at: UTC.timestamp(12345, 0), id: eew_id.to_string(), status: Status::Normal,
		number: number, detail: EEWDetail::Cancel
	}
}

#[test]
fn it_should_hold_related_eews_within_the_same_block()
{
	let eew1 = make_dummy_eew("A", 1);
	let eew2 = make_dummy_eew("A", 2);
	let eew3 = make_dummy_eew("A", 3);
	let eew4 = make_dummy_eew("B", 1);

	let mut buf = EEWBuffer::with_capacity(4);

	assert!(buf.append(&eew1) != None);
	assert!(buf.append(&eew2) != None);
	assert!(buf.append(&eew3) == Some(&[eew1, eew2, eew3]));
	assert!(buf.append(&eew4) == Some(&[eew4]));
}

#[test]
fn it_should_not_save_old_eew()
{
	let eew1 = make_dummy_eew("A", 1);
	let eew2 = make_dummy_eew("A", 2);
	let eew3 = make_dummy_eew("A", 3);
	let eew4 = make_dummy_eew("A", 4);

	let mut buf = EEWBuffer::with_capacity(4);

	assert!(buf.append(&eew1) != None);
	assert!(buf.append(&eew4) == Some(&[eew1, eew4]));
	assert!(buf.append(&eew2) == None);
	assert!(buf.append(&eew3) == None);
}

#[test]
fn it_should_erase_old_blocks_with_fifo_manner()
{
	let eewa1 = make_dummy_eew("A", 1);
	let eewa2 = make_dummy_eew("A", 2);
	let eewb = make_dummy_eew("B", 1);
	let eewc1 = make_dummy_eew("C", 1);
	let eewc2 = make_dummy_eew("C", 2);
	let eewd1 = make_dummy_eew("D", 1);
	let eewd2 = make_dummy_eew("D", 2);

	let mut buf = EEWBuffer::with_capacity(2);

	assert!(buf.append(&eewa1) != None);
	assert!(buf.append(&eewb) != None);
	assert!(buf.append(&eewc1) != None);
	assert!(buf.append(&eewd1) != None);

	assert!(buf.append(&eewd2) == Some(&[eewd1, eewd2]));
	assert!(buf.append(&eewa2) == Some(&[eewa2]));
	assert!(buf.append(&eewc2) == Some(&[eewc2]));
}
