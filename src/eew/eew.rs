use chrono::*;


#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum IssuePattern { Cancel, IntensityOnly, LowAccuracy, HighAccuracy }

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Source { Tokyo, Osaka }

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Kind { Normal, Drill, Cancel, DrillCancel, Reference, Trial }

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

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone, Copy)]
pub enum IntensityClass {
	Zero, One, Two, Three, Four, FiveLower, FiveUpper, SixLower, SixUpper, Seven
}

impl IntensityClass {

	pub fn new(intensity: f32) -> IntensityClass
	{
		match intensity {
			x if x < 0.5 => IntensityClass::Zero,
			x if x < 1.5 => IntensityClass::One,
			x if x < 2.5 => IntensityClass::Two,
			x if x < 3.5 => IntensityClass::Three,
			x if x < 4.5 => IntensityClass::Four,
			x if x < 5.0 => IntensityClass::FiveLower,
			x if x < 5.5 => IntensityClass::FiveUpper,
			x if x < 6.0 => IntensityClass::SixLower,
			x if x < 6.5 => IntensityClass::SixUpper,
			_ => IntensityClass::Seven,
		}
	}

	pub fn ord(&self) -> i32
	{
		match *self {
			IntensityClass::Zero => 0,
			IntensityClass::One => 1,
			IntensityClass::Two => 2,
			IntensityClass::Three => 3,
			IntensityClass::Four => 4,
			IntensityClass::FiveLower => 5,
			IntensityClass::FiveUpper => 6,
			IntensityClass::SixLower => 7,
			IntensityClass::SixUpper => 8,
			IntensityClass::Seven => 9,
		}
	}
}

#[derive(PartialEq, Debug, Clone)]
pub struct AreaEEW {

	pub area_name: String,
	pub minimum_intensity: IntensityClass,
	pub maximum_intensity: Option<IntensityClass>,
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
	pub maximum_intensity: Option<IntensityClass>,

	pub epicenter_accuracy: EpicenterAccuracy,
	pub depth_accuracy: DepthAccuracy,
	pub magnitude_accuracy: MagnitudeAccuracy,

	pub epicenter_category: EpicenterCategory,
	pub warning_status: WarningStatus,
	pub intensity_change: IntensityChange,
	pub change_reason: ChangeReason,

	pub area_info: Vec<AreaEEW>,
}
