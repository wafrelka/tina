extern crate chrono;
extern crate tina;

use tina::*;

mod eew_builder;
use eew_builder::*;

const DEF_COND: ValueCondition = ValueCondition {
	first: None, succeeding: None, alert: None, last: None, cancel: None, drill: None, test: None,
	phase_changed: None, epicenter_name_changed: None, magnitude_over: None, intensity_over: None,
	intensity_up: None, intensity_down: None,
};

#[test]
fn it_should_return_true_when_no_value_is_supplied()
{
	let eew = EEWBuilder::new().build();
	let cond = ValueCondition { .. DEF_COND };

	assert_eq!(cond.is_satisfied(&eew, None), true);
	assert_eq!(cond.is_satisfied(&eew, Some(&eew)), true);
}

#[test]
fn it_should_handle_first_condition()
{
	let eew = EEWBuilder::new().build();
	let cond = ValueCondition { first: Some(true), .. DEF_COND };

	assert_eq!(cond.is_satisfied(&eew, None), true);
	assert_eq!(cond.is_satisfied(&eew, Some(&eew)), false);
}

#[test]
fn it_should_handle_succeeding_condition()
{
	let eew = EEWBuilder::new().build();
	let cond = ValueCondition { succeeding: Some(true), .. DEF_COND };

	assert_eq!(cond.is_satisfied(&eew, None), false);
	assert_eq!(cond.is_satisfied(&eew, Some(&eew)), true);
}

#[test]
fn it_should_handle_alert_condition()
{
	let bad_eew = EEWBuilder::new().build();
	let good_eew = EEWBuilder::new().warning_status(WarningStatus::Alert).build();
	let cond = ValueCondition { alert: Some(true), .. DEF_COND };

	assert_eq!(cond.is_satisfied(&bad_eew, None), false);
	assert_eq!(cond.is_satisfied(&good_eew, None), true);
}

#[test]
#[ignore]
fn it_should_handle_last_condition()
{
	unimplemented!()
}

#[test]
#[ignore]
fn it_should_handle_cancel_condition()
{
	unimplemented!()
}

#[test]
#[ignore]
fn it_should_handle_drill_condition()
{
	unimplemented!()
}

#[test]
#[ignore]
fn it_should_handle_test_condition()
{
	unimplemented!()
}

#[test]
#[ignore]
fn it_should_handle_phase_changed_condition()
{
	unimplemented!()
}

#[test]
fn it_should_handle_epicenter_name_changed_condition()
{
	let a_eew = EEWBuilder::new().epicenter_name("A").build();
	let b_eew = EEWBuilder::new().epicenter_name("B").build();
	let cond = ValueCondition { epicenter_name_changed: Some(true), .. DEF_COND };

	assert_eq!(cond.is_satisfied(&a_eew, None), false);
	assert_eq!(cond.is_satisfied(&a_eew, Some(&a_eew)), false);
	assert_eq!(cond.is_satisfied(&a_eew, Some(&b_eew)), true);
}

#[test]
fn it_should_handle_magnitude_over_condition()
{
	let detail_none_eew = EEWBuilder::new().detail_none().build();
	let m_none_eew = EEWBuilder::new().magnitude(None).build();
	let bad_eew = EEWBuilder::new().magnitude(Some(4.0)).build();
	let good_eew = EEWBuilder::new().magnitude(Some(4.1)).build();
	let cond = ValueCondition { magnitude_over: Some(4.1), .. DEF_COND };

	assert_eq!(cond.is_satisfied(&detail_none_eew, None), false);
	assert_eq!(cond.is_satisfied(&m_none_eew, None), false);
	assert_eq!(cond.is_satisfied(&bad_eew, None), false);
	assert_eq!(cond.is_satisfied(&good_eew, None), true);
}

#[test]
fn it_should_handle_intensity_over_condition()
{
	let detail_none_eew = EEWBuilder::new().detail_none().build();
	let i_none_eew = EEWBuilder::new().maximum_intensity(None).build();
	let bad_eew = EEWBuilder::new().maximum_intensity(Some(IntensityClass::FiveLower)).build();
	let good_eew = EEWBuilder::new().maximum_intensity(Some(IntensityClass::FiveUpper)).build();
	let cond = ValueCondition { intensity_over: Some(IntensityClass::FiveUpper), .. DEF_COND };

	assert_eq!(cond.is_satisfied(&detail_none_eew, None), false);
	assert_eq!(cond.is_satisfied(&i_none_eew, None), false);
	assert_eq!(cond.is_satisfied(&bad_eew, None), false);
	assert_eq!(cond.is_satisfied(&good_eew, None), true);
}

