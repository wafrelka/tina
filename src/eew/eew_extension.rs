use eew::*;


#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone, Copy)]
pub enum EEWPhase {
	Cancel,
	Forecast,
	Alert
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone, Copy)]
pub enum IntensityClass {
	Unknown, Zero, One, Two, Three, Four, FiveLower, FiveUpper, SixLower, SixUpper, Seven
}

impl IntensityClass {

	pub fn new(intensity: Option<f32>) -> IntensityClass
	{
		if let Some(i) = intensity {
			match i {
				x if x < 0.5 => IntensityClass::Zero,
				x if x < 1.5 => IntensityClass::One,
				x if x < 2.5 => IntensityClass::Two,
				x if x < 3.5 => IntensityClass::Three,
				x if x < 4.5 => IntensityClass::Four,
				x if x < 5.0 => IntensityClass::FiveLower,
				x if x < 5.5 => IntensityClass::FiveUpper,
				x if x < 6.0 => IntensityClass::SixLower,
				x if x < 6.5 => IntensityClass::SixUpper,
				_ => IntensityClass::Seven
			}
		} else {
			IntensityClass::Unknown
		}
	}
}

impl EEW {

	pub fn get_eew_phase(&self) -> Option<EEWPhase>
	{
		match self.kind {

			Kind::Cancel | Kind::DrillCancel => Some(EEWPhase::Cancel),

			Kind::Normal | Kind::Drill | Kind::Reference | Kind::Test => {

				match self.detail.as_ref().map(|d| d.warning_status) {
					Some(WarningStatus::Alert) => Some(EEWPhase::Alert),
					Some(WarningStatus::Forecast) => Some(EEWPhase::Forecast),
					_ => None,
				}
			}
		}
	}

	pub fn is_high_accuracy(&self) -> bool
	{
		self.issue_pattern == IssuePattern::HighAccuracy
	}

	pub fn get_maximum_intensity_class(&self) -> IntensityClass
	{
		self.detail.as_ref().map_or(IntensityClass::Unknown, |d| IntensityClass::new(d.maximum_intensity))
	}

	pub fn is_last(&self) -> bool
	{
		match self.status {
			Status::LastWithCorrection | Status::Last => true,
			_ => false
		}
	}

	pub fn is_drill(&self) -> bool
	{
		self.kind == Kind::Drill || self.kind == Kind::DrillCancel
	}
}
