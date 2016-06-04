extern crate chrono;

use std::str;
use std::ascii::AsciiExt;
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

	return Err(JMAFormatParseError::InvalidPattern);
}
