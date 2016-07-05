use eew::*;


#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum EEWPhase {
	Cancel,
	Forecast,
	Alert
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
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


pub fn get_eew_phase(eew: &EEW) -> Option<EEWPhase>
{
	match eew.kind {
		Kind::Drill | Kind::DrillCancel => return None,
		Kind::Reference | Kind::Test => return None,
		Kind::Cancel => return Some(EEWPhase::Cancel),
		Kind::Normal => {}
	};

	if let EEWDetail::Full(ref detail) = eew.detail {

		let phase = match detail.warning_status {
			WarningStatus::Alert => EEWPhase::Alert,
			WarningStatus::Forecast => EEWPhase::Forecast,
			_ => return None
		};

		return Some(phase);
	}

	return None;
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

	pub fn get_eew_phase(&self) -> Option<EEWPhase> {
		get_eew_phase(&self)
	}

	pub fn is_high_accuracy(&self) -> bool {
		match self.detail {
			EEWDetail::Full(ref detail) => match detail.issue_pattern {
				IssuePattern::HighAccuracy => true,
				_ => false
			},
			EEWDetail::Cancel => false
		}
	}

	pub fn get_maximum_intensity_class(&self) -> IntensityClass {
		match self.detail {
			EEWDetail::Full(ref detail) => IntensityClass::new(detail.maximum_intensity),
			EEWDetail::Cancel => IntensityClass::Unknown
		}
	}
}
