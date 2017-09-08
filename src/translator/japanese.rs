use std::cmp::Ordering;

use chrono::*;

use eew::*;


pub fn format_time(dt: &DateTime<UTC>) -> format::DelayedFormat<format::strftime::StrftimeItems>
{
	let jst: FixedOffset = FixedOffset::east(9 * 3600); // XXX: want to use const keyword...
	const TIME_FORMAT: &'static str = "%H:%M:%S";
	dt.with_timezone(&jst).format(TIME_FORMAT)
}

pub fn format_eew_number(eew: &EEW) -> String
{
	format!("第{}報", eew.number)
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

pub fn compare_intensity(l: Option<IntensityClass>, r: Option<IntensityClass>) -> Ordering
{
	match (l, r) {
		(None, None) => Ordering::Equal,
		(None, _) => Ordering::Less,
		(_, None) => Ordering::Greater,
		(Some(lv), Some(rv)) => lv.cmp(&rv),
	}
}

pub fn format_eew_short(eew: &EEW, prev_opt: Option<&EEW>) -> Option<String>
{
	match eew.kind {
		Kind::Cancel => return Some(format!("[取消] --- | {} {}", format_eew_number(eew), eew.id)),
		Kind::DrillCancel => return Some(format!("[訓練 | 取消] --- | {} {}", format_eew_number(eew), eew.id)),
		_ => {}
	}

	let reliable = match eew.issue_pattern {
		IssuePattern::LowAccuracy | IssuePattern::HighAccuracy => true,
		_ => false,
	};

	let head = match (eew.get_eew_phase(), reliable) {
		(Some(EEWPhase::Forecast), true) => "予報",
		(Some(EEWPhase::Forecast), false) => "速報",
		(Some(EEWPhase::Alert), _) => "警報",
		_ => return None
	};

	let id = &eew.id;
	let num_str = format_eew_number(eew);

	let prev_detail = prev_opt.and_then(|p| p.detail.as_ref());
	let comp = prev_detail.and_then(|pd|
		eew.detail.as_ref().map(|ed|
			compare_intensity(ed.maximum_intensity, pd.maximum_intensity)
		)
	);
	let updown = match comp {
		Some(Ordering::Greater) => "↑",
		Some(Ordering::Less) => "↓",
		_ => ""
	};

	let detail_str = match eew.detail {

		None => "---".to_string(),

		Some(ref detail) => {

			let intensity = format_intensity(detail.maximum_intensity);

			format!("{} {} {} {} ({})",
				detail.epicenter_name, intensity,
				format_magnitude(detail.magnitude), format_depth(detail.depth),
				format_position(detail.epicenter))
		}
	};

	let last_str = if eew.is_last() { "/最終" } else { "" };
	let kind_str = match eew.kind {
		Kind::Reference => "参考情報 | ",
		Kind::Test => "テスト配信 | ",
		Kind::Drill => "訓練 | ",
		_ => ""
	};

	let s = format!("[{}{}{}{}] {} {}発生 | {} {}",
		kind_str, head, updown, last_str, detail_str, format_time(&eew.occurred_at), num_str, id);

	return Some(s);
}
