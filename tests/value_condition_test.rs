extern crate chrono;
extern crate tina;

use std::sync::Arc;

use tina::*;

mod eew_builder;
use eew_builder::*;

const DEF_COND: ValueCondition = ValueCondition {
	first: None, succeeding: None, alert: None, last: None, cancel: None, drill: None, test: None,
	phase_changed: None, magnitude_over: None, intensity_over: None,
	intensity_up: None, intensity_down: None,
};

fn check(cond: &Condition, cur: &Arc<EEW>, prev: Option<&Arc<EEW>>) -> bool
{
	let v = match prev {
		Some(a) => vec! { a.clone() },
		None => vec!{}
	};
	cond.is_satisfied(cur, &v)
}

#[test]
fn it_should_return_true_when_no_value_is_supplied()
{
	let eew = Arc::new(EEWBuilder::new().build());
	let cond = ValueCondition { .. DEF_COND };

	assert_eq!(cond.is_satisfied(&eew, &vec!{}), true);
	assert_eq!(cond.is_satisfied(&eew, &vec!{ eew.clone() }), true);
	assert_eq!(cond.is_satisfied(&eew, &vec!{ eew.clone(), eew.clone() }), true);
}

#[test]
fn it_should_handle_first_condition()
{
	let eew = Arc::new(EEWBuilder::new().build());
	let cond = ValueCondition { first: Some(true), .. DEF_COND };

	assert_eq!(cond.is_satisfied(&eew, &vec!{}), true);
	assert_eq!(cond.is_satisfied(&eew, &vec!{ eew.clone() }), false);
	assert_eq!(cond.is_satisfied(&eew, &vec!{ eew.clone(), eew.clone() }), false);
}

#[test]
fn it_should_handle_succeeding_condition()
{
	let eew = Arc::new(EEWBuilder::new().build());
	let cond = ValueCondition { succeeding: Some(true), .. DEF_COND };

	assert_eq!(cond.is_satisfied(&eew, &vec!{}), false);
	assert_eq!(cond.is_satisfied(&eew, &vec!{ eew.clone() }), true);
	assert_eq!(cond.is_satisfied(&eew, &vec!{ eew.clone(), eew.clone() }), true);
}

#[test]
fn it_should_handle_alert_condition()
{
	let bad_eew = Arc::new(EEWBuilder::new().build());
	let good_eew = Arc::new(EEWBuilder::new().warning_status(WarningStatus::Alert).build());
	let cond = ValueCondition { alert: Some(true), .. DEF_COND };

	assert_eq!(check(&cond, &bad_eew, None), false);
	assert_eq!(check(&cond, &good_eew, None), true);
}

#[test]
fn it_should_handle_last_condition()
{
	// TODO: implement test
}

#[test]
fn it_should_handle_cancel_condition()
{
	// TODO: implement test
}

#[test]
fn it_should_handle_drill_condition()
{
	// TODO: implement test
}

#[test]
fn it_should_handle_test_condition()
{
	// TODO: implement test
}

#[test]
fn it_should_handle_magnitude_over_condition()
{
	let detail_none_eew = Arc::new(EEWBuilder::new().detail_none().build());
	let m_none_eew = Arc::new(EEWBuilder::new().magnitude(None).build());
	let bad_eew = Arc::new(EEWBuilder::new().magnitude(Some(4.0)).build());
	let good_eew = Arc::new(EEWBuilder::new().magnitude(Some(4.1)).build());
	let cond = ValueCondition { magnitude_over: Some(4.1), .. DEF_COND };

	assert_eq!(check(&cond, &detail_none_eew, None), false);
	assert_eq!(check(&cond, &m_none_eew, None), false);
	assert_eq!(check(&cond, &bad_eew, None), false);
	assert_eq!(check(&cond, &good_eew, None), true);
}

#[test]
fn it_should_handle_intensity_over_condition()
{
	let detail_none_eew = Arc::new(EEWBuilder::new().detail_none().build());
	let i_none_eew = Arc::new(EEWBuilder::new().maximum_intensity(None).build());
	let bad_eew = Arc::new(EEWBuilder::new().maximum_intensity(Some(IntensityClass::FiveLower)).build());
	let good_eew = Arc::new(EEWBuilder::new().maximum_intensity(Some(IntensityClass::FiveUpper)).build());
	let cond = ValueCondition { intensity_over: Some(IntensityClass::FiveUpper), .. DEF_COND };

	assert_eq!(check(&cond, &detail_none_eew, None), false);
	assert_eq!(check(&cond, &i_none_eew, None), false);
	assert_eq!(check(&cond, &bad_eew, None), false);
	assert_eq!(check(&cond, &good_eew, None), true);
}

