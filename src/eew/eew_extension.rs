use eew::*;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone, Copy)]
pub enum EEWPhase {
	Cancel,
	FastForecast,
	Forecast,
	Alert,
}

impl EEW {

	pub fn get_eew_phase(&self) -> Option<EEWPhase>
	{
		match self.kind {

			Kind::Cancel | Kind::DrillCancel => Some(EEWPhase::Cancel),

			Kind::Normal | Kind::Drill | Kind::Reference | Kind::Trial => {

				match self.detail.as_ref().map(|d| d.warning_status) {
					Some(WarningStatus::Alert) => Some(EEWPhase::Alert),
					Some(WarningStatus::Forecast) => {
						match self.issue_pattern {
							IssuePattern::IntensityOnly | IssuePattern::LowAccuracy
								=> Some(EEWPhase::FastForecast),
							IssuePattern::HighAccuracy
								=> Some(EEWPhase::Forecast),
							_ => None,
						}
					}
					_ => None,
				}
			}
		}
	}

	pub fn is_high_accuracy(&self) -> bool
	{
		self.issue_pattern == IssuePattern::HighAccuracy
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

	pub fn is_test(&self) -> bool
	{
		self.kind == Kind::Reference || self.kind == Kind::Trial
	}

	pub fn is_succeeded_by(&self, eew: &EEW) -> bool
	{
		if self.id != eew.id {
			false
		} else if self.number != eew.number {
			self.number < eew.number
		} else {
			match (self.get_eew_phase(), eew.get_eew_phase()) {
				(Some(EEWPhase::Cancel), _) => false,
				(_, Some(EEWPhase::Cancel)) => true,
				_ => false,
			}
		}
	}
}
