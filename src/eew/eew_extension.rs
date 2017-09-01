use eew::*;


#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone)]
pub enum EEWPhase {
	Cancel,
	Forecast,
	Alert
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone)]
pub enum IntensityClass {
	Unknown,
	Zero,
	One,
	Two,
	Three,
	Four,
	FiveMinus,
	FivePlus,
	SixMinus,
	SixPlus,
	Seven
}

impl IntensityClass {
	pub fn new(intensity: Option<f32>) -> IntensityClass {
		if let Some(i) = intensity {
			match i {
				x if x < 0.5 => IntensityClass::Zero,
				x if x < 1.5 => IntensityClass::One,
				x if x < 2.5 => IntensityClass::Two,
				x if x < 3.5 => IntensityClass::Three,
				x if x < 4.5 => IntensityClass::Four,
				x if x < 5.0 => IntensityClass::FiveMinus,
				x if x < 5.5 => IntensityClass::FivePlus,
				x if x < 6.0 => IntensityClass::SixMinus,
				x if x < 6.5 => IntensityClass::SixPlus,
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

				if let EEWDetail::Full(ref detail) = self.detail {
					match detail.warning_status {
						WarningStatus::Alert => Some(EEWPhase::Alert),
						WarningStatus::Forecast => Some(EEWPhase::Forecast),
						_ => None
					}
				} else {
					None
				}
			}
		}
	}

	pub fn is_high_accuracy(&self) -> bool
	{
		match self.detail {
			EEWDetail::Full(ref detail) => match detail.issue_pattern {
				IssuePattern::HighAccuracy => true,
				_ => false
			},
			EEWDetail::Cancel => false
		}
	}

	pub fn get_maximum_intensity_class(&self) -> IntensityClass
	{
		match self.detail {
			EEWDetail::Full(ref detail) => IntensityClass::new(detail.maximum_intensity),
			EEWDetail::Cancel => IntensityClass::Unknown
		}
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