#[test]
fn it_should_handle_intensity_up_condition()
{
	let detail_none_eew = Arc::new(EEWBuilder::new().detail_none().build());
	let i_none_eew = Arc::new(EEWBuilder::new().maximum_intensity(None).build());
	let lo_eew = Arc::new(EEWBuilder::new().maximum_intensity(Some(IntensityClass::FiveLower)).build());
	let hi_eew = Arc::new(EEWBuilder::new().maximum_intensity(Some(IntensityClass::FiveUpper)).build());
	let cond = ValueCondition { intensity_up: Some(1), .. DEF_COND };

	assert_eq!(check(&cond, &detail_none_eew, Some(&detail_none_eew)), false);
	assert_eq!(check(&cond, &detail_none_eew, Some(&i_none_eew)), false);
	assert_eq!(check(&cond, &detail_none_eew, Some(&lo_eew)), false);
	assert_eq!(check(&cond, &detail_none_eew, Some(&hi_eew)), false);

	assert_eq!(check(&cond, &i_none_eew, Some(&detail_none_eew)), false);
	assert_eq!(check(&cond, &i_none_eew, Some(&i_none_eew)), false);
	assert_eq!(check(&cond, &i_none_eew, Some(&lo_eew)), false);
	assert_eq!(check(&cond, &i_none_eew, Some(&hi_eew)), false);

	assert_eq!(check(&cond, &lo_eew, Some(&detail_none_eew)), false);
	assert_eq!(check(&cond, &lo_eew, Some(&i_none_eew)), true);
	assert_eq!(check(&cond, &lo_eew, Some(&lo_eew)), false);
	assert_eq!(check(&cond, &lo_eew, Some(&hi_eew)), false);

	assert_eq!(check(&cond, &hi_eew, Some(&detail_none_eew)), false);
	assert_eq!(check(&cond, &hi_eew, Some(&i_none_eew)), true);
	assert_eq!(check(&cond, &hi_eew, Some(&lo_eew)), true);
	assert_eq!(check(&cond, &hi_eew, Some(&hi_eew)), false);
}

#[test]
fn it_should_handle_intensity_up_condition_with_value_2()
{
	let i_none_eew = Arc::new(EEWBuilder::new().maximum_intensity(None).build());
	let zero_eew = Arc::new(EEWBuilder::new().maximum_intensity(Some(IntensityClass::Zero)).build());
	let one_eew = Arc::new(EEWBuilder::new().maximum_intensity(Some(IntensityClass::One)).build());
	let two_eew = Arc::new(EEWBuilder::new().maximum_intensity(Some(IntensityClass::Two)).build());
	let three_eew = Arc::new(EEWBuilder::new().maximum_intensity(Some(IntensityClass::Three)).build());
	let cond = ValueCondition { intensity_up: Some(2), .. DEF_COND };

	assert_eq!(check(&cond, &zero_eew, Some(&i_none_eew)), false);
	assert_eq!(check(&cond, &one_eew, Some(&i_none_eew)), true);

	assert_eq!(check(&cond, &two_eew, Some(&one_eew)), false);
	assert_eq!(check(&cond, &three_eew, Some(&one_eew)), true);
}

#[test]
fn it_should_handle_intensity_down_condition()
{
	let detail_none_eew = Arc::new(EEWBuilder::new().detail_none().build());
	let i_none_eew = Arc::new(EEWBuilder::new().maximum_intensity(None).build());
	let lo_eew = Arc::new(EEWBuilder::new().maximum_intensity(Some(IntensityClass::FiveLower)).build());
	let hi_eew = Arc::new(EEWBuilder::new().maximum_intensity(Some(IntensityClass::FiveUpper)).build());
	let cond = ValueCondition { intensity_down: Some(1), .. DEF_COND };

	assert_eq!(check(&cond, &detail_none_eew, Some(&detail_none_eew)), false);
	assert_eq!(check(&cond, &detail_none_eew, Some(&i_none_eew)), false);
	assert_eq!(check(&cond, &detail_none_eew, Some(&lo_eew)), false);
	assert_eq!(check(&cond, &detail_none_eew, Some(&hi_eew)), false);

	assert_eq!(check(&cond, &i_none_eew, Some(&detail_none_eew)), false);
	assert_eq!(check(&cond, &i_none_eew, Some(&i_none_eew)), false);
	assert_eq!(check(&cond, &i_none_eew, Some(&lo_eew)), true);
	assert_eq!(check(&cond, &i_none_eew, Some(&hi_eew)), true);

	assert_eq!(check(&cond, &lo_eew, Some(&detail_none_eew)), false);
	assert_eq!(check(&cond, &lo_eew, Some(&i_none_eew)), false);
	assert_eq!(check(&cond, &lo_eew, Some(&lo_eew)), false);
	assert_eq!(check(&cond, &lo_eew, Some(&hi_eew)), true);

	assert_eq!(check(&cond, &hi_eew, Some(&detail_none_eew)), false);
	assert_eq!(check(&cond, &hi_eew, Some(&i_none_eew)), false);
	assert_eq!(check(&cond, &hi_eew, Some(&lo_eew)), false);
	assert_eq!(check(&cond, &hi_eew, Some(&hi_eew)), false);
}

#[test]
fn it_should_handle_intensity_down_condition_with_value_2()
{
	let i_none_eew = Arc::new(EEWBuilder::new().maximum_intensity(None).build());
	let zero_eew = Arc::new(EEWBuilder::new().maximum_intensity(Some(IntensityClass::Zero)).build());
	let one_eew = Arc::new(EEWBuilder::new().maximum_intensity(Some(IntensityClass::One)).build());
	let two_eew = Arc::new(EEWBuilder::new().maximum_intensity(Some(IntensityClass::Two)).build());
	let three_eew = Arc::new(EEWBuilder::new().maximum_intensity(Some(IntensityClass::Three)).build());
	let cond = ValueCondition { intensity_down: Some(2), .. DEF_COND };

	assert_eq!(check(&cond, &i_none_eew, Some(&zero_eew)), false);
	assert_eq!(check(&cond, &i_none_eew, Some(&one_eew)), true);

	assert_eq!(check(&cond, &one_eew, Some(&two_eew)), false);
	assert_eq!(check(&cond, &one_eew, Some(&three_eew)), true);
}
