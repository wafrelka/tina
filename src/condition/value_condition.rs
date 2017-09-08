use std::sync::Arc;

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
	pub accuracy_changed: Option<bool>,

	pub magnitude_over: Option<f32>,
	pub intensity_over: Option<IntensityClass>,

	pub magnitude_up: Option<f32>,
	pub magnitude_down: Option<f32>,
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

fn test_detail<V, F>(expected: Option<V>, latest: &Arc<EEW>, f: F) -> bool
	where F: FnOnce(V, &EEWDetail) -> bool
{
	match expected {
		None => true,
		Some(v) => latest.detail.as_ref().map_or(false, |d| f(v, d)),
	}
}

fn test_with_prev<V, F>(expected: Option<V>, latest: &Arc<EEW>, prev: Option<&Arc<EEW>>, f: F) -> bool
	where F: FnOnce(V, &Arc<EEW>, &Arc<EEW>) -> bool
{
	match (expected, prev) {
		(None, _) => true,
		(Some(_), None) => false,
		(Some(v), Some(eew)) => f(v, latest, eew),
	}
}

fn test_with_prev_detail<V, F>(expected: Option<V>, latest: &Arc<EEW>, prev: Option<&Arc<EEW>>, f: F) -> bool
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

	fn is_satisfied(&self, latest: &Arc<EEW>, prevs: &[Arc<EEW>]) -> bool
	{
		let prev = prevs.iter().last();

		let simple_conds = [
			test_bool(self.first, prevs.len() <= 0),
			test_bool(self.succeeding, prevs.len() >= 1),
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
			test_with_prev(self.accuracy_changed, latest, prev,
				|v, latest, prev| (latest.is_high_accuracy() != prev.is_high_accuracy()) == v),
			test_with_prev_detail(self.magnitude_up, latest, prev,
				|v, latest, prev| match (latest.magnitude, prev.magnitude) {
					(Some(x), Some(y)) => (x - y) >= v,
					_ => false
				}),
			test_with_prev_detail(self.magnitude_down, latest, prev,
				|v, latest, prev| match (latest.magnitude, prev.magnitude) {
					(Some(x), Some(y)) => (y - x) >= v,
					_ => false
				}),
			test_with_prev_detail(self.intensity_up, latest, prev,
				|v, latest, prev| {
					let l_v = latest.maximum_intensity.map_or(-1, |i| i.ord());
					let p_v = prev.maximum_intensity.map_or(-1, |i| i.ord());
					let diff = l_v - p_v;
					diff >= v as i32
				}),
			test_with_prev_detail(self.intensity_down, latest, prev,
				|v, latest, prev| {
					let l_v = latest.maximum_intensity.map_or(-1, |i| i.ord());
					let p_v = prev.maximum_intensity.map_or(-1, |i| i.ord());
					let diff = p_v - l_v;
					diff >= v as i32
				}),
		];

		simple_conds.into_iter().all(|&v| v) && comp_conds.into_iter().all(|&v| v)
	}
}
