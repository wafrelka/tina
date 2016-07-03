use eew::*;


#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum EEWPhase {
	Cancel,
	FastForecast,
	Forecast,
	FastAlert,
	Alert
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum IntensityClass {
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
		_ => {}
	};

	if eew.kind == Kind::Cancel {
		return Some(EEWPhase::Cancel);
	}

	if let EEWDetail::Full(ref detail) = eew.detail {

		let phase = match detail.warning_status {
			WarningStatus::Alert => match detail.issue_pattern {
				IssuePattern::HighAccuracy => EEWPhase::Alert,
				IssuePattern::LowAccuracy | IssuePattern::IntensityOnly => EEWPhase::FastAlert
			},
			WarningStatus::Forecast => match detail.issue_pattern {
				IssuePattern::HighAccuracy => EEWPhase::Forecast,
				IssuePattern::LowAccuracy | IssuePattern::IntensityOnly => EEWPhase::FastForecast
			},
			_ => return None
		};

		return Some(phase);
	}

	return None;
}

impl IntensityClass {
	pub fn new(intensity: f32) -> IntensityClass {
		match intensity {
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
	}
}

impl EEW {

	pub fn get_eew_phase(&self) -> Option<EEWPhase> {
		get_eew_phase(&self)
	}

	pub fn get_maximum_intensity_class(&self) -> Option<IntensityClass> {
		match self.detail {
			EEWDetail::Full(ref detail) => detail.maximum_intensity.map(|i| IntensityClass::new(i)),
			EEWDetail::Cancel => None
		}
	}
}
