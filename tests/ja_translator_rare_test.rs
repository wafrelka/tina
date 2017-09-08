extern crate chrono;
extern crate tina;

use chrono::*;
use tina::*;

fn make_base_eew() -> EEW
{
	EEW {
		issue_pattern: IssuePattern::HighAccuracy, source: Source::Tokyo, kind: Kind::Normal,
		issued_at: UTC.ymd(2010, 1, 1).and_hms(1, 0, 2),
		occurred_at: UTC.ymd(2010, 1, 1).and_hms(0, 55, 59),
		id: "ND20100101005559".to_owned(), status: Status::Normal, number: 10,
		detail: Some(make_base_eew_detail()),
	}
}

fn make_base_eew_detail() -> EEWDetail
{
	EEWDetail {
		epicenter_name: "奈良県".to_owned(), epicenter: (34.4, 135.7),
		depth: Some(10.0), magnitude: Some(5.9),
		maximum_intensity: Some(IntensityClass::FiveLower),
		epicenter_accuracy: EpicenterAccuracy::GridSearchLow,
		depth_accuracy: DepthAccuracy::GridSearchLow,
		magnitude_accuracy: MagnitudeAccuracy::SWave,
		epicenter_category: EpicenterCategory::Land,
		warning_status: WarningStatus::Forecast,
		intensity_change: IntensityChange::Unknown,
		change_reason: ChangeReason::Unknown,
		area_info: vec!{},
	}
}


#[test]
fn it_should_format_eew_with_south_west_epicenter()
{
	let eew = EEW {
		detail: Some(EEWDetail {
			epicenter: (-34.4, -135.7),
			.. make_base_eew().detail.unwrap()
		}),
		.. make_base_eew()
	};

	let expected =
		"[予報] 奈良県 震度5弱 M5.9 10km (S34.4/W135.7) 09:55:59発生 \
		| 第10報 ND20100101005559".to_owned();

	let result = ja_format_eew_short(&eew, None);

	assert_eq!(result, Some(expected));
}

#[test]
fn it_should_format_drill_eew()
{
	let eew = EEW {
		kind: Kind::Drill,
		.. make_base_eew()
	};

	let expected =
		"[訓練 | 予報] 奈良県 震度5弱 M5.9 10km (N34.4/E135.7) 09:55:59発生 \
		| 第10報 ND20100101005559".to_owned();

	let result = ja_format_eew_short(&eew, None);

	assert_eq!(result, Some(expected));
}

#[test]
fn it_should_format_drill_cancel_eew()
{
	let eew = EEW {
		issue_pattern: IssuePattern::Cancel,
		kind: Kind::DrillCancel,
		detail: None,
		.. make_base_eew()
	};

	let expected =
		"[訓練 | 取消] --- | 第10報 ND20100101005559".to_owned();

	let result = ja_format_eew_short(&eew, None);

	assert_eq!(result, Some(expected));
}

#[test]
fn it_should_format_reference_eew()
{
	let eew = EEW {
		kind: Kind::Reference,
		.. make_base_eew()
	};

	let expected =
		"[テスト配信 | 予報] 奈良県 震度5弱 M5.9 10km (N34.4/E135.7) 09:55:59発生 \
		| 第10報 ND20100101005559".to_owned();

	let result = ja_format_eew_short(&eew, None);

	assert_eq!(result, Some(expected));
}

#[test]
fn it_should_format_trial_eew()
{
	let eew = EEW {
		kind: Kind::Trial,
		.. make_base_eew()
	};

	let expected =
		"[テスト配信 | 予報] 奈良県 震度5弱 M5.9 10km (N34.4/E135.7) 09:55:59発生 \
		| 第10報 ND20100101005559".to_owned();

	let result = ja_format_eew_short(&eew, None);

	assert_eq!(result, Some(expected));
}
