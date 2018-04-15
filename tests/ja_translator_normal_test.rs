extern crate chrono;
extern crate tina;

use tina::*;

mod eew_builder;
use eew_builder::*;


#[test]
fn it_should_format_cancel_eew()
{
	let eew = EEWBuilder::new()
		.issue_pattern(IssuePattern::Cancel)
		.kind(Kind::Cancel)
		.detail_none()
		.build();

	let expected = "[取消] --- | 第10報 NDXXXX".to_owned();

	let result = ja_format_eew_oneline(&eew, None);

	assert_eq!(result, Some(expected));
}

#[test]
fn it_should_format_intensity_only_eew()
{
	let eew = EEWBuilder::new()
		.issue_pattern(IssuePattern::IntensityOnly)
		.magnitude(None)
		.build();

	let expected =
		"[速報] 奈良県 震度5弱 M--- 10km (N34.4/E135.7) 09:55:59発生 | 第10報 NDXXXX".to_owned();

	let result = ja_format_eew_oneline(&eew, None);

	assert_eq!(result, Some(expected));
}

#[test]
fn it_should_format_low_accuracy_eew()
{
	let eew = EEWBuilder::new()
		.issue_pattern(IssuePattern::LowAccuracy)
		.build();

	let expected =
		"[速報] 奈良県 震度5弱 M5.9 10km (N34.4/E135.7) 09:55:59発生 | 第10報 NDXXXX".to_owned();

	let result = ja_format_eew_oneline(&eew, None);

	assert_eq!(result, Some(expected));
}

#[test]
fn it_should_format_high_accuracy_eew()
{
	let eew = EEWBuilder::new()
		.build();

	let expected =
		"[予報] 奈良県 震度5弱 M5.9 10km (N34.4/E135.7) 09:55:59発生 | 第10報 NDXXXX".to_owned();

	let result = ja_format_eew_oneline(&eew, None);

	assert_eq!(result, Some(expected));
}

#[test]
fn it_should_format_alert_eew_with_intensity_only()
{
	let eew = EEWBuilder::new()
		.issue_pattern(IssuePattern::IntensityOnly)
		.magnitude(None)
		.warning_status(WarningStatus::Alert)
		.build();

	let expected =
		"[警報] 奈良県 震度5弱 M--- 10km (N34.4/E135.7) 09:55:59発生 | 第10報 NDXXXX".to_owned();

	let result = ja_format_eew_oneline(&eew, None);

	assert_eq!(result, Some(expected));
}

#[test]
fn it_should_format_alert_eew_with_low_accuracy()
{
	let eew = EEWBuilder::new()
		.issue_pattern(IssuePattern::LowAccuracy)
		.warning_status(WarningStatus::Alert)
		.build();

	let expected =
		"[警報] 奈良県 震度5弱 M5.9 10km (N34.4/E135.7) 09:55:59発生 | 第10報 NDXXXX".to_owned();

	let result = ja_format_eew_oneline(&eew, None);

	assert_eq!(result, Some(expected));
}


#[test]
fn it_should_format_alert_eew_with_high_accuracy()
{
	let eew = EEWBuilder::new()
		.issue_pattern(IssuePattern::HighAccuracy)
		.warning_status(WarningStatus::Alert)
		.build();

	let expected =
		"[警報] 奈良県 震度5弱 M5.9 10km (N34.4/E135.7) 09:55:59発生 | 第10報 NDXXXX".to_owned();

	let result = ja_format_eew_oneline(&eew, None);

	assert_eq!(result, Some(expected));
}

#[test]
fn it_should_format_last_eew()
{
	let eew = EEWBuilder::new()
		.status(Status::Last)
		.warning_status(WarningStatus::Alert)
		.build();

	let expected =
		"[警報/最終] 奈良県 震度5弱 M5.9 10km (N34.4/E135.7) 09:55:59発生 | 第10報 NDXXXX".to_owned();

	let result = ja_format_eew_oneline(&eew, None);

	assert_eq!(result, Some(expected));
}

#[test]
fn it_should_format_last_with_correction_eew()
{
	let eew = EEWBuilder::new()
		.status(Status::LastWithCorrection)
		.warning_status(WarningStatus::Alert)
		.build();

	let expected =
		"[警報/最終] 奈良県 震度5弱 M5.9 10km (N34.4/E135.7) 09:55:59発生 | 第10報 NDXXXX".to_owned();

	let result = ja_format_eew_oneline(&eew, None);

	assert_eq!(result, Some(expected));
}

#[test]
fn it_should_format_correction_eew()
{
	let eew = EEWBuilder::new()
		.status(Status::Correction)
		.warning_status(WarningStatus::Alert)
		.build();

	let expected =
		"[警報] 奈良県 震度5弱 M5.9 10km (N34.4/E135.7) 09:55:59発生 | 第10報 NDXXXX".to_owned();

	let result = ja_format_eew_oneline(&eew, None);

	assert_eq!(result, Some(expected));
}

#[test]
fn it_should_format_eew_with_unknown_values()
{
	let eew = EEWBuilder::new()
		.depth(None)
		.magnitude(None)
		.maximum_intensity(None)
		.build();

	let expected =
		"[予報] 奈良県 震度不明 M--- ---km (N34.4/E135.7) 09:55:59発生 | 第10報 NDXXXX".to_owned();

	let result = ja_format_eew_oneline(&eew, None);

	assert_eq!(result, Some(expected));
}
