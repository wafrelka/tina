extern crate chrono;
extern crate tina;

use tina::*;

mod base_eew;
use base_eew::*;


#[test]
fn it_should_output_none_for_low_accuracy_eew_without_detail()
{
	let eew = EEW {
		issue_pattern: IssuePattern::LowAccuracy,
		detail: None,
		.. make_base_eew()
	};

	let result = eew.get_eew_phase();

	assert_eq!(result, None);
}
