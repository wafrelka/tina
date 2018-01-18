use std::cmp::Ordering;
use std::fmt::Write;

use chrono::{DateTime, Utc, FixedOffset};
use chrono::format::DelayedFormat;
use chrono::format::strftime::StrftimeItems;

use eew::*;


pub fn format_time(dt: &DateTime<Utc>) -> DelayedFormat<StrftimeItems>
{
	let jst: FixedOffset = FixedOffset::east(9 * 3600); // XXX: want to use const keyword...
	const TIME_FORMAT: &'static str = "%H:%M:%S";
	dt.with_timezone(&jst).format(TIME_FORMAT)
}

pub fn format_position(pos: (f32, f32)) -> String
{
	let lat_h = if pos.0 >= 0.0 { "N" } else { "S" };
	let lon_h = if pos.1 >= 0.0 { "E" } else { "W" };
	format!("{}{:.1}/{}{:.1}", lat_h, pos.0.abs(), lon_h, pos.1.abs())
}

pub fn format_magnitude(m: Option<f32>) -> String
{
	match m {
		None => "M---".to_string(),
		Some(f) => format!("M{:.1}", f)
	}
}

pub fn format_depth(d: Option<f32>) -> String
{
	match d {
		None => "---km".to_string(),
		Some(f) => format!("{:.0}km", f)
	}
}

pub fn format_intensity(intensity: Option<IntensityClass>) -> String
{
	match intensity {
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
	}.to_string()
}

pub fn compare_intensity(eew: &EEW, prev_opt: Option<&EEW>) -> Ordering
{
	let prev_detail_opt = prev_opt.and_then(|p| p.detail.as_ref());

	if eew.detail.is_none() || prev_detail_opt.is_none() {
		return Ordering::Equal;
	}

	let detail = eew.detail.as_ref().unwrap();
	let prev_detail = prev_detail_opt.unwrap();

	match (detail.maximum_intensity, prev_detail.maximum_intensity) {
		(None, None) => Ordering::Equal,
		(None, _) => Ordering::Less,
		(_, None) => Ordering::Greater,
		(Some(lv), Some(rv)) => lv.cmp(&rv),
	}
}

pub fn format_eew_short(eew: &EEW, prev_opt: Option<&EEW>) -> Option<String>
{
	let mut header = String::new();
	let mut body = String::new();
	let mut footer = String::new();

	if eew.is_test() {
		header += "テスト配信 | ";
	}

	if eew.is_drill() {
		header += "訓練 | ";
	}

	let title = match eew.get_eew_phase() {
		Some(EEWPhase::Cancel) => "取消",
		Some(EEWPhase::FastForecast) => "速報",
		Some(EEWPhase::Forecast) => "予報",
		Some(EEWPhase::Alert) => "警報",
		_ => return None,
	};
	header += title;

	let updown = match compare_intensity(eew, prev_opt) {
		Ordering::Greater => "↑",
		Ordering::Less => "↓",
		Ordering::Equal => "",
	};
	header += updown;

	if eew.is_last() {
		header += "/最終";
	}

	match eew.detail {

		None => { body += "---" },

		Some(ref detail) => {

			write_unwrap!(&mut body, "{} {} {} {} ({}) {}発生",
				detail.epicenter_name, format_intensity(detail.maximum_intensity),
				format_magnitude(detail.magnitude), format_depth(detail.depth),
				format_position(detail.epicenter), format_time(&eew.occurred_at));
		},
	}

	write_unwrap!(&mut footer, "第{}報 {}", eew.number, eew.id);

	return Some(format!("[{}] {} | {}", header, body, footer));
}
