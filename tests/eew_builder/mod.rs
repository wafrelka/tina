use chrono::*;
use tina::*;

#[derive(PartialEq, Debug, Clone)]
pub struct EEWBuilder {
	issue_pattern: Option<IssuePattern>,
	kind: Option<Kind>,
	issued_at: Option<DateTime<UTC>>,
	occurred_at: Option<DateTime<UTC>>,
	id: Option<String>,
	status: Option<Status>,
	number: Option<u32>,
	detail_none: bool,
	epicenter: Option<(f32, f32)>,
	depth: Option<Option<f32>>,
	magnitude: Option<Option<f32>>,
	maximum_intensity: Option<Option<IntensityClass>>,
	warning_status: Option<WarningStatus>,
}

impl EEWBuilder {

	pub fn new() -> Self
	{
		EEWBuilder {
			issue_pattern: None, kind: None, issued_at: None, occurred_at: None,
			id: None, status: None, number: None, detail_none: false,
			epicenter: None, depth: None, magnitude: None,
			maximum_intensity: None, warning_status: None,
		}
	}

	#[allow(dead_code)]
	pub fn issue_pattern(self, issue_pattern: IssuePattern) -> Self
	{
		Self { issue_pattern: Some(issue_pattern), .. self }
	}

	#[allow(dead_code)]
	pub fn kind(self, kind: Kind) -> Self
	{
		Self { kind: Some(kind), .. self }
	}

	#[allow(dead_code)]
	pub fn issued_at(self, issued_at: DateTime<UTC>) -> Self
	{
		Self { issued_at: Some(issued_at), .. self }
	}

	#[allow(dead_code)]
	pub fn occurred_at(self, occurred_at: DateTime<UTC>) -> Self
	{
		Self { occurred_at: Some(occurred_at), .. self }
	}

	#[allow(dead_code)]
	pub fn id(self, id: String) -> Self
	{
		Self { id: Some(id), .. self }
	}

	#[allow(dead_code)]
	pub fn status(self, status: Status) -> Self
	{
		Self { status: Some(status), .. self }
	}

	#[allow(dead_code)]
	pub fn number(self, number: u32) -> Self
	{
		Self { number: Some(number), .. self }
	}

	#[allow(dead_code)]
	pub fn detail_none(self) -> Self
	{
		Self { detail_none: true, .. self }
	}

	#[allow(dead_code)]
	pub fn epicenter(self, epicenter: (f32, f32)) -> Self
	{
		Self { epicenter: Some(epicenter), .. self }
	}

	#[allow(dead_code)]
	pub fn depth(self, depth: Option<f32>) -> Self
	{
		Self { depth: Some(depth), .. self }
	}

	#[allow(dead_code)]
	pub fn magnitude(self, magnitude: Option<f32>) -> Self
	{
		Self { magnitude: Some(magnitude), .. self }
	}

	#[allow(dead_code)]
	pub fn maximum_intensity(self, maximum_intensity: Option<IntensityClass>) -> Self
	{
		Self { maximum_intensity: Some(maximum_intensity), .. self }
	}

	#[allow(dead_code)]
	pub fn warning_status(self, warning_status: WarningStatus) -> Self
	{
		Self { warning_status: Some(warning_status), .. self }
	}

	pub fn build(self) -> EEW
	{
		let detail = EEWDetail {
			epicenter_name: "奈良県".to_owned(),
			epicenter: self.epicenter.unwrap_or((34.4, 135.7)),
			depth: self.depth.unwrap_or(Some(10.0)),
			magnitude: self.magnitude.unwrap_or(Some(5.9)),
			maximum_intensity: self.maximum_intensity.unwrap_or(Some(IntensityClass::FiveLower)),
			epicenter_accuracy: EpicenterAccuracy::GridSearchLow,
			depth_accuracy: DepthAccuracy::GridSearchLow,
			magnitude_accuracy: MagnitudeAccuracy::SWave,
			epicenter_category: EpicenterCategory::Land,
			warning_status: self.warning_status.unwrap_or(WarningStatus::Forecast),
			intensity_change: IntensityChange::Unknown,
			change_reason: ChangeReason::Unknown,
			area_info: vec!{},
		};

		EEW {
			issue_pattern: self.issue_pattern.unwrap_or(IssuePattern::HighAccuracy),
			source: Source::Tokyo,
			kind: self.kind.unwrap_or(Kind::Normal),
			issued_at: self.issued_at.unwrap_or(UTC.ymd(2010, 1, 1).and_hms(1, 0, 2)),
			occurred_at: self.occurred_at.unwrap_or(UTC.ymd(2010, 1, 1).and_hms(0, 55, 59)),
			id: self.id.unwrap_or("ND20100101005559".to_owned()),
			status: self.status.unwrap_or(Status::Normal),
			number: self.number.unwrap_or(10),
			detail: if self.detail_none { None } else { Some(detail) },
		}
	}
}
