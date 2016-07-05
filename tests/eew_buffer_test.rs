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

	buf.append(&eew1);
	buf.append(&eew2);
	assert!(buf.append(&eew3) == [eew1, eew2, eew3]);
	assert!(buf.append(&eew4) == [eew4]);
}

#[test]
fn it_should_not_save_old_eew()
{
	let eew1 = make_dummy_eew("A", 1);
	let eew2 = make_dummy_eew("A", 2);
	let eew3 = make_dummy_eew("A", 3);
	let eew4 = make_dummy_eew("A", 4);

	let mut buf = EEWBuffer::with_capacity(4);

	buf.append(&eew1);
	buf.append(&eew4);
	buf.append(&eew2);
	assert!(buf.append(&eew3) == [eew1, eew4]);
}

#[test]
fn it_should_erase_old_blocks_with_fifo_manner()
{
	let eewa1 = make_dummy_eew("A", 1);
	let eewa2 = make_dummy_eew("A", 2);
	let eewb = make_dummy_eew("B", 1);
	let eewc = make_dummy_eew("C", 1);
	let eewd1 = make_dummy_eew("D", 1);
	let eewd2 = make_dummy_eew("D", 2);

	let mut buf = EEWBuffer::with_capacity(2);

	buf.append(&eewa1);
	buf.append(&eewb);
	buf.append(&eewc);
	buf.append(&eewd1);

	assert!(buf.append(&eewa2) == [eewa2]);
	assert!(buf.append(&eewd2) == [eewd1, eewd2]);
}
