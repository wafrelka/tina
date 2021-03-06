use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::error::Error as StdError;

use csv::ReaderBuilder;
use serde::{Deserializer, Deserialize};
use serde::de::Error as SerdeError;
use serde_yaml;
use serde_yaml::Value;
use slog::Level;

use tina::{ValueCondition, DisjunctiveCondition, IntensityClass};


#[derive(Debug, Clone)]
pub enum ConfigLoadError {
	ConfigFileIo,
	CodeDictFileIO,
	InvalidCodeFormat,
	InvalidYamlFormat,
	InvalidKeyValue(String),
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct RawRootConfig {
	pub path: DictPathConfig,
	pub wni: WniConfig,
	pub twitter: Option<TwitterConfig>,
	pub slack: Option<SlackConfig>,
	pub log: LogConfig,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct DictPathConfig {
	pub area: String,
	pub epicenter: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct LogConfig {
	pub wni_log_path: Option<String>,
	pub eew_log_path: Option<String>,
	#[serde(default)] pub wni_stdout_log: bool,
	#[serde(default)] pub eew_stdout_log: bool,
	#[serde(deserialize_with = "deserialize_log_level")] pub log_level: Level,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct WniConfig {
	pub id: String,
	pub password: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct TwitterConfig {
	pub consumer_token: String,
	pub consumer_secret: String,
	pub access_token: String,
	pub access_secret: String,
	#[serde(default)] pub in_reply_to_enabled: bool,
	#[serde(default)] pub updown_enabled: bool,
	pub cond: Option<Vec<ValueConditionConfig>>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct SlackConfig {
	pub webhook_url: String,
	#[serde(default)] pub updown_enabled: bool,
	pub cond: Option<Vec<ValueConditionConfig>>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ValueConditionConfig {

	pub first: Option<bool>,
	pub succeeding: Option<bool>,

	pub alert: Option<bool>,
	pub last: Option<bool>,
	pub cancel: Option<bool>,
	#[serde(default="def_opt_false")] pub drill: Option<bool>,
	#[serde(default="def_opt_false")] pub test: Option<bool>,

	pub phase_changed: Option<bool>,
	pub epicenter_name_changed: Option<bool>,

	pub magnitude_over: Option<f32>,
	pub intensity_over: Option<f32>,

	pub intensity_up: Option<u8>,
	pub intensity_down: Option<u8>,
}

#[derive(Debug)]
pub struct Config {
	pub area_dict: HashMap<[u8; 3], String>,
	pub epicenter_dict: HashMap<[u8; 3], String>,
	pub wni: WniConfig,
	pub twitter: Option<TwitterConfig>,
	pub slack: Option<SlackConfig>,
	pub log: LogConfig,
}

impl From<ValueConditionConfig> for ValueCondition {

	fn from(conf: ValueConditionConfig) -> ValueCondition {

		ValueCondition {
			first: conf.first, succeeding: conf.succeeding, alert: conf.alert, last: conf.last,
			cancel: conf.cancel, drill: conf.drill, test: conf.test,
			phase_changed: conf.phase_changed, epicenter_name_changed: conf.epicenter_name_changed,
			magnitude_over: conf.magnitude_over,
			intensity_over: conf.intensity_over.map(|i| IntensityClass::new(i)),
			intensity_up: conf.intensity_up, intensity_down: conf.intensity_down,
		}
	}
}

pub fn build_yaml_condition(v: Vec<ValueConditionConfig>) -> DisjunctiveCondition<ValueCondition>
{
	v.into_iter().map(|vc| vc.into()).collect::<Vec<ValueCondition>>().into()
}

fn deserialize_log_level<'d, D>(deserializer: D) -> Result<Level, D::Error>
	where D: Deserializer<'d>
{
	if let Ok(s) = String::deserialize(deserializer) {
		match s.as_str() {
			"critical" => Ok(Level::Critical),
			"error" => Ok(Level::Error),
			"warning" => Ok(Level::Warning),
			"info" => Ok(Level::Info),
			"debug" => Ok(Level::Debug),
			_ => Err(D::Error::custom("unknown log level"))
		}
	} else {
		Ok(Level::Info)
	}
}

fn def_opt_false() -> Option<bool> { Some(false) }


fn load_code_dict(path: &str) -> Result<HashMap<[u8; 3], String>, ConfigLoadError>
{
	let mut reader = try!(ReaderBuilder::new().has_headers(false).from_path(path).map_err(|_| ConfigLoadError::CodeDictFileIO));

	let mut dict = HashMap::new();

	for record in reader.deserialize() {

		let (code, name): (String, String) = try!(record.map_err(|_| ConfigLoadError::InvalidCodeFormat));

		let mut encoded = [0; 3];
		let bytes = code.as_bytes();

		if bytes.len() != encoded.len() {
			return Err(ConfigLoadError::InvalidCodeFormat);
		}

		encoded.copy_from_slice(bytes);
		dict.insert(encoded, name);
	}

	return Ok(dict);
}

impl Config {

	pub fn load_config(path: &str) -> Result<Config, ConfigLoadError>
	{
		let mut file = try!(File::open(path).map_err(|_| ConfigLoadError::ConfigFileIo));
		let mut data = String::new();
		try!(file.read_to_string(&mut data).map_err(|_| ConfigLoadError::ConfigFileIo));

		let raw_value: Value =
			try!(serde_yaml::from_str(&data).map_err(|_| ConfigLoadError::InvalidYamlFormat));
		let raw_root_conf: RawRootConfig =
			serde_yaml::from_value(raw_value.clone())
			.map_err(|err| ConfigLoadError::InvalidKeyValue(err.description().to_owned()))?;

		let area_dict = try!(load_code_dict(&raw_root_conf.path.area));
		let epicenter_dict = try!(load_code_dict(&raw_root_conf.path.epicenter));

		let conf = Config {
			area_dict: area_dict,
			epicenter_dict: epicenter_dict,
			wni: raw_root_conf.wni,
			twitter: raw_root_conf.twitter,
			slack: raw_root_conf.slack,
			log: raw_root_conf.log,
		};

		Ok(conf)
	}
}
