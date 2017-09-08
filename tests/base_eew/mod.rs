use chrono::*;
use tina::*;

pub fn make_base_eew() -> EEW
{
	EEW {
		issue_pattern: IssuePattern::HighAccuracy, source: Source::Tokyo, kind: Kind::Normal,
		issued_at: UTC.ymd(2010, 1, 1).and_hms(1, 0, 2),
		occurred_at: UTC.ymd(2010, 1, 1).and_hms(0, 55, 59),
		id: "ND20100101005559".to_owned(), status: Status::Normal, number: 10,
		detail: Some(make_base_eew_detail()),
	}
}

pub fn make_base_eew_detail() -> EEWDetail
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
