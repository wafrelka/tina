use chrono::*;

use eew::*;
use translator::eew_extension::*;


pub fn format_time(dt: &DateTime<UTC>) -> format::DelayedFormat<format::strftime::StrftimeItems>
{
	let jst: FixedOffset = FixedOffset::east(9 * 3600); // XXX: want to use const keyword...
	const TIME_FORMAT: &'static str = "%H:%M:%S";
	return dt.with_timezone(&jst).format(TIME_FORMAT);
}

pub fn format_eew_number(eew: &EEW) -> String
{
	match eew.status {
		Status::LastWithCorrection | Status::Last => "最終報".to_string(),
		_ => format!("第{}報", eew.number)
	}
}

pub fn format_position(pos: (f32, f32)) -> String
{
	let lat_h = if pos.0 >= 0.0 { "N" } else { "S" };
	let lon_h = if pos.1 >= 0.0 { "E" } else { "W" };
	return format!("{}{:.1}/{}{:.1}", lat_h, pos.0.abs(), lon_h, pos.1.abs());
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

pub fn format_intensity(i: Option<IntensityClass>) -> String
{
	match i {
		None => "震度不明",
		Some(c) => match c {
			IntensityClass::Zero => "震度0",
			IntensityClass::One => "震度1",
			IntensityClass::Two => "震度2",
			IntensityClass::Three => "震度3",
			IntensityClass::Four => "震度4",
		 	IntensityClass::FiveMinus => "震度5弱",
			IntensityClass::FivePlus => "震度5強",
			IntensityClass::SixMinus => "震度6弱",
			IntensityClass::SixPlus => "震度6強",
			IntensityClass::Seven => "震度7"
		}
	}.to_string()
}

pub fn format_eew_short(eew: &EEW) -> Option<String>
{
	match eew.kind {
		Kind::Drill | Kind::DrillCancel => return None,
		Kind::Reference | Kind::Test => return None,
		_ => {}
	};

	if eew.get_eew_phase() == Some(EEWPhase::Cancel) {
		return Some(format!("[取消] --- / {} {}", format_eew_number(eew), eew.id));
	}

	let ref id = eew.id;
	let num_str = format_eew_number(eew);

	let head = match eew.get_eew_phase() {
		Some(EEWPhase::FastForecast) => "予報(速報)",
		Some(EEWPhase::Forecast) => "予報",
		Some(EEWPhase::FastAlert) => "警報(速報)",
		Some(EEWPhase::Alert) => "警報",
		_ => "不明"
	};

	let detail_str = match eew.detail {

		EEWDetail::Cancel => "---".to_string(),

		EEWDetail::Full(ref detail) => {

			let updown = match detail.intensity_change {
				IntensityChange::Up => "↑",
				IntensityChange::Down => "↓",
				_ => ""
			};
			let intensity = format_intensity(eew.get_maximum_intensity_class()) + updown;

			format!("{} {} {} {} ({})",
				detail.epicenter_name, intensity,
				format_magnitude(detail.magnitude), format_depth(detail.depth),
				format_position(detail.epicenter))
		}
	};

	let s = format!("[{}] {} {}発生 / {} {}",
		head, detail_str, format_time(&eew.occurred_at), num_str, id);

	return Some(s);
}

pub fn format_eew_detailed(eew: &EEW) -> String
{
	let head = format!("[EEW: {} - {}]\n", eew.id, eew.number);
	let base = format!("source: {:?}, kind: {:?}, issued_at: {:?}, occurred_at: {:?}, status: {:?}\n",
		eew.source, eew.kind, eew.issued_at, eew.occurred_at, eew.status);

	let extended = match eew.detail {

		EEWDetail::Cancel => "".to_string(),

		EEWDetail::Full(ref detail) => {

			let global = format!("issue_pattern: {:?}, epicenter_name: {}, epicenter: {:?}, \
				depth: {:?}, magnitude: {:?}, maximum_intensity: {:?}, epicenter_accuracy: {:?}, \
				depth_accuracy: {:?}, magnitude_accuracy: {:?}, epicenter_category: {:?} \
				warning_status: {:?}, intensity_change: {:?}, change_reason: {:?}\n",
				detail.issue_pattern, detail.epicenter_name, detail.epicenter, detail.depth,
				detail.magnitude, detail.maximum_intensity, detail.epicenter_accuracy,
				detail.depth_accuracy, detail.magnitude_accuracy, detail.epicenter_category,
				detail.warning_status, detail.intensity_change, detail.change_reason);

			let areas = detail.area_info.iter().map(|area|
				format!("area_name: {}, minimum_intensity: {:?}, maximum_intensity: {:?}, \
				reached_at: {:?}, warning_status: {:?}, wave_status: {:?}\n",
				area.area_name, area.minimum_intensity, area.maximum_intensity,
				area.reached_at, area.warning_status, area.wave_status)
			).collect::<Vec<_>>().concat();

			global + &areas
		}
	};

	return head + &base + &extended;
}
