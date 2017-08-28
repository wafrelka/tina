#![cfg(test)]

extern crate chrono;
extern crate tina;

use chrono::*;
use tina::*;


#[test]
fn it_should_format_cancel_eew()
{
	let eew = EEW {
		source: Source::Tokyo,
		kind: Kind::Cancel,
		issued_at: UTC.ymd(2012, 1, 8).and_hms(4, 32, 17),
		occurred_at: UTC.ymd(2012, 1, 8).and_hms(4, 31, 54),
		id: "ND20120108133201".to_owned(),
		status: Status::Normal,
		number: 3,
		detail: EEWDetail::Cancel,
	};

	let expected = "[取消] --- | 第3報 ND20120108133201";

	let result = ja_format_eew_short(&eew, None);

	assert!(result.is_some());
	assert_eq!(result.unwrap(), expected);
}

#[test]
fn it_should_format_low_accuracy_eew()
{
	let eew = EEW {
		source: Source::Tokyo,
		kind: Kind::Normal,
		issued_at: UTC.ymd(2013, 8, 4).and_hms(3, 29, 5),
		occurred_at: UTC.ymd(2013, 8, 4).and_hms(3, 28, 49),
		id: "ND20130804122902".to_owned(),
		status: Status::Normal,
		number: 1,
		detail: EEWDetail::Full(FullEEW {
			issue_pattern: IssuePattern::IntensityOnly,
			epicenter_name: "宮城県沖".to_owned(),
			epicenter: (38.0, 142.0),
			depth: Some(10.0),
			magnitude: Some(5.9),
			maximum_intensity: Some(4.0),
			epicenter_accuracy: EpicenterAccuracy::Single,
			depth_accuracy: DepthAccuracy::Single,
			magnitude_accuracy: MagnitudeAccuracy::PWave,
			epicenter_category: EpicenterCategory::Sea,
			warning_status: WarningStatus::Forecast,
			intensity_change: IntensityChange::Unknown,
			change_reason: ChangeReason::Unknown,
			area_info: vec!{}
		})
	};

	let expected = "[速報] 宮城県沖 震度4 M5.9 10km (N38.0/E142.0) 12:28:49発生 | 第1報 ND20130804122902";

	let result = ja_format_eew_short(&eew, None);

	assert!(result.is_some());
	assert_eq!(result.unwrap(), expected);
}

#[test]
fn it_should_format_high_accuracy_eew()
{
	let eew = EEW {
		source: Source::Tokyo,
		kind: Kind::Normal,
		issued_at: UTC.ymd(2013, 8, 4).and_hms(3, 29, 5),
		occurred_at: UTC.ymd(2013, 8, 4).and_hms(0, 0, 0),
		id: "ND20130804122902".to_owned(),
		status: Status::Correction,
		number: 2,
		detail: EEWDetail::Full(FullEEW {
			issue_pattern: IssuePattern::HighAccuracy,
			epicenter_name: "宮城県沖".to_owned(),
			epicenter: (-38.0, -142.0),
			depth: None,
			magnitude: None,
			maximum_intensity: None,
			epicenter_accuracy: EpicenterAccuracy::Single,
			depth_accuracy: DepthAccuracy::Single,
			magnitude_accuracy: MagnitudeAccuracy::PWave,
			epicenter_category: EpicenterCategory::Sea,
			warning_status: WarningStatus::Forecast,
			intensity_change: IntensityChange::Unknown,
			change_reason: ChangeReason::Unknown,
			area_info: vec!{}
		})
	};

	let expected = "[予報] 宮城県沖 震度不明 M--- ---km (S38.0/W142.0) 09:00:00発生 | 第2報 ND20130804122902";

	let result = ja_format_eew_short(&eew, None);

	assert!(result.is_some());
	assert_eq!(result.unwrap(), expected);
}

