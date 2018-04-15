use eew::{EEW, EEWPhase, EEWDetail, IntensityClass};
use condition::Condition;


pub struct ValueCondition {

	pub first: Option<bool>,
	pub succeeding: Option<bool>,

	pub alert: Option<bool>,
	pub last: Option<bool>,
	pub cancel: Option<bool>,
	pub drill: Option<bool>,
	pub test: Option<bool>,

	pub phase_changed: Option<bool>,
	pub epicenter_name_changed: Option<bool>,

	pub magnitude_over: Option<f32>,
	pub intensity_over: Option<IntensityClass>,

	pub intensity_up: Option<u8>,
	pub intensity_down: Option<u8>,
}

fn test_bool(expected: Option<bool>, actual: bool) -> bool
{
	match expected {
		None => true,
		Some(v) => v == actual,
	}
}

fn test_detail<V, F>(expected: Option<V>, latest: &EEW, f: F) -> bool
	where F: FnOnce(V, &EEWDetail) -> bool
{
	match expected {
		None => true,
		Some(v) => latest.detail.as_ref().map_or(false, |d| f(v, d)),
	}
}

fn test_with_prev<V, F>(expected: Option<V>, latest: &EEW, prev: Option<&EEW>, f: F) -> bool
	where F: FnOnce(V, &EEW, &EEW) -> bool
{
	match (expected, prev) {
		(None, _) => true,
		(Some(_), None) => false,
		(Some(v), Some(eew)) => f(v, latest, eew),
	}
}

fn test_with_prev_detail<V, F>(expected: Option<V>, latest: &EEW, prev: Option<&EEW>, f: F) -> bool
	where F: FnOnce(V, &EEWDetail, &EEWDetail) -> bool
{
	match (expected, prev) {
		(None, _) => true,
		(Some(_), None) => false,
		(Some(v), Some(eew)) =>
			latest.detail.as_ref().and_then(|ld| eew.detail.as_ref().map(|ed| f(v, &ld, &ed))).unwrap_or(false),
	}
}

impl Condition for ValueCondition {

	fn is_satisfied(&self, latest: &EEW, prev: Option<&EEW>) -> bool
	{
		let simple_conds = [
			test_bool(self.first, prev.is_none()),
			test_bool(self.succeeding, prev.is_some()),
			test_bool(self.alert, latest.get_eew_phase() == Some(EEWPhase::Alert)),
			test_bool(self.last, latest.is_last()),
			test_bool(self.cancel, latest.get_eew_phase() == Some(EEWPhase::Cancel)),
			test_bool(self.drill, latest.is_drill()),
			test_bool(self.test, latest.is_test()),
			test_detail(self.magnitude_over, latest, |v, detail| detail.magnitude.map_or(false, |m| m >= v)),
			test_detail(self.intensity_over, latest, |v, detail| detail.maximum_intensity.map_or(false, |m| m >= v)),
		];

		let comp_conds = [
			test_with_prev(self.phase_changed, latest, prev,
				|v, latest, prev| (latest.get_eew_phase() != prev.get_eew_phase()) == v),
			test_with_prev_detail(self.epicenter_name_changed, latest, prev,
				|v, latest, prev| {
					(latest.epicenter_name != prev.epicenter_name) == v
				}),
			test_with_prev_detail(self.intensity_up, latest, prev,
				|v, latest, prev| {
					let l_v = latest.maximum_intensity.map_or(-1, |i| i as i32);
					let p_v = prev.maximum_intensity.map_or(-1, |i| i as i32);
					let diff = l_v - p_v;
					diff >= (v as i32)
				}),
			test_with_prev_detail(self.intensity_down, latest, prev,
				|v, latest, prev| {
					let l_v = latest.maximum_intensity.map_or(-1, |i| i as i32);
					let p_v = prev.maximum_intensity.map_or(-1, |i| i as i32);
					let diff = p_v - l_v;
					diff >= (v as i32)
				}),
		];

		simple_conds.into_iter().all(|&v| v) && comp_conds.into_iter().all(|&v| v)
	}
}
