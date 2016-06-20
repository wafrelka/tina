use std::str;
use std::collections::HashMap;

use chrono::*;

use eew::*;


#[derive(Debug)]
pub enum JMAFormatParseError {
	TooShort,
	Split,
	InvalidPattern,
	InvalidSource,
	InvalidKind,
	InvalidIssueTime,
	InvalidOoccurrenceTime,
	InvalidId,
	InvalidStatus,
	InvalidNumber,
	UnknownEpicenter,
	InvalidLL,
	InvalidDepth,
	InvalidMagnitude,
	InvalidMaximumIntensity,
	InvalidEpicenterAccuracy,
	InvalidDepthAccuracy,
	InvalidMagnitudeAccuracy,
	InvalidEpicenterCategory,
	InvalidWarningStatus,
	InvalidIntensityChange,
	InvalidChangeReason,
	InvalidWaveStatus,
	InvalidEBI,
	PrematureEOS,
}

fn parse_datetime(datetime_text: &[u8]) -> Option<DateTime<UTC>>
{
	let jst: FixedOffset = FixedOffset::east(9 * 3600); // XXX: want to use const keyword...
	const DATETIME_FORMAT: &'static str = "%y%m%d%H%M%S";

	return str::from_utf8(&datetime_text).ok().and_then( |converted|
		jst.datetime_from_str(&converted, DATETIME_FORMAT).ok().map( |dt|
			dt.with_timezone(&UTC)
		)
	);
}

fn parse_number(number_text: &[u8]) -> Option<u32>
{
	str::from_utf8(&number_text).ok().and_then( |converted|
		converted.parse().ok()
	)
}

fn parse_intensity(intensity_text: &[u8]) -> Option<f32>
{
	match intensity_text {
		b"01" => Some(1.0),
		b"02" => Some(2.0),
		b"03" => Some(3.0),
		b"04" => Some(4.0),
		b"5-" => Some(4.75),
		b"5+" => Some(5.25),
		b"6-" => Some(5.75),
		b"6+" => Some(6.25),
		b"07" => Some(7.0),
		_ => None
	}
}

fn parse_arrival_time(arrival_text: &[u8], base: &DateTime<UTC>) -> Option<DateTime<UTC>>
{
	let jst: FixedOffset = FixedOffset::east(9 * 3600); // XXX: want to use const keyword...
	const TIME_FORMAT: &'static str = "%H%M%S";

	let adjust = |a_t: NaiveTime| {
		let base_t = base.with_timezone(&jst).time();
		let diff = a_t - base_t;
		if diff < Duration::seconds(0) {
			return base.checked_add(Duration::days(1) + diff);
		} else {
			return base.checked_add(diff);
		}
	};

	return str::from_utf8(&arrival_text).ok().and_then( |converted|
		NaiveTime::parse_from_str(&converted, TIME_FORMAT).ok().and_then( |t|
			adjust(t)
		)
	);
}

