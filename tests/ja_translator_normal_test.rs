extern crate chrono;
extern crate tina;

use chrono::*;
use tina::*;

mod eew_builder;
use eew_builder::*;


struct MiniEEW {
	issue_pattern: IssuePattern,
	kind: Kind,
	status: Status,
	detail: Option<MiniEEWDetail>,
}

struct MiniEEWDetail {
	depth: Option<f32>,
	magnitude: Option<f32>,
	maximum_intensity: Option<IntensityClass>,
	warning_status: WarningStatus,
}

impl From<MiniEEW> for EEW {

	fn from(v: MiniEEW) -> EEW
	{
		EEW {
			issue_pattern: v.issue_pattern, source: Source::Tokyo, kind: v.kind,
			issued_at: UTC.ymd(2010, 1, 1).and_hms(1, 0, 2),
			occurred_at: UTC.ymd(2010, 1, 1).and_hms(0, 55, 59),
			id: "ND20100101005559".to_owned(), status: v.status, number: 10,
			detail: v.detail.map(|d| d.into()),
		}
	}
}

impl From<MiniEEWDetail> for EEWDetail {

	fn from(v: MiniEEWDetail) -> EEWDetail
	{
		EEWDetail {
			epicenter_name: "奈良県".to_owned(), epicenter: (34.4, 135.7),
			depth: v.depth, magnitude: v.magnitude,
			maximum_intensity: v.maximum_intensity,
			epicenter_accuracy: EpicenterAccuracy::GridSearchLow,
			depth_accuracy: DepthAccuracy::GridSearchLow,
			magnitude_accuracy: MagnitudeAccuracy::SWave,
			epicenter_category: EpicenterCategory::Land,
			warning_status: v.warning_status,
			intensity_change: IntensityChange::Unknown,
			change_reason: ChangeReason::Unknown,
			area_info: vec!{},
		}
	}
}


#[test]
fn it_should_format_cancel_eew()
{
	let eew = EEWBuilder::new()
		.issue_pattern(IssuePattern::Cancel)
		.kind(Kind::Cancel)
		.detail_none()
		.build();

	let expected = "[取消] --- | 第10報 NDXXXX".to_owned();

	let result = ja_format_eew_short(&eew, None);

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

	let result = ja_format_eew_short(&eew, None);

	assert_eq!(result, Some(expected));
}

#[test]
fn it_should_format_low_accuracy_eew()
{
	let eew = EEWBuilder::new()
		.issue_pattern(IssuePattern::LowAccuracy)
		.build();

	let expected =
		"[予報] 奈良県 震度5弱 M5.9 10km (N34.4/E135.7) 09:55:59発生 | 第10報 NDXXXX".to_owned();

	let result = ja_format_eew_short(&eew, None);

	assert_eq!(result, Some(expected));
}

#[test]
fn it_should_format_high_accuracy_eew()
{
	let eew = EEWBuilder::new()
		.build();

	let expected =
		"[予報] 奈良県 震度5弱 M5.9 10km (N34.4/E135.7) 09:55:59発生 | 第10報 NDXXXX".to_owned();

	let result = ja_format_eew_short(&eew, None);

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

	let result = ja_format_eew_short(&eew, None);

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

	let result = ja_format_eew_short(&eew, None);

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

	let result = ja_format_eew_short(&eew, None);

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

	let result = ja_format_eew_short(&eew, None);

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

	let result = ja_format_eew_short(&eew, None);

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

	let result = ja_format_eew_short(&eew, None);

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

	let result = ja_format_eew_short(&eew, None);

	assert_eq!(result, Some(expected));
}
