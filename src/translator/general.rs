use std::fmt::Write;

use eew::*;




pub fn format_eew_full(eew: &EEW) -> String
{
	let mut output = String::new();

	write_unwrap!(&mut output, "[EEW: {} - {}]\n", eew.id, eew.number);

	write_unwrap!(&mut output, "issue_pattern: {:?}, source: {:?}, kind: {:?}, \
		issued_at: {:?}, occurred_at: {:?}, status: {:?}\n",
		eew.issue_pattern, eew.source, eew.kind, eew.issued_at, eew.occurred_at, eew.status);

	if let Some(ref detail) = eew.detail {

		write_unwrap!(&mut output, "epicenter_name: {}, epicenter: {:?}, \
			depth: {:?}, magnitude: {:?}, maximum_intensity: {:?}, epicenter_accuracy: {:?}, \
			depth_accuracy: {:?}, magnitude_accuracy: {:?}, epicenter_category: {:?}, \
			warning_status: {:?}, intensity_change: {:?}, change_reason: {:?}\n",
			detail.epicenter_name, detail.epicenter, detail.depth,
			detail.magnitude, detail.maximum_intensity, detail.epicenter_accuracy,
			detail.depth_accuracy, detail.magnitude_accuracy, detail.epicenter_category,
			detail.warning_status, detail.intensity_change, detail.change_reason);

		for area in detail.area_info.iter() {
			write_unwrap!(&mut output, "area_name: {}, minimum_intensity: {:?}, \
				maximum_intensity: {:?}, reach_at: {:?}, warning_status: {:?}, wave_status: {:?}\n",
				area.area_name, area.minimum_intensity, area.maximum_intensity,
				area.reach_at, area.warning_status, area.wave_status);
		}
	}

	output
}
