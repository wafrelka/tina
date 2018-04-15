extern crate chrono;
extern crate tina;

use tina::*;

mod eew_builder;
use eew_builder::*;

#[test]
fn it_should_handle_phase_none_for_low_accuracy_eew_without_detail()
{
	let eew = EEWBuilder::new()
		.issue_pattern(IssuePattern::LowAccuracy)
		.detail_none()
		.build();

	assert_eq!(eew.get_eew_phase(), None);
}

#[test]
fn it_should_handle_phases()
{
	let eew_cancel_x = EEWBuilder::new().kind(Kind::Cancel).build();
	let eew_cancel_y = EEWBuilder::new().kind(Kind::DrillCancel).build();

	let eew_alert_x = EEWBuilder::new().warning_status(WarningStatus::Alert)
		.issue_pattern(IssuePattern::HighAccuracy).build();
	let eew_alert_y = EEWBuilder::new().warning_status(WarningStatus::Alert)
		.issue_pattern(IssuePattern::LowAccuracy).build();
	let eew_alert_z = EEWBuilder::new().warning_status(WarningStatus::Alert)
		.issue_pattern(IssuePattern::IntensityOnly).build();

	let eew_forecast = EEWBuilder::new().warning_status(WarningStatus::Forecast)
		.issue_pattern(IssuePattern::HighAccuracy).build();
	let eew_fast_x = EEWBuilder::new().warning_status(WarningStatus::Forecast)
		.issue_pattern(IssuePattern::LowAccuracy).build();
	let eew_fast_y = EEWBuilder::new().warning_status(WarningStatus::Forecast)
		.issue_pattern(IssuePattern::IntensityOnly).build();

	assert_eq!(eew_cancel_x.get_eew_phase(), Some(EEWPhase::Cancel));
	assert_eq!(eew_cancel_y.get_eew_phase(), Some(EEWPhase::Cancel));
	assert_eq!(eew_alert_x.get_eew_phase(), Some(EEWPhase::Alert));
	assert_eq!(eew_alert_y.get_eew_phase(), Some(EEWPhase::Alert));
	assert_eq!(eew_alert_z.get_eew_phase(), Some(EEWPhase::Alert));
	assert_eq!(eew_forecast.get_eew_phase(), Some(EEWPhase::Forecast));
	assert_eq!(eew_fast_x.get_eew_phase(), Some(EEWPhase::FastForecast));
	assert_eq!(eew_fast_y.get_eew_phase(), Some(EEWPhase::FastForecast));
}

#[test]
fn it_should_handle_successor_condition_for_cancel_eew()
{
	let eew_2 = EEWBuilder::new().number(2).build();
	let eew_1_cancel = EEWBuilder::new().number(1).kind(Kind::Cancel).build();
	let eew_2_cancel = EEWBuilder::new().number(2).kind(Kind::Cancel).build();
	let eew_3_cancel = EEWBuilder::new().number(3).kind(Kind::Cancel).build();

	assert_eq!(eew_2.is_succeeded_by(&eew_1_cancel), false);
	assert_eq!(eew_2.is_succeeded_by(&eew_2_cancel), true);
	assert_eq!(eew_2.is_succeeded_by(&eew_3_cancel), true);
}

#[test]
fn it_should_handle_successor_condition_for_normal_eew()
{
	let eew_1 = EEWBuilder::new().number(1).build();
	let eew_2 = EEWBuilder::new().number(2).build();
	let eew_other = EEWBuilder::new().id("A").number(2).build();

	assert_eq!(eew_1.is_succeeded_by(&eew_1), false);
	assert_eq!(eew_1.is_succeeded_by(&eew_2), true);
	assert_eq!(eew_2.is_succeeded_by(&eew_1), false);
	assert_eq!(eew_2.is_succeeded_by(&eew_2), false);

	assert_eq!(eew_1.is_succeeded_by(&eew_other), false);
	assert_eq!(eew_2.is_succeeded_by(&eew_other), false);
	assert_eq!(eew_other.is_succeeded_by(&eew_1), false);
	assert_eq!(eew_other.is_succeeded_by(&eew_2), false);
}