#[test]
fn it_should_format_alert_eew_with_high_accuracy()
{
	let eew = EEW {
		source: Source::Tokyo,
		kind: Kind::Normal,
		issued_at: UTC.ymd(2013, 8, 8).and_hms(7, 57, 2),
		occurred_at: UTC.ymd(2013, 8, 8).and_hms(0, 55, 59),
		id: "ND20130808165608".to_owned(),
		status: Status::Last,
		number: 6,
		detail: EEWDetail::Full(FullEEW {
			issue_pattern: IssuePattern::HighAccuracy,
			epicenter_name: "奈良県".to_owned(),
			epicenter: (34.4, 135.7),
			depth: Some(60.0),
			magnitude: Some(6.8),
			maximum_intensity: Some(5.25),
			epicenter_accuracy: EpicenterAccuracy::GridSearchLow,
			depth_accuracy: DepthAccuracy::GridSearchLow,
			magnitude_accuracy: MagnitudeAccuracy::SWave,
			epicenter_category: EpicenterCategory::Land,
			warning_status: WarningStatus::Alert,
			intensity_change: IntensityChange::Down,
			change_reason: ChangeReason::Magnitude,
			area_info: vec!{}
		})
	};

	let expected = "[警報/最終] 奈良県 震度5強 M6.8 60km (N34.4/E135.7) 09:55:59発生 | 第6報 ND20130808165608";

	let result = ja_format_eew_short(&eew, None);

	assert!(result.is_some());
	assert_eq!(result.unwrap(), expected);
}

#[test]
fn it_should_format_alert_eew_with_low_accuracy()
{
	let eew = EEW {
		source: Source::Tokyo,
		kind: Kind::Normal,
		issued_at: UTC.ymd(2013, 8, 8).and_hms(7, 57, 2),
		occurred_at: UTC.ymd(2013, 8, 8).and_hms(0, 55, 59),
		id: "ND20130808165608".to_owned(),
		status: Status::Last,
		number: 6,
		detail: EEWDetail::Full(FullEEW {
			issue_pattern: IssuePattern::IntensityOnly,
			epicenter_name: "奈良県".to_owned(),
			epicenter: (34.4, 135.7),
			depth: Some(60.0),
			magnitude: Some(6.8),
			maximum_intensity: Some(5.25),
			epicenter_accuracy: EpicenterAccuracy::GridSearchLow,
			depth_accuracy: DepthAccuracy::GridSearchLow,
			magnitude_accuracy: MagnitudeAccuracy::SWave,
			epicenter_category: EpicenterCategory::Land,
			warning_status: WarningStatus::Alert,
			intensity_change: IntensityChange::Down,
			change_reason: ChangeReason::Magnitude,
			area_info: vec!{}
		})
	};

	let expected = "[警報/最終] 奈良県 震度5強 M6.8 60km (N34.4/E135.7) 09:55:59発生 | 第6報 ND20130808165608";

	let result = ja_format_eew_short(&eew, None);

	assert!(result.is_some());
	assert_eq!(result.unwrap(), expected);
}

#[test]
fn it_should_format_test_eew()
{
	let eew = EEW {
		source: Source::Tokyo,
		kind: Kind::Test,
		issued_at: UTC.ymd(2013, 8, 8).and_hms(7, 57, 2),
		occurred_at: UTC.ymd(2013, 8, 8).and_hms(0, 55, 59),
		id: "ND20130808165608".to_owned(),
		status: Status::Last,
		number: 6,
		detail: EEWDetail::Full(FullEEW {
			issue_pattern: IssuePattern::IntensityOnly,
			epicenter_name: "奈良県".to_owned(),
			epicenter: (34.4, 135.7),
			depth: Some(60.0),
			magnitude: Some(6.8),
			maximum_intensity: Some(5.25),
			epicenter_accuracy: EpicenterAccuracy::GridSearchLow,
			depth_accuracy: DepthAccuracy::GridSearchLow,
			magnitude_accuracy: MagnitudeAccuracy::SWave,
			epicenter_category: EpicenterCategory::Land,
			warning_status: WarningStatus::Alert,
			intensity_change: IntensityChange::Down,
			change_reason: ChangeReason::Magnitude,
			area_info: vec!{}
		})
	};

	let expected = "[テスト配信 | 警報/最終] 奈良県 震度5強 M6.8 60km (N34.4/E135.7) 09:55:59発生 | 第6報 ND20130808165608";

	let result = ja_format_eew_short(&eew, None);

	assert!(result.is_some());
	assert_eq!(result.unwrap(), expected);
}

