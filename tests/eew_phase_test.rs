extern crate chrono;
extern crate tina;

use tina::*;

mod eew_builder;
use eew_builder::*;


#[test]
fn it_should_output_none_for_low_accuracy_eew_without_detail()
{
	let eew = EEWBuilder::new()
		.issue_pattern(IssuePattern::LowAccuracy)
		.detail_none()
		.build();

	let result = eew.get_eew_phase();

	assert_eq!(result, None);
}
