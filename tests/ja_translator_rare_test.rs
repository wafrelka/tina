extern crate chrono;
extern crate tina;

use tina::*;

mod eew_builder;
use eew_builder::*;

#[test]
fn it_should_format_eew_with_south_west_epicenter()
{
	let eew = EEWBuilder::new()
		.epicenter((-34.4, -135.7))
		.build();

	let expected =
		"[予報] 奈良県 震度5弱 M5.9 10km (S34.4/W135.7) 09:55:59発生 \
		| 第10報 ND20100101005559".to_owned();

	let result = ja_format_eew_short(&eew, None);

	assert_eq!(result, Some(expected));
}

#[test]
fn it_should_format_drill_eew()
{
	let eew = EEWBuilder::new()
		.kind(Kind::Drill)
		.build();

	let expected =
		"[訓練 | 予報] 奈良県 震度5弱 M5.9 10km (N34.4/E135.7) 09:55:59発生 \
		| 第10報 ND20100101005559".to_owned();

	let result = ja_format_eew_short(&eew, None);

	assert_eq!(result, Some(expected));
}

#[test]
fn it_should_format_drill_cancel_eew()
{
	let eew = EEWBuilder::new()
		.issue_pattern(IssuePattern::Cancel)
		.kind(Kind::DrillCancel)
		.detail_none()
		.build();

	let expected =
		"[訓練 | 取消] --- | 第10報 ND20100101005559".to_owned();

	let result = ja_format_eew_short(&eew, None);

	assert_eq!(result, Some(expected));
}

#[test]
fn it_should_format_reference_eew()
{
	let eew = EEWBuilder::new()
		.kind(Kind::Reference)
		.build();

	let expected =
		"[テスト配信 | 予報] 奈良県 震度5弱 M5.9 10km (N34.4/E135.7) 09:55:59発生 \
		| 第10報 ND20100101005559".to_owned();

	let result = ja_format_eew_short(&eew, None);

	assert_eq!(result, Some(expected));
}

#[test]
fn it_should_format_trial_eew()
{
	let eew = EEWBuilder::new()
		.kind(Kind::Trial)
		.build();

	let expected =
		"[テスト配信 | 予報] 奈良県 震度5弱 M5.9 10km (N34.4/E135.7) 09:55:59発生 \
		| 第10報 ND20100101005559".to_owned();

	let result = ja_format_eew_short(&eew, None);

	assert_eq!(result, Some(expected));
}
