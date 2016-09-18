#![cfg(test)]

extern crate chrono;
extern crate tina;

use std::sync::Arc;

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
	let eew1 = Arc::new(make_dummy_eew("A", 1));
	let eew2 = Arc::new(make_dummy_eew("A", 2));
	let eew3 = Arc::new(make_dummy_eew("A", 3));
	let eew4 = Arc::new(make_dummy_eew("B", 1));

	let mut buf = EEWBuffer::with_allocation(4);

	assert!(buf.append(eew1.clone()) != None);
	assert!(buf.append(eew2.clone()) != None);
	assert!(buf.append(eew3.clone()) == Some(&[eew1, eew2, eew3]));
	assert!(buf.append(eew4.clone()) == Some(&[eew4]));
}

#[test]
fn it_should_not_save_old_eew()
{
	let eew1 = Arc::new(make_dummy_eew("A", 1));
	let eew2 = Arc::new(make_dummy_eew("A", 2));
	let eew3 = Arc::new(make_dummy_eew("A", 3));
	let eew4 = Arc::new(make_dummy_eew("A", 4));

	let mut buf = EEWBuffer::with_allocation(4);

	assert!(buf.append(eew1.clone()) != None);
	assert!(buf.append(eew4.clone()) == Some(&[eew1, eew4]));
	assert!(buf.append(eew2.clone()) == None);
	assert!(buf.append(eew3.clone()) == None);
}

#[test]
fn it_should_erase_old_blocks_with_fifo_manner()
{
	let eewa1 = Arc::new(make_dummy_eew("A", 1));
	let eewa2 = Arc::new(make_dummy_eew("A", 2));
	let eewb1 = Arc::new(make_dummy_eew("B", 1));
	let eewc1 = Arc::new(make_dummy_eew("C", 1));
	let eewc2 = Arc::new(make_dummy_eew("C", 2));
	let eewd1 = Arc::new(make_dummy_eew("D", 1));
	let eewd2 = Arc::new(make_dummy_eew("D", 2));

	let mut buf = EEWBuffer::with_allocation(2);

	assert!(buf.append(eewa1.clone()) != None);
	assert!(buf.append(eewb1.clone()) != None);
	assert!(buf.append(eewc1.clone()) != None);
	assert!(buf.append(eewd1.clone()) != None);

	assert!(buf.append(eewd2.clone()) == Some(&[eewd1, eewd2]));
	assert!(buf.append(eewa2.clone()) == Some(&[eewa2]));
	assert!(buf.append(eewc2.clone()) == Some(&[eewc2]));
}

#[test]
fn it_should_reject_same_number_eew()
{
	let eew1a = Arc::new(make_dummy_eew("A", 1));
	let eew2a = Arc::new(make_dummy_eew("A", 2));
	let eew2b = Arc::new(make_dummy_eew("A", 2));

	let mut buf = EEWBuffer::with_allocation(4);

	assert!(buf.append(eew1a.clone()) != None);
	assert!(buf.append(eew2a.clone()) == Some(&[eew1a, eew2a]));
	assert!(buf.append(eew2b.clone()) == None);
}

#[test]
fn it_should_accept_same_number_eew_with_cancel()
{
	let eew1a = Arc::new(make_dummy_eew("A", 1));
	let eew2a = Arc::new(make_dummy_eew("A", 2));
	let eew2b = Arc::new(make_dummy_cancel_eew("A", 2));
	let eew2c = Arc::new(make_dummy_cancel_eew("A", 2));
	let eew2d = Arc::new(make_dummy_eew("A", 2));
	let eew2e = Arc::new(make_dummy_cancel_eew("A", 2));
	let eew3a = Arc::new(make_dummy_eew("A", 3));
	let eew3b = Arc::new(make_dummy_cancel_eew("A", 3));

	let mut buf = EEWBuffer::with_allocation(4);

	assert!(buf.append(eew1a.clone()) != None);
	assert!(buf.append(eew2a.clone()) != None);
	assert!(buf.append(eew2b.clone()) != None);
	assert!(buf.append(eew2c.clone()) == None);
	assert!(buf.append(eew2d.clone()) == None);
	assert!(buf.append(eew2e.clone()) == None);
	assert!(buf.append(eew3a.clone()) != None);
	assert!(buf.append(eew3b.clone()) == Some(&[eew1a, eew2a, eew2b, eew3a, eew3b]));
}
