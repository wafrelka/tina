#![cfg(test)]

extern crate chrono;
extern crate tina;

use std::collections::HashMap;

use chrono::*;
use tina::*;


#[test]
fn it_should_parse_cancel_eew()
{
	let telegram = b"39 03 10 120108133217 C11 120108133154 \
		ND20120108133201 NCN003 JD////////////// JN/// \
		/// //// ///// /// // // RK///// RT///// RC///// \
		9999=";

	let epicenter = HashMap::new();
	let area = HashMap::new();

	let expected = EEW {
		source: Source::Tokyo,
		kind: Kind::Cancel,
		issued_at: UTC.ymd(2012, 1, 8).and_hms(4, 32, 17),
		occurred_at: UTC.ymd(2012, 1, 8).and_hms(4, 31, 54),
		id: "ND20120108133201".to_owned(),
		status: Status::Normal,
		number: 3,
		detail: EEWDetail::Cancel,
	};

	let result = parse_jma_format(telegram, &epicenter, &area);

	assert!(result.is_ok());
	assert_eq!(result.unwrap(), expected);
}

#[test]
fn it_should_parse_normal_eew_01()
{
	let telegram = b"36 03 00 130804122905 C11 130804122849 \
		ND20130804122902 NCN001 JD////////////// JN/// \
		287 N380 E1420 010 59 04 RK11311 RT10/// RC///// \
		9999=";

	let mut epicenter = HashMap::new();
	let area = HashMap::new();
	epicenter.insert(b"287".to_owned(), "宮城県沖".to_owned());

	let expected = EEW {
		source: Source::Tokyo,
		kind: Kind::Normal,
		issued_at: UTC.ymd(2013, 8, 4).and_hms(3, 29, 5),
		occurred_at: UTC.ymd(2013, 8, 4).and_hms(3, 28, 49),
		id: "ND20130804122902".to_owned(),
		status: Status::Normal,
		number: 1,
		detail: EEWDetail::Full(FullEEW {
			issue_pattern: IssuePattern::LowAccuracy,
			epicenter_name: "宮城県沖".to_owned(),
			epicenter: (38.0, 142.0),
			depth: Some(10.0),
			magnitude: Some(5.9),
			maximum_intensity: Some(4.0),
			epicenter_accuracy: EpicenterAccuracy::Single,
			depth_accuracy: DepthAccuracy::Single,
			magnitude_accuracy: MagnitudeAccuracy::PWave,
			epicenter_category: EpicenterCategory::Sea,
			warning_status: WarningStatus::Forecast,
			intensity_change: IntensityChange::Unknown,
			change_reason: ChangeReason::Unknown,
			area_info: vec!{}
		})
	};

	let result = parse_jma_format(telegram, &epicenter, &area);

	assert!(result.is_ok());
	assert_eq!(result.unwrap(), expected);
}

#[test]
fn it_should_parse_normal_eew_02()
{
	let telegram = b"37 03 00 130808165702 C11 130808165559 \
		ND20130808165608 NCN006 JD////////////// JN/// \
		540 N344 E1357 060 68 5+ RK33513 RT01/// RC21/// \
		9999=";

	let mut epicenter = HashMap::new();
	let area = HashMap::new();
	epicenter.insert(b"540".to_owned(), "奈良県".to_owned());

	let expected = EEW {
		source: Source::Tokyo,
		kind: Kind::Normal,
		issued_at: UTC.ymd(2013, 8, 8).and_hms(7, 57, 2),
		occurred_at: UTC.ymd(2013, 8, 8).and_hms(7, 55, 59),
		id: "ND20130808165608".to_owned(),
		status: Status::Normal,
		number: 6,
		detail: EEWDetail::Full(FullEEW {
			issue_pattern: IssuePattern::HighAccuracy,
			epicenter_name: "奈良県".to_owned(),
			epicenter: (34.4, 135.7),
			depth: Some(60.0),
			magnitude: Some(6.8),
			maximum_intensity: Some(5.25),
			epicenter_accuracy: EpicenterAccuracy::GridSearchLow,
			depth_accuracy: DepthAccuracy::GridSearchLow,
			magnitude_accuracy: MagnitudeAccuracy::SWave,
			epicenter_category: EpicenterCategory::Land,
			warning_status: WarningStatus::Alert,
			intensity_change: IntensityChange::Down,
			change_reason: ChangeReason::Magnitude,
			area_info: vec!{}
		})
	};

	let result = parse_jma_format(telegram, &epicenter, &area);

	assert!(result.is_ok());
	assert_eq!(result.unwrap(), expected);
}

