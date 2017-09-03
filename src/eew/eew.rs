use chrono::*;


#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum IssuePattern { Cancel, IntensityOnly, LowAccuracy, HighAccuracy }

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Source { Tokyo, Osaka }

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Kind { Normal, Drill, Cancel, DrillCancel, Reference, Test }

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Status { Normal, Correction, CancelCorrection, LastWithCorrection, Last, Unknown }

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum EpicenterAccuracy {
	Single, Territory, GridSearchLow, GridSearchHigh,
	NIEDLow, NIEDHigh, EPOSLow, EPOSHigh, Unknown
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum DepthAccuracy {
	Single, Territory, GridSearchLow, GridSearchHigh,
	NIEDLow, NIEDHigh, EPOSLow, EPOSHigh, Unknown
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum MagnitudeAccuracy {
	NIED, PWave, PSMixed, SWave, EPOS, Level, Unknown
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum EpicenterCategory { Land, Sea, Unknown }

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum WarningStatus { Forecast, Alert, Unknown }

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum IntensityChange { Same, Up, Down, Unknown }

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum ChangeReason { Nothing, Magnitude, Epicenter, Mixed, Depth, Unknown }

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum WaveStatus { Unreached, Reached, Unknown }

#[derive(PartialEq, Debug, Clone)]
pub struct AreaEEW {

	pub area_name: String,
	pub minimum_intensity: f32,
	pub maximum_intensity: Option<f32>,
	pub reach_at: Option<DateTime<UTC>>,
	pub warning_status: WarningStatus,
	pub wave_status: WaveStatus,
}

#[derive(PartialEq, Debug, Clone)]
pub struct EEW {

	pub issue_pattern: IssuePattern,
	pub source: Source,
	pub kind: Kind,
	pub issued_at: DateTime<UTC>,

	pub occurred_at: DateTime<UTC>,
	pub id: String,
	pub status: Status,
	pub number: u32, // we don't accept an EEW which has no telegram number

	pub detail: Option<EEWDetail>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct EEWDetail {

	pub epicenter_name: String,
	pub epicenter: (f32, f32),
	pub depth: Option<f32>,
	pub magnitude: Option<f32>,
	pub maximum_intensity: Option<f32>,

	pub epicenter_accuracy: EpicenterAccuracy,
	pub depth_accuracy: DepthAccuracy,
	pub magnitude_accuracy: MagnitudeAccuracy,

	pub epicenter_category: EpicenterCategory,
	pub warning_status: WarningStatus,
	pub intensity_change: IntensityChange,
	pub change_reason: ChangeReason,

	pub area_info: Vec<AreaEEW>,
}
