use eew::*;


pub fn format_eew_full(eew: &EEW) -> String
{
	let head = format!("[EEW: {} - {}]\n", eew.id, eew.number);
	let base = format!("source: {:?}, kind: {:?}, issued_at: {:?}, occurred_at: {:?}, status: {:?}\n",
		eew.source, eew.kind, eew.issued_at, eew.occurred_at, eew.status);

	let extended = match eew.detail {

		EEWDetail::Cancel => "".to_string(),

		EEWDetail::Full(ref detail) => {

			let global = format!("issue_pattern: {:?}, epicenter_name: {}, epicenter: {:?}, \
				depth: {:?}, magnitude: {:?}, maximum_intensity: {:?}, epicenter_accuracy: {:?}, \
				depth_accuracy: {:?}, magnitude_accuracy: {:?}, epicenter_category: {:?}, \
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
