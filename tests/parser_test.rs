#[cfg(test)]

extern crate chrono;
extern crate tina;

use self::chrono::*;
use self::tina::*;


#[test]
fn it_should_parse_cancel_eew()
{
	let telegram = b"39 03 10 120108133217 C11 120108133154 \
		ND20120108133201 NCN003 JD////////////// JN/// \
		/// //// ///// /// // // RK///// RT///// RC///// \
		9999=";

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

	let result = parse_jma_format(telegram);

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

	// XXX: currently we don't care about epicenter_name
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
			epicenter_name: "287".to_owned(),
			epicenter: (38.0, 142.0),
			depth: Some(10.0),
			magnitude: Some(5.9),
			maximum_intensity: Some(4.0),
			epicenter_accuracy: EpicenterAccuracy::Single,
			depth_accuracy: DepthAccuracy::Single,
			magnitude_accuracy: MagnitudeAccuracy::PWave,
			epicenter_caterogy: EpicenterCategory::Sea,
			warning_status: WarningStatus::Forecast,
			intensity_change: IntensityChange::Unknown,
			change_reason: ChangeReason::Unknown,
			area_eew: vec!{}
		})
	};

	let result = parse_jma_format(telegram);

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

	// XXX: currently we don't care about epicenter_name
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
			epicenter_name: "540".to_owned(),
			epicenter: (34.4, 135.7),
			depth: Some(60.0),
			magnitude: Some(6.8),
			maximum_intensity: Some(5.25),
			epicenter_accuracy: EpicenterAccuracy::GridSearchLow,
			depth_accuracy: DepthAccuracy::GridSearchLow,
			magnitude_accuracy: MagnitudeAccuracy::SWave,
			epicenter_caterogy: EpicenterCategory::Land,
			warning_status: WarningStatus::Alert,
			intensity_change: IntensityChange::Down,
			change_reason: ChangeReason::Magnitude,
			area_eew: vec!{}
		})
	};

	let result = parse_jma_format(telegram);

	assert!(result.is_ok());
	assert_eq!(result.unwrap(), expected);
}

#[test]
#[ignore]
fn it_should_parse_ebi_part()
{
	let telegram = b"37 03 00 130808165702 C11 130808165559 \
		ND20130808165608 NCN006 JD////////////// JN/// \
		540 N344 E1357 060 68 5+ RK33513 RT01/// RC21/// \
		EBI 521 S5+5+ ////// 11 520 S5+5+ ////// 11 540 S5+5- ////// 11 \
		511 S5+5- ////// 11 550 S5-5- ////// 11 531 S5-5- ////// 11 \
		461 S5-5- ////// 11 462 S5-5- ////// 11 501 S5-5- ////// 11 \
		610 S5-5- ////// 11 551 S5-04 ////// 11 535 S5-04 ////// 11 \
		460 S5-04 ////// 11 451 S5-04 ////// 11 581 S5-04 ////// 11 \
		532 S0404 ////// 11 600 S0404 ////// 11 601 S0404 ////// 11 \
		500 S0404 ////// 11 510 S0404 ////// 11 401 S0404 ////// 11 \
		530 S0404 ////// 11 432 S0404 ////// 11 450 S0404 ////// 11 \
		580 S0404 ////// 11 431 S0404 ////// 11 611 S0404 ////// 11 \
		560 S0404 ////// 11 400 S0404 ////// 11 443 S0404 ////// 11 \
		630 S0404 ////// 11 562 S0404 ////// 11 631 S0404 ////// 11 \
		391 S0404 ////// 11 620 S0404 ////// 11 563 S0404 165703 10 \
		592 S0404 165703 10 591 S0403 ////// 11 \
		9999=";

	let expected = vec!{};

	let full_result = parse_jma_format(telegram);
	assert!(full_result.is_ok());

	let result = match full_result.unwrap().detail {
		EEWDetail::Full(v) => v.area_eew,
		EEWDetail::Cancel => panic!(),
	};

	assert_eq!(result, expected);
}
