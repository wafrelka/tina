extern crate chrono;

use std::str;
use self::chrono::*;
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
}

fn parse_datetime(datetime_text: &[u8]) -> Option<DateTime<UTC>>
{
	let JST: FixedOffset = FixedOffset::east(9 * 3600); // XXX: want to use const keyword...
	let DATETIME_FORMAT: &'static str = "%y%m%d%H%M%S";

	return str::from_utf8(&datetime_text).ok().and_then( |converted|
		JST.datetime_from_str(&converted, DATETIME_FORMAT).ok().map( |dt|
			dt.with_timezone(&UTC)
		)
	);
}

fn parse_number(number_text: &[u8]) -> Option<u32>
{
	return str::from_utf8(&number_text).ok().and_then( |converted|
		converted.parse().ok()
	);
}

fn parse_epicenter_code(code: &[u8]) -> Option<String>
{
	return str::from_utf8(&code).map( |s| s.to_string()).ok();
}

fn parse_area_code(code: &[u8]) -> Option<String>
{
	return str::from_utf8(&code).map( |s| s.to_string()).ok();
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

pub fn parse_jma_format(text: &[u8]) -> Result<EEW, JMAFormatParseError>
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
	if &text[23..25] != b"11" {
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

	let epicenter_name = match parse_epicenter_code(&text[86..89]) {
		Some(e) => e,
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

	let depth = match parse_number(&text[101..104]) {
		Some(v) => Some(v as f32),
		None => match &text[100..103] {
			b"///" => None,
			_ => return Err(JMAFormatParseError::InvalidDepth)
		}
	};

	let magnitude = match parse_number(&text[105..107]) {
		Some(v) => Some((v as f32) / 10.0),
		None => match &text[104..106] {
			b"///" => None,
			_ => return Err(JMAFormatParseError::InvalidMagnitude)
		}
	};

	let maximum_intensity = match parse_intensity(&text[108..110]) {
		Some(v) => Some(v),
		None => match &text[107..109] {
			b"//" => None,
			_ => return Err(JMAFormatParseError::InvalidMaximumIntensity)
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

	// TODO: parse EBI part

	let area_eew = vec! {};

	let detail = FullEEW {

		issue_pattern: issue_pattern,
		epicenter_name: epicenter_name.to_string(),
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

		area_eew: area_eew
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