#[test]
fn it_should_format_reference_eew()
{
	let eew = EEW {
		source: Source::Tokyo,
		kind: Kind::Reference,
		issued_at: UTC.ymd(2013, 8, 8).and_hms(7, 57, 2),
		occurred_at: UTC.ymd(2013, 8, 8).and_hms(0, 55, 59),
		id: "ND20130808165608".to_owned(),
		status: Status::Last,
		number: 6,
		detail: EEWDetail::Full(FullEEW {
			issue_pattern: IssuePattern::IntensityOnly,
			epicenter_name: "奈良県".to_owned(),
			epicenter: (34.4, 135.7),
			depth: Some(60.0),
			magnitude: Some(6.8),
			maximum_intensity: Some(5.25),
			epicenter_accuracy: EpicenterAccuracy::GridSearchLow,
			depth_accuracy: DepthAccuracy::GridSearchLow,
			magnitude_accuracy: MagnitudeAccuracy::SWave,
			epicenter_category: EpicenterCategory::Land,
			warning_status: WarningStatus::Alert,
			intensity_change: IntensityChange::Down,
			change_reason: ChangeReason::Magnitude,
			area_info: vec!{}
		})
	};

	let expected = "[参考情報 | 警報/最終] 奈良県 震度5強 M6.8 60km (N34.4/E135.7) 09:55:59発生 | 第6報 ND20130808165608";

	let result = ja_format_eew_short(&eew, None);

	assert!(result.is_some());
	assert_eq!(result.unwrap(), expected);
}

#[test]
fn it_should_format_drill_eew()
{
	let eew = EEW {
		source: Source::Tokyo,
		kind: Kind::Drill,
		issued_at: UTC.ymd(2013, 8, 8).and_hms(7, 57, 2),
		occurred_at: UTC.ymd(2013, 8, 8).and_hms(0, 55, 59),
		id: "ND20130808165608".to_owned(),
		status: Status::Last,
		number: 6,
		detail: EEWDetail::Full(FullEEW {
			issue_pattern: IssuePattern::IntensityOnly,
			epicenter_name: "奈良県".to_owned(),
			epicenter: (34.4, 135.7),
			depth: Some(60.0),
			magnitude: Some(6.8),
			maximum_intensity: Some(5.25),
			epicenter_accuracy: EpicenterAccuracy::GridSearchLow,
			depth_accuracy: DepthAccuracy::GridSearchLow,
			magnitude_accuracy: MagnitudeAccuracy::SWave,
			epicenter_category: EpicenterCategory::Land,
			warning_status: WarningStatus::Alert,
			intensity_change: IntensityChange::Down,
			change_reason: ChangeReason::Magnitude,
			area_info: vec!{}
		})
	};

	let expected = "[訓練 | 警報/最終] 奈良県 震度5強 M6.8 60km (N34.4/E135.7) 09:55:59発生 | 第6報 ND20130808165608";

	let result = ja_format_eew_short(&eew, None);

	assert!(result.is_some());
	assert_eq!(result.unwrap(), expected);
}

#[test]
fn it_should_format_drill_cancel_eew()
{
	let eew = EEW {
		source: Source::Tokyo,
		kind: Kind::DrillCancel,
		issued_at: UTC.ymd(2012, 1, 8).and_hms(4, 32, 17),
		occurred_at: UTC.ymd(2012, 1, 8).and_hms(4, 31, 54),
		id: "ND20120108133201".to_owned(),
		status: Status::Normal,
		number: 3,
		detail: EEWDetail::Cancel,
	};

	let expected = "[訓練 | 取消] --- | 第3報 ND20120108133201";

	let result = ja_format_eew_short(&eew, None);

	assert!(result.is_some());
	assert_eq!(result.unwrap(), expected);
}

#[test]
fn it_should_fail_to_format_inconsistent_eew_1()
{
	let eew = EEW {
		source: Source::Tokyo,
		kind: Kind::Test,
		issued_at: UTC.ymd(2012, 1, 8).and_hms(4, 32, 17),
		occurred_at: UTC.ymd(2012, 1, 8).and_hms(4, 31, 54),
		id: "ND20120108133201".to_owned(),
		status: Status::Normal,
		number: 3,
		detail: EEWDetail::Cancel,
	};

	let result = ja_format_eew_short(&eew, None);

	assert!(result.is_none());
}

#[test]
fn it_should_fail_to_format_inconsistent_eew_2()
{
	let eew = EEW {
		source: Source::Tokyo,
		kind: Kind::Normal,
		issued_at: UTC.ymd(2012, 1, 8).and_hms(4, 32, 17),
		occurred_at: UTC.ymd(2012, 1, 8).and_hms(4, 31, 54),
		id: "ND20120108133201".to_owned(),
		status: Status::Normal,
		number: 3,
		detail: EEWDetail::Cancel,
	};

	let result = ja_format_eew_short(&eew, None);

	assert!(result.is_none());
}
