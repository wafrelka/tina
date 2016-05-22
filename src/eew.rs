extern crate chrono;

use self::chrono::*;


pub enum IssuePattern { IntensityOnly, LowAccuracy, HighAccuracy, Cancel }
pub enum Source { Sapporo, Sendai, Tokyo, Osaka, Fukuoka, Okinawa }
pub enum Kind { Normal, Drill, Cancel, DrillCancel, Reference, Test }
pub enum Status { Normal, Correction, CancelCorrection, LastWithCorrection, Last, Unknown }

pub enum EpicenterAccuracy {
	Single, Territory, GridSearchLow, GridSearchHigh,
	NIEDLow, NIEDHigh, EPOSLow, EPOSHigh, Reserved, Unknown
}

pub enum DepthAccuracy {
	Single, Territory, GridSearchLow, GridSearchHigh,
	NIEDLow, NIEDHigh, EPOSLow, EPOSHigh, Reserved, Unknown
}

pub enum MagnitudeAccuracy {
	NIED, PWave, PSMixed, SWave, EPOS, Level,
	Undefined, Reserved, Unknown
}

pub enum EpicenterCategory { Land, Sea, Undefined, Unknown }
pub enum WarningStatus { Forecast, Alert, Undefined, Unknown }
pub enum IntensityChange { Same, Up, Down, Undefined, Unknown }
pub enum ChangeReason { Nothing, Magnitude, Epicenter, Mixed, Depth, Undefined, Unknown }
pub enum WaveStatus { Unreached, Reached, Undefined, Unknown }

pub struct AreaEEW {

	area_name: String,
	minimum_intensity: f32,
	maximum_intensity: Option<f32>,
	reached_at: DateTime<UTC>,
	warning_status: WarningStatus,
	wave_status: WaveStatus,
}

pub struct EEW {

	pattern: IssuePattern,
	source: Source,
	kind: Kind,
	issued_at: DateTime<UTC>,

	occurred_at: DateTime<UTC>,
	id: String,
	status: Status,
	number: Option<u32>,
	epicenter_name: String,

	epicenter: (f32, f32),
	depth: f32,
	magnitude: f32,
	maximum_intensity: f32,

	epicenter_accuracy: EpicenterAccuracy,
	depth_accuracy: DepthAccuracy,
	magnitude_accuracy: MagnitudeAccuracy,

	epicenter_caterogy: EpicenterCategory,
	warning_status: WarningStatus,
	intensity_change: IntensityChange,
	change_reason: ChangeReason,

	area_eew: Vec<AreaEEW>,
}

pub trait ToEEW {
	fn to_eew(&self, text: &str) -> EEW;
}
