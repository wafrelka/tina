#![cfg(test)]

extern crate chrono;
extern crate tina;

use chrono::*;
use tina::*;


fn make_dummy_eew(eew_id: &str, number: u32) -> EEW
{
	EEW { source: Source::Tokyo, kind: Kind::Normal, issued_at: UTC.timestamp(12345, 0),
		occurred_at: UTC.timestamp(12345, 0), id: eew_id.to_string(), status: Status::Normal,
		number: number, detail: EEWDetail::Full(FullEEW {
			issue_pattern: IssuePattern::HighAccuracy, epicenter_name: "-".to_string(),
			epicenter: (0.0, 0.0), depth: Some(10.0), magnitude: None, maximum_intensity: None,
			epicenter_accuracy: EpicenterAccuracy::NIEDHigh,
			depth_accuracy: DepthAccuracy::NIEDHigh,
			magnitude_accuracy: MagnitudeAccuracy::NIED,
			epicenter_category: EpicenterCategory::Land, warning_status: WarningStatus::Forecast,
			intensity_change: IntensityChange::Same, change_reason: ChangeReason::Nothing,
			area_info: vec!{}
		})
	}
}

fn make_dummy_cancel_eew(eew_id: &str, number: u32) -> EEW
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

#[test]
fn it_should_reject_same_number_eew()
{
	let eew1 = make_dummy_eew("A", 1);
	let eew2a = make_dummy_eew("A", 2);
	let eew2b = make_dummy_eew("A", 2);

	let mut buf = EEWBuffer::with_capacity(4);

	assert!(buf.append(&eew1) != None);
	assert!(buf.append(&eew2a) == Some(&[eew1, eew2a]));
	assert!(buf.append(&eew2b) == None);
}

#[test]
fn it_should_accept_same_number_eew_with_cancel()
{
	let eew1 = make_dummy_eew("A", 1);
	let eew2a = make_dummy_eew("A", 2);
	let eew2b = make_dummy_cancel_eew("A", 2);
	let eew2c = make_dummy_cancel_eew("A", 2);
	let eew2d = make_dummy_eew("A", 2);
	let eew2e = make_dummy_cancel_eew("A", 2);
	let eew3a = make_dummy_eew("A", 3);
	let eew3b = make_dummy_cancel_eew("A", 3);

	let mut buf = EEWBuffer::with_capacity(4);

	assert!(buf.append(&eew1) != None);
	assert!(buf.append(&eew2a) != None);
	assert!(buf.append(&eew2b) != None);
	assert!(buf.append(&eew2c) == None);
	assert!(buf.append(&eew2d) == None);
	assert!(buf.append(&eew2e) == None);
	assert!(buf.append(&eew3a) != None);
	assert!(buf.append(&eew3b) == Some(&[eew1, eew2a, eew2b, eew3a, eew3b]));
}
