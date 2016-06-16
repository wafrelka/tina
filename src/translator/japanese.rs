use chrono::*;

use eew::*;


pub fn format_datetime(dt: &DateTime<UTC>) -> format::DelayedFormat<format::strftime::StrftimeItems>
{
	let jst: FixedOffset = FixedOffset::east(9 * 3600); // XXX: want to use const keyword...
	const DATETIME_FORMAT: &'static str = "%H:%M:%S";
	return dt.with_timezone(&jst).format(DATETIME_FORMAT);
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

pub fn format_intensity(i: Option<f32>) -> String
{
	match i {
		None => "震度不明",
		Some(f) if f < 0.5 => "震度0",
		Some(f) if f < 1.5 => "震度1",
		Some(f) if f < 2.5 => "震度2",
		Some(f) if f < 3.5 => "震度3",
		Some(f) if f < 4.5 => "震度4",
		Some(f) if f < 5.0 => "震度5弱",
		Some(f) if f < 5.5 => "震度5強",
		Some(f) if f < 6.0 => "震度6弱",
		Some(f) if f < 6.5 => "震度6強",
		Some(_) => "震度7",
	}.to_string()
}

pub fn format_eew_short(eew: &EEW) -> Option<String>
{
	match eew.kind {
		Kind::Drill | Kind::DrillCancel => return None,
		Kind::Reference | Kind::Test => return None,
		_ => {}
	};

	let mut s = String::new();

	if eew.kind == Kind::Cancel {

		s.push_str(&format!("[取消] --- / {} {}", format_eew_number(eew), eew.id));
		return Some(s);
	}

	let mut head = "速報";
	let ref id = eew.id;
	let num_str = format_eew_number(eew);

	let mut detail_str = "---".to_string();

	if let EEWDetail::Full(ref detail) = eew.detail {

		let updown = match detail.intensity_change {
			IntensityChange::Up => "↑",
			IntensityChange::Down => "↓",
			_ => ""
		};
		let intensity = format_intensity(detail.maximum_intensity) + updown;

		detail_str = format!("{} {} {} {} ({})",
			detail.epicenter_name, intensity,
			format_magnitude(detail.magnitude), format_depth(detail.depth),
			format_position(detail.epicenter));

		head = match detail.warning_status {
			WarningStatus::Alert => match detail.issue_pattern {
				IssuePattern::HighAccuracy => "警報",
				IssuePattern::LowAccuracy | IssuePattern::IntensityOnly => "警報(速報)",
			},
			WarningStatus::Forecast => match detail.issue_pattern {
				IssuePattern::HighAccuracy => "予報",
				IssuePattern::LowAccuracy | IssuePattern::IntensityOnly => "予報(速報)",
			},
			_ => "不明"
		};
	}

	s.push_str(&format!("[{}] {} {}発生 / {} {}",
		head, detail_str, format_datetime(&eew.occurred_at), num_str, id));

	return Some(s);
}

pub fn format_eew_detailed(eew: &EEW) -> String
{
	let mut s = String::new();

	s.push_str(&format!("[EEW: {} - {}]\n", eew.id, eew.number));
	s.push_str(&format!("source: {:?}, kind: {:?}, issued_at: {:?}, occurred_at: {:?}, status: {:?}\n",
		eew.source, eew.kind, eew.issued_at, eew.occurred_at, eew.status));

	if let EEWDetail::Full(ref detail) = eew.detail {

		s.push_str(&format!("issue_pattern: {:?}, epicenter_name: {}, epicenter: {:?}, depth: {:?}, \
			magnitude: {:?}, maximum_intensity: {:?}, epicenter_accuracy: {:?}, \
			depth_accuracy: {:?}, magnitude_accuracy: {:?}, epicenter_caterogy: {:?} \
			warning_status: {:?}, intensity_change: {:?}, change_reason: {:?}\n",
			detail.issue_pattern, detail.epicenter_name, detail.epicenter, detail.depth,
			detail.magnitude, detail.maximum_intensity, detail.epicenter_accuracy,
			detail.depth_accuracy, detail.magnitude_accuracy, detail.epicenter_caterogy,
			detail.warning_status, detail.intensity_change, detail.change_reason));

		for ref area in &detail.area_info {

			s.push_str(&format!("area_name: {}, minimum_intensity: {:?}, maximum_intensity: {:?}, \
				reached_at: {:?}, warning_status: {:?}, wave_status: {:?}\n",
				area.area_name, area.minimum_intensity, area.maximum_intensity,
				area.reached_at, area.warning_status, area.wave_status));
		}
	}

	return s;
}