#[test]
fn it_should_handle_intensity_up_condition()
{
	let detail_none_eew = EEWBuilder::new().detail_none().build();
	let i_none_eew = EEWBuilder::new().maximum_intensity(None).build();
	let lo_eew = EEWBuilder::new().maximum_intensity(Some(IntensityClass::FiveLower)).build();
	let hi_eew = EEWBuilder::new().maximum_intensity(Some(IntensityClass::FiveUpper)).build();
	let cond = ValueCondition { intensity_up: Some(1), .. DEF_COND };

	assert_eq!(cond.is_satisfied(&detail_none_eew, Some(&detail_none_eew)), false);
	assert_eq!(cond.is_satisfied(&detail_none_eew, Some(&i_none_eew)), false);
	assert_eq!(cond.is_satisfied(&detail_none_eew, Some(&lo_eew)), false);
	assert_eq!(cond.is_satisfied(&detail_none_eew, Some(&hi_eew)), false);

	assert_eq!(cond.is_satisfied(&i_none_eew, Some(&detail_none_eew)), false);
	assert_eq!(cond.is_satisfied(&i_none_eew, Some(&i_none_eew)), false);
	assert_eq!(cond.is_satisfied(&i_none_eew, Some(&lo_eew)), false);
	assert_eq!(cond.is_satisfied(&i_none_eew, Some(&hi_eew)), false);

	assert_eq!(cond.is_satisfied(&lo_eew, Some(&detail_none_eew)), false);
	assert_eq!(cond.is_satisfied(&lo_eew, Some(&i_none_eew)), true);
	assert_eq!(cond.is_satisfied(&lo_eew, Some(&lo_eew)), false);
	assert_eq!(cond.is_satisfied(&lo_eew, Some(&hi_eew)), false);

	assert_eq!(cond.is_satisfied(&hi_eew, Some(&detail_none_eew)), false);
	assert_eq!(cond.is_satisfied(&hi_eew, Some(&i_none_eew)), true);
	assert_eq!(cond.is_satisfied(&hi_eew, Some(&lo_eew)), true);
	assert_eq!(cond.is_satisfied(&hi_eew, Some(&hi_eew)), false);
}

#[test]
fn it_should_handle_intensity_up_condition_with_value_2()
{
	let i_none_eew = EEWBuilder::new().maximum_intensity(None).build();
	let zero_eew = EEWBuilder::new().maximum_intensity(Some(IntensityClass::Zero)).build();
	let one_eew = EEWBuilder::new().maximum_intensity(Some(IntensityClass::One)).build();
	let two_eew = EEWBuilder::new().maximum_intensity(Some(IntensityClass::Two)).build();
	let three_eew = EEWBuilder::new().maximum_intensity(Some(IntensityClass::Three)).build();
	let cond = ValueCondition { intensity_up: Some(2), .. DEF_COND };

	assert_eq!(cond.is_satisfied(&zero_eew, Some(&i_none_eew)), false);
	assert_eq!(cond.is_satisfied(&one_eew, Some(&i_none_eew)), true);

	assert_eq!(cond.is_satisfied(&two_eew, Some(&one_eew)), false);
	assert_eq!(cond.is_satisfied(&three_eew, Some(&one_eew)), true);
}

#[test]
fn it_should_handle_intensity_down_condition()
{
	let detail_none_eew = EEWBuilder::new().detail_none().build();
	let i_none_eew = EEWBuilder::new().maximum_intensity(None).build();
	let lo_eew = EEWBuilder::new().maximum_intensity(Some(IntensityClass::FiveLower)).build();
	let hi_eew = EEWBuilder::new().maximum_intensity(Some(IntensityClass::FiveUpper)).build();
	let cond = ValueCondition { intensity_down: Some(1), .. DEF_COND };

	assert_eq!(cond.is_satisfied(&detail_none_eew, Some(&detail_none_eew)), false);
	assert_eq!(cond.is_satisfied(&detail_none_eew, Some(&i_none_eew)), false);
	assert_eq!(cond.is_satisfied(&detail_none_eew, Some(&lo_eew)), false);
	assert_eq!(cond.is_satisfied(&detail_none_eew, Some(&hi_eew)), false);

	assert_eq!(cond.is_satisfied(&i_none_eew, Some(&detail_none_eew)), false);
	assert_eq!(cond.is_satisfied(&i_none_eew, Some(&i_none_eew)), false);
	assert_eq!(cond.is_satisfied(&i_none_eew, Some(&lo_eew)), true);
	assert_eq!(cond.is_satisfied(&i_none_eew, Some(&hi_eew)), true);

	assert_eq!(cond.is_satisfied(&lo_eew, Some(&detail_none_eew)), false);
	assert_eq!(cond.is_satisfied(&lo_eew, Some(&i_none_eew)), false);
	assert_eq!(cond.is_satisfied(&lo_eew, Some(&lo_eew)), false);
	assert_eq!(cond.is_satisfied(&lo_eew, Some(&hi_eew)), true);

	assert_eq!(cond.is_satisfied(&hi_eew, Some(&detail_none_eew)), false);
	assert_eq!(cond.is_satisfied(&hi_eew, Some(&i_none_eew)), false);
	assert_eq!(cond.is_satisfied(&hi_eew, Some(&lo_eew)), false);
	assert_eq!(cond.is_satisfied(&hi_eew, Some(&hi_eew)), false);
}

#[test]
fn it_should_handle_intensity_down_condition_with_value_2()
{
	let i_none_eew = EEWBuilder::new().maximum_intensity(None).build();
	let zero_eew = EEWBuilder::new().maximum_intensity(Some(IntensityClass::Zero)).build();
	let one_eew = EEWBuilder::new().maximum_intensity(Some(IntensityClass::One)).build();
	let two_eew = EEWBuilder::new().maximum_intensity(Some(IntensityClass::Two)).build();
	let three_eew = EEWBuilder::new().maximum_intensity(Some(IntensityClass::Three)).build();
	let cond = ValueCondition { intensity_down: Some(2), .. DEF_COND };

	assert_eq!(cond.is_satisfied(&i_none_eew, Some(&zero_eew)), false);
	assert_eq!(cond.is_satisfied(&i_none_eew, Some(&one_eew)), true);

	assert_eq!(cond.is_satisfied(&one_eew, Some(&two_eew)), false);
	assert_eq!(cond.is_satisfied(&one_eew, Some(&three_eew)), true);
}