#[test]
fn it_should_parse_ebi_part()
{
	let telegram = b"37 03 00 130808165702 C11 130808165559 \
		ND20130808165608 NCN006 JD////////////// JN/// \
		540 N344 E1357 060 68 5+ RK33513 RT01/// RC21/// \
		EBI 521 S5+5+ ////// 11 540 S5+5- ////// 11 511 S5+5- ////// 11 \
		550 S5-5- ////// 11 551 S5-04 ////// 11 535 S5-04 ////// 11 \
		391 S0404 ////// 11 620 S0404 ////// 11 563 S0404 165703 10 \
		592 S0404 165703 10 591 S0403 ////// 11 301 S5-// 010101 01 \
		9999=";

	let mut epicenter = HashMap::new();
	let mut area = HashMap::new();
	epicenter.insert(b"540".to_owned(), "奈良県".to_owned());
	area.insert(b"521".to_owned(), "大阪府南部".to_owned());
	area.insert(b"540".to_owned(), "奈良県".to_owned());
	area.insert(b"511".to_owned(), "京都府南部".to_owned());
	area.insert(b"550".to_owned(), "和歌山県北部".to_owned());
	area.insert(b"551".to_owned(), "和歌山県南部".to_owned());
	area.insert(b"535".to_owned(), "兵庫県淡路島".to_owned());
	area.insert(b"391".to_owned(), "石川県加賀".to_owned());
	area.insert(b"620".to_owned(), "愛媛県東予".to_owned());
	area.insert(b"563".to_owned(), "鳥取県西部".to_owned());
	area.insert(b"592".to_owned(), "広島県南西部".to_owned());
	area.insert(b"591".to_owned(), "広島県南東部".to_owned());
	area.insert(b"301".to_owned(), "茨城県南部".to_owned());

	let make_areaeew = |area_name: &str, minimum_intensity: f32, maximum_intensity: Option<f32>,
		reached_at: Option<DateTime<UTC>>, warning: bool, reached: bool|
		-> AreaEEW { AreaEEW {
			area_name: area_name.to_string(),
			minimum_intensity: minimum_intensity,
			maximum_intensity: maximum_intensity,
			reached_at: reached_at,
			warning_status: if warning { WarningStatus::Alert } else { WarningStatus::Forecast },
			wave_status: if reached { WaveStatus::Reached } else { WaveStatus::Unreached }
		} };

	let date = UTC.ymd(2013, 8, 8);

	let expected = vec!{
		make_areaeew("大阪府南部", 5.25, Some(5.25), None, true, true),
		make_areaeew("奈良県", 4.75, Some(5.25), None, true, true),
		make_areaeew("京都府南部", 4.75, Some(5.25), None, true, true),
		make_areaeew("和歌山県北部", 4.75, Some(4.75), None, true, true),
		make_areaeew("和歌山県南部", 4.0, Some(4.75), None, true, true),
		make_areaeew("兵庫県淡路島", 4.0, Some(4.75), None, true, true),
		make_areaeew("石川県加賀", 4.0, Some(4.0), None, true, true),
		make_areaeew("愛媛県東予", 4.0, Some(4.0), None, true, true),
		make_areaeew("鳥取県西部", 4.0, Some(4.0), Some(date.and_hms(7, 57, 3)), true, false),
		make_areaeew("広島県南西部", 4.0, Some(4.0), Some(date.and_hms(7, 57, 3)), true, false),
		make_areaeew("広島県南東部", 3.0, Some(4.0), None, true, true),
		make_areaeew("茨城県南部", 4.75, None, Some(date.and_hms(16, 1, 1)), false, true)
	};

	let full_result = parse_jma_format(telegram, &epicenter, &area);
	assert!(full_result.is_ok());

	let result = match full_result.unwrap().detail {
		EEWDetail::Full(v) => v.area_info,
		EEWDetail::Cancel => panic!(),
	};

	for (r, e) in result.iter().zip(expected.iter()) {
		assert_eq!(r, e);
	}

	assert_eq!(result, expected);
}

#[test]
fn it_should_parse_eew_with_unknown_values()
{
	let telegram = b"37 03 00 160610231341 C11 160610231254 \
		ND20160610231334 NCN001 JD////////////// JN/// \
		432 N354 E1369 /// // // RK6620/ RT00/// RC///// \
		9999=";

	let mut epicenter = HashMap::new();
	let area = HashMap::new();
	epicenter.insert(b"432".to_owned(), "岐阜県美濃中西部".to_owned());

	let expected = EEW {
		source: Source::Tokyo,
		kind: Kind::Normal,
		issued_at: UTC.ymd(2016, 6, 10).and_hms(14, 13, 41),
		occurred_at: UTC.ymd(2016, 6, 10).and_hms(14, 12, 54),
		id: "ND20160610231334".to_owned(),
		status: Status::Normal,
		number: 1,
		detail: EEWDetail::Full(FullEEW {
			issue_pattern: IssuePattern::HighAccuracy,
			epicenter_name: "岐阜県美濃中西部".to_owned(),
			epicenter: (35.4, 136.9),
			depth: None,
			magnitude: None,
			maximum_intensity: None,
			epicenter_accuracy: EpicenterAccuracy::NIEDHigh,
			depth_accuracy: DepthAccuracy::NIEDHigh,
			magnitude_accuracy: MagnitudeAccuracy::NIED,
			epicenter_category: EpicenterCategory::Land,
			warning_status: WarningStatus::Forecast,
			intensity_change: IntensityChange::Unknown,
			change_reason: ChangeReason::Unknown,
			area_info: vec!{}
		})
	};

	let result = parse_jma_format(telegram, &epicenter, &area);

	assert!(result.is_ok());
	assert_eq!(result.unwrap(), expected);
}
