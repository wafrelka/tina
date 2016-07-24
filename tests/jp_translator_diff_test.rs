#![cfg(test)]

extern crate chrono;
extern crate tina;

use chrono::*;
use tina::*;


enum Diff { Up, Down, Same }

fn make_dummy_eew(maximum_intensity: Option<f32>) -> EEW
{
	EEW {
		source: Source::Tokyo, kind: Kind::Normal, issued_at: UTC.timestamp(12345, 0),
		occurred_at: UTC.timestamp(12345, 0), id: "XXX".to_owned(),
		status: Status::Normal, number: 1,
		detail: EEWDetail::Full(FullEEW {
			issue_pattern: IssuePattern::HighAccuracy, epicenter_name: "YYY".to_owned(),
			epicenter: (38.0, 142.0), depth: Some(10.0), magnitude: Some(5.9),
			maximum_intensity: maximum_intensity,
			epicenter_accuracy: EpicenterAccuracy::Single, depth_accuracy: DepthAccuracy::Single,
			magnitude_accuracy: MagnitudeAccuracy::PWave,
			epicenter_category: EpicenterCategory::Sea, warning_status: WarningStatus::Forecast,
			intensity_change: IntensityChange::Unknown, change_reason: ChangeReason::Unknown,
			area_info: vec!{}
		})
	}
}

fn expect_eew_string(maximum_intensity: Option<f32>, updown: Diff) -> String
{
	let i_str = match IntensityClass::new(maximum_intensity) {
		IntensityClass::Unknown => "震度不明",
		IntensityClass::Zero => "震度0",
		IntensityClass::One => "震度1",
		IntensityClass::Two => "震度2",
		IntensityClass::Three => "震度3",
		IntensityClass::Four => "震度4",
		IntensityClass::FiveMinus => "震度5弱",
		IntensityClass::FivePlus => "震度5強",
		IntensityClass::SixMinus => "震度6弱",
		IntensityClass::SixPlus => "震度6強",
		IntensityClass::Seven => "震度7"
	};
	let updown_str = match updown {
		Diff::Up => "↑",
		Diff::Down => "↓",
		Diff::Same => ""
	};
	return format!("[予報{}] YYY {} M5.9 10km (N38.0/E142.0) 12:25:45発生 | 第1報 XXX",
		updown_str, i_str);
}

#[test]
fn it_should_format_with_intensity_same_1()
{
	let eew1 = make_dummy_eew(Some(4.0));
	let eew2 = make_dummy_eew(Some(4.0));
	let expected = expect_eew_string(Some(4.0), Diff::Same);
	let result = ja_format_eew_short(&eew2, Some(&eew1));

	assert!(result.is_some());
	assert_eq!(result.unwrap(), expected);
}

#[test]
fn it_should_format_with_intensity_same_2()
{
	let eew1 = make_dummy_eew(Some(4.0));
	let eew2 = make_dummy_eew(Some(4.0));
	let expected = expect_eew_string(Some(4.0), Diff::Same);
	let result = ja_format_eew_short(&eew2, Some(&eew1));

	assert!(result.is_some());
	assert_eq!(result.unwrap(), expected);
}

#[test]
fn it_should_format_with_intensity_up_1()
{
	let eew1 = make_dummy_eew(Some(2.0));
	let eew2 = make_dummy_eew(Some(4.0));
	let expected = expect_eew_string(Some(4.0), Diff::Up);
	let result = ja_format_eew_short(&eew2, Some(&eew1));

	assert!(result.is_some());
	assert_eq!(result.unwrap(), expected);
}

#[test]
fn it_should_format_with_intensity_up_2()
{
	let eew1 = make_dummy_eew(None);
	let eew2 = make_dummy_eew(Some(4.0));
	let expected = expect_eew_string(Some(4.0), Diff::Up);
	let result = ja_format_eew_short(&eew2, Some(&eew1));

	assert!(result.is_some());
	assert_eq!(result.unwrap(), expected);
}

#[test]
fn it_should_format_with_intensity_down_1()
{
	let eew1 = make_dummy_eew(Some(4.0));
	let eew2 = make_dummy_eew(Some(2.0));
	let expected = expect_eew_string(Some(2.0), Diff::Down);
	let result = ja_format_eew_short(&eew2, Some(&eew1));

	assert!(result.is_some());
	assert_eq!(result.unwrap(), expected);
}

#[test]
fn it_should_format_with_intensity_down_2()
{
	let eew1 = make_dummy_eew(Some(4.0));
	let eew2 = make_dummy_eew(None);
	let expected = expect_eew_string(None, Diff::Down);
	let result = ja_format_eew_short(&eew2, Some(&eew1));

	assert!(result.is_some());
	assert_eq!(result.unwrap(), expected);
}