pub fn parse_jma_format(text: &[u8],
	epicenter_code_dict: &HashMap<[u8; 3], String>,
	area_code_dict: &HashMap<[u8; 3], String>) -> Result<EEW, JMAFormatParseError>
{
	if text.len() < 140 {
		return Err(JMAFormatParseError::TooShort);
	}

	let source = match &text[3..5] {
		b"03" => Source::Tokyo,
		b"04" => Source::Osaka,
		_ => return Err(JMAFormatParseError::InvalidSource)
	};

	let kind = match &text[6..8] {
		b"00" => Kind::Normal,
		b"01" => Kind::Drill,
		b"10" => Kind::Cancel,
		b"11" => Kind::DrillCancel,
		b"20" => Kind::Reference,
		b"30" => Kind::Test,
		_ => return Err(JMAFormatParseError::InvalidKind)
	};

	let issued_at = match parse_datetime(&text[9..21]) {
		Some(dt) => dt,
		None => return Err(JMAFormatParseError::InvalidIssueTime)
	};

	// TODO: accept split telegrams
	if text[24] != b'1' {
		return Err(JMAFormatParseError::Split);
	}

	let occurred_at = match parse_datetime(&text[26..38]) {
		Some(dt) => dt,
		None => return Err(JMAFormatParseError::InvalidOoccurrenceTime)
	};

	let id = match str::from_utf8(&text[39..55]) {
		Ok(s) => s,
		Err(_) => return Err(JMAFormatParseError::InvalidId)
	};

	let status = match text[59] {
		b'0' => Status::Normal,
		b'6' => Status::Correction,
		b'7' => Status::CancelCorrection,
		b'8' => Status::LastWithCorrection,
		b'9' => Status::Last,
		b'/' => Status::Unknown,
		_ => return Err(JMAFormatParseError::InvalidStatus)
	};

	// we don't accept an EEW which has no telegram number
	let number = match parse_number(&text[60..62]) {
		Some(n) => n,
		None => return Err(JMAFormatParseError::InvalidNumber)
	};

	if &text[0..2] == b"39" {

		return Ok(EEW {
			source: source,
			kind: kind,
			issued_at: issued_at,
			occurred_at: occurred_at,
			id: id.to_string(),
			status: status,
			number: number,
			detail: EEWDetail::Cancel,
		});
	}

	let issue_pattern = match &text[0..2] {
		b"35" => IssuePattern::IntensityOnly,
		b"36" => IssuePattern::LowAccuracy,
		b"37" => IssuePattern::HighAccuracy,
		_ => return Err(JMAFormatParseError::InvalidPattern)
	};

	let epicenter_name = match epicenter_code_dict.get(&text[86..89]) {
		Some(s) => s.clone(),
		None => return Err(JMAFormatParseError::UnknownEpicenter)
	};

	let lat_value = match parse_number(&text[91..94]) {
		Some(v) => (v as f32) / 10.0,
		None => return Err(JMAFormatParseError::InvalidLL)
	};

	let lat = match text[90] {
		b'N' =>  lat_value,
		b'S' => -lat_value,
		_ => return Err(JMAFormatParseError::InvalidLL)
	};

	let lon_value = match parse_number(&text[96..100]) {
		Some(v) => (v as f32) / 10.0,
		None => return Err(JMAFormatParseError::InvalidLL)
	};

	let lon = match text[95] {
		b'E' =>  lon_value,
		b'W' => -lon_value,
		_ => return Err(JMAFormatParseError::InvalidLL)
	};

	let depth = {
		let t = &text[101..104];
		match parse_number(t) {
			Some(v) => Some(v as f32),
			None => match t {
				b"///" => None,
				_ => return Err(JMAFormatParseError::InvalidDepth)
			}
		}
	};

	let magnitude = {
		let t = &text[105..107];
		match parse_number(t) {
			Some(v) => Some((v as f32) / 10.0),
			None => match t {
				b"//" => None,
				_ => return Err(JMAFormatParseError::InvalidMagnitude)
			}
		}
	};

	let maximum_intensity = {
		let t = &text[108..110];
		match parse_intensity(t) {
			Some(v) => Some(v),
			None => match t {
				b"//" => None,
				_ => return Err(JMAFormatParseError::InvalidMaximumIntensity)
			}
		}
	};

	let epicenter_accuracy = match text[113] {
		b'1' => EpicenterAccuracy::Single,
		b'2' => EpicenterAccuracy::Territory,
		b'3' => EpicenterAccuracy::GridSearchLow,
		b'4' => EpicenterAccuracy::GridSearchHigh,
		b'5' => EpicenterAccuracy::NIEDLow,
		b'6' => EpicenterAccuracy::NIEDHigh,
		b'7' => EpicenterAccuracy::EPOSLow,
		b'8' => EpicenterAccuracy::EPOSHigh,
		b'/' => EpicenterAccuracy::Unknown,
		_ => return Err(JMAFormatParseError::InvalidEpicenterAccuracy)
	};

	let depth_accuracy = match text[114] {
		b'1' => DepthAccuracy::Single,
		b'2' => DepthAccuracy::Territory,
		b'3' => DepthAccuracy::GridSearchLow,
		b'4' => DepthAccuracy::GridSearchHigh,
		b'5' => DepthAccuracy::NIEDLow,
		b'6' => DepthAccuracy::NIEDHigh,
		b'7' => DepthAccuracy::EPOSLow,
		b'8' => DepthAccuracy::EPOSHigh,
		b'/' => DepthAccuracy::Unknown,
		_ => return Err(JMAFormatParseError::InvalidDepthAccuracy)
	};

	let magnitude_accuracy = match text[115] {
		b'2' => MagnitudeAccuracy::NIED,
		b'3' => MagnitudeAccuracy::PWave,
		b'4' => MagnitudeAccuracy::PSMixed,
		b'5' => MagnitudeAccuracy::SWave,
		b'6' => MagnitudeAccuracy::EPOS,
		b'8' => MagnitudeAccuracy::Level,
		b'/' => MagnitudeAccuracy::Unknown,
		_ => return Err(JMAFormatParseError::InvalidMagnitudeAccuracy)
	};

	let epicenter_caterogy = match text[121] {
		b'0' => EpicenterCategory::Land,
		b'1' => EpicenterCategory::Sea,
		b'/' => EpicenterCategory::Unknown,
		_ => return Err(JMAFormatParseError::InvalidEpicenterCategory)
	};

	let warning_status = match text[122] {
		b'0' => WarningStatus::Forecast,
		b'1' => WarningStatus::Alert,
		b'/' => WarningStatus::Unknown,
		_ => return Err(JMAFormatParseError::InvalidWarningStatus)
	};

	let intensity_change = match text[129] {
		b'0' => IntensityChange::Same,
		b'1' => IntensityChange::Up,
		b'2' => IntensityChange::Down,
		b'/' => IntensityChange::Unknown,
		_ => return Err(JMAFormatParseError::InvalidIntensityChange)
	};

	let change_reason = match text[130] {
		b'0' => ChangeReason::Nothing,
		b'1' => ChangeReason::Magnitude,
		b'2' => ChangeReason::Epicenter,
		b'3' => ChangeReason::Mixed,
		b'4' => ChangeReason::Depth,
		b'/' => ChangeReason::Unknown,
		_ => return Err(JMAFormatParseError::InvalidChangeReason)
	};

	let mut area_info = vec! {};

	if &text[135..138] == b"EBI" {

		let ebi_part_len = 20;
		let mut it = 138;

		while it + ebi_part_len < text.len() {

			if &text[(it+1)..(it+6)] == b"9999=" {
				break;
			}

			let part = &text[it..(it+ebi_part_len)];

			let area_name = match area_code_dict.get(&part[1..4]) {
				Some(s) => s.clone(),
				None => return Err(JMAFormatParseError::InvalidEBI)
			};

			let left_intensity = match parse_intensity(&part[6..8]) {
				Some(v) => v,
				None => return Err(JMAFormatParseError::InvalidEBI)
			};

			let right_intensity = {
				let t = &part[8..10];
				match parse_intensity(t) {
					Some(v) => Some(v),
					None => match t {
						b"//" => None,
						_ => return Err(JMAFormatParseError::InvalidEBI)
					}
				}
			};

			let (minimum_intensity, maximum_intensity) = match right_intensity {
				Some(v) => (v, Some(left_intensity)),
				None => (left_intensity, None)
			};

			let reached_at = {
				let t = &part[11..17];
				match parse_arrival_time(t, &occurred_at) {
					Some(v) => Some(v),
					None => match t {
						b"//////" => None,
						_ => return Err(JMAFormatParseError::InvalidEBI)
					}
				}
			};

			let local_warning_status = match part[18] {
				b'0' => WarningStatus::Forecast,
				b'1' => WarningStatus::Alert,
				b'/' => WarningStatus::Unknown,
				_ => return Err(JMAFormatParseError::InvalidEBI)
			};

			let wave_status = match part[19] {
				b'0' => WaveStatus::Unreached,
				b'1' => WaveStatus::Reached,
				b'/' => WaveStatus::Unknown,
				_ => return Err(JMAFormatParseError::InvalidEBI)
			};

			let area_eew = AreaEEW {
				area_name: area_name,
				minimum_intensity: minimum_intensity,
				maximum_intensity: maximum_intensity,
				reached_at: reached_at,
				warning_status: local_warning_status,
				wave_status: wave_status,
			};

			area_info.push(area_eew);

			it += ebi_part_len;
		}

		if it + 5 >= text.len() || &text[(it+1)..(it+6)] != b"9999=" {
			return Err(JMAFormatParseError::PrematureEOS);
		}
	}

	let detail = FullEEW {

		issue_pattern: issue_pattern,
		epicenter_name: epicenter_name,
		epicenter: (lat, lon),
		depth: depth,
		magnitude: magnitude,
		maximum_intensity: maximum_intensity,
		epicenter_accuracy: epicenter_accuracy,
		depth_accuracy: depth_accuracy,
		magnitude_accuracy: magnitude_accuracy,
		epicenter_caterogy: epicenter_caterogy,
		warning_status: warning_status,
		intensity_change: intensity_change,
		change_reason: change_reason,

		area_info: area_info
	};

	return Ok(EEW{
		source: source,
		kind: kind,
		issued_at: issued_at,
		occurred_at: occurred_at,
		id: id.to_string(),
		status: status,
		number: number,
		detail: EEWDetail::Full(detail),
	});
}
