extern crate chrono;
extern crate tina;

use tina::*;

mod eew_builder;
use eew_builder::*;

enum Diff { Up, Down, Same }

fn expect_normal_eew_string(maximum_intensity: Option<IntensityClass>, updown: Diff) -> String
{
	let i_str = match maximum_intensity {
		None => "震度不明",
		Some(IntensityClass::Zero) => "震度0",
		Some(IntensityClass::One) => "震度1",
		Some(IntensityClass::Two) => "震度2",
		Some(IntensityClass::Three) => "震度3",
		Some(IntensityClass::Four) => "震度4",
		Some(IntensityClass::FiveLower) => "震度5弱",
		Some(IntensityClass::FiveUpper) => "震度5強",
		Some(IntensityClass::SixLower) => "震度6弱",
		Some(IntensityClass::SixUpper) => "震度6強",
		Some(IntensityClass::Seven) => "震度7"
	};
	let updown_str = match updown {
		Diff::Up => "↑",
		Diff::Down => "↓",
		Diff::Same => ""
	};

	format!("[予報{}] 奈良県 {} M5.9 10km (N34.4/E135.7) 09:55:59発生 | 第10報 NDXXXX", updown_str, i_str)
}

fn expect_cancel_eew_string() -> String
{
	"[取消] --- | 第10報 NDXXXX".to_string()
}

#[test]
fn it_should_format_with_intensity_same()
{
	let eew1 = EEWBuilder::new().maximum_intensity(Some(IntensityClass::Four)).build();
	let eew2 = EEWBuilder::new().maximum_intensity(Some(IntensityClass::Four)).build();
	let expected = expect_normal_eew_string(Some(IntensityClass::Four), Diff::Same);
	let result = ja_format_eew_oneline(&eew2, Some(&eew1));

	assert_eq!(result, Some(expected));
}

#[test]
fn it_should_format_with_intensity_up()
{
	let eew1 = EEWBuilder::new().maximum_intensity(Some(IntensityClass::Zero)).build();
	let eew2 = EEWBuilder::new().maximum_intensity(Some(IntensityClass::Four)).build();
	let expected = expect_normal_eew_string(Some(IntensityClass::Four), Diff::Up);
	let result = ja_format_eew_oneline(&eew2, Some(&eew1));

	assert_eq!(result, Some(expected));
}

#[test]
fn it_should_format_with_intensity_up_from_unknown()
{
	let eew1 = EEWBuilder::new().maximum_intensity(None).build();
	let eew2 = EEWBuilder::new().maximum_intensity(Some(IntensityClass::Zero)).build();
	let expected = expect_normal_eew_string(Some(IntensityClass::Zero), Diff::Up);
	let result = ja_format_eew_oneline(&eew2, Some(&eew1));

	assert_eq!(result, Some(expected));
}

#[test]
fn it_should_format_with_intensity_down()
{
	let eew1 = EEWBuilder::new().maximum_intensity(Some(IntensityClass::Four)).build();
	let eew2 = EEWBuilder::new().maximum_intensity(Some(IntensityClass::Zero)).build();
	let expected = expect_normal_eew_string(Some(IntensityClass::Zero), Diff::Down);
	let result = ja_format_eew_oneline(&eew2, Some(&eew1));

	assert_eq!(result, Some(expected));
}

#[test]
fn it_should_format_with_intensity_down_to_unknown()
{
	let eew1 = EEWBuilder::new().maximum_intensity(Some(IntensityClass::Four)).build();
	let eew2 = EEWBuilder::new().maximum_intensity(None).build();
	let expected = expect_normal_eew_string(None, Diff::Down);
	let result = ja_format_eew_oneline(&eew2, Some(&eew1));

	assert_eq!(result, Some(expected));
}

#[test]
fn it_should_format_from_normal_to_cancel()
{
	let eew1 = EEWBuilder::new().maximum_intensity(Some(IntensityClass::Four)).build();
	let eew2 = EEWBuilder::new().issue_pattern(IssuePattern::Cancel).kind(Kind::Cancel).detail_none().build();
	let expected = expect_cancel_eew_string();
	let result = ja_format_eew_oneline(&eew2, Some(&eew1));

	assert_eq!(result, Some(expected));
}

#[test]
fn it_should_format_from_cancel_to_normal()
{
	let eew1 = EEWBuilder::new().issue_pattern(IssuePattern::Cancel).kind(Kind::Cancel).detail_none().build();
	let eew2 = EEWBuilder::new().maximum_intensity(Some(IntensityClass::Four)).build();
	let expected = expect_normal_eew_string(Some(IntensityClass::Four), Diff::Same);
	let result = ja_format_eew_oneline(&eew2, Some(&eew1));

	assert_eq!(result, Some(expected));
}
